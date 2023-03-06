use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::RestError;
use crate::util::{build_pkey, parse_pkey};

pub mod util;

mod rest;
mod types;

#[derive(Serialize, Deserialize, Debug)]
pub struct PartitionQoS {
    // Default 2k; one of 2k or 4k; the MTU of the services.
    pub mtu_limit: i32,
    // Default is None, value can be range from 0-15
    pub service_level: i32,
    // Default is None, can be one of the following: 2.5, 10, 30, 5, 20, 40, 60, 80, 120, 14, 56, 112, 168, 25, 100, 200, or 300
    pub rate_limit: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortBinding {
    // The GUID of Port.
    pub guid: String,
    // Default false; store the PKey at index 0 of the PKey table of the GUID.
    pub index0: bool,
    // Default is full:
    //   "full" - members with full membership can communicate with all hosts (members) within the network/partition
    //   "limited" - members with limited membership cannot communicate with other members with limited membership. However, communication is allowed between every other combination of membership types.
    pub membership: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Partition {
    // The name of Partition.
    pub name: String,
    // The pkeys of Partition.
    pub pkey: i32,
    // Default false
    pub ipoib: bool,
    // The QoS of Partition.
    pub qos: PartitionQoS,
    // The Ports belong to the partition
    pub guids: Vec<PortBinding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Port {
    pub guid: String,
    pub name: String,
    pub system_id: String,
    pub lid: i32,
    pub dname: String,
    pub system_name: String,
    pub physical_state: String,
    pub logical_state: String,
}

impl From<HashMap<String, Value>> for Port {
    fn from(value: HashMap<String, Value>) -> Self {
        let get_str = |v: Option<&Value>| -> String { v.unwrap().as_str().unwrap().to_string() };
        let get_i64 = |v: Option<&Value>| -> i64 { v.unwrap().as_i64().unwrap() };

        Self {
            guid: get_str(value.get("guid")),
            name: get_str(value.get("name")),
            system_id: get_str(value.get("systemID")),
            lid: get_i64(value.get("lid")) as i32,
            dname: get_str(value.get("dname")),
            system_name: get_str(value.get("system_name")),
            physical_state: get_str(value.get("physical_state")),
            logical_state: get_str(value.get("logical_state")),
        }
    }
}

pub struct Filter {
    pub guids: Option<Vec<String>>,
}

impl Filter {
    fn valid(&self, p: &Port) -> bool {
        // Check GUID filter
        if let Some(guids) = &self.guids {
            let mut found = false;
            for id in guids {
                if p.guid == *id {
                    found = true;
                    break;
                }
            }

            if !found {
                return false;
            }
        }

        // All filters are passed, return true.
        true
    }
}

impl From<Vec<PortBinding>> for Filter {
    fn from(guids: Vec<PortBinding>) -> Self {
        let mut v = Vec::new();
        for i in &guids {
            v.push(i.guid.to_string());
        }

        Self{
            guids: Some(v)
        }
    }
}

pub struct UFM {
    client: rest::RestClient,
}

pub enum UFMError {
    Unknown { msg: String },
    NotFound { msg: String },
    InvalidPKey { msg: String },
    InvalidConfig { msg: String },
}

impl From<RestError> for UFMError {
    fn from(e: RestError) -> Self {
        match &e {
            RestError::Unknown { msg } => UFMError::Unknown {
                msg: msg.to_string(),
            },
            RestError::NotFound { msg } => UFMError::NotFound {
                msg: msg.to_string(),
            },
            RestError::AuthFailure { msg } => UFMError::InvalidConfig {
                msg: msg.to_string(),
            },
            RestError::InvalidConfig { msg } => UFMError::InvalidConfig {
                msg: msg.to_string(),
            },
        }
    }
}

impl fmt::Debug for UFMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            UFMError::Unknown { msg } => {
                write!(f, "Unknown: {}", msg)
            }
            UFMError::NotFound { msg } => {
                write!(f, "Not found: {}", msg)
            }
            UFMError::InvalidPKey { msg } => {
                write!(f, "Invalid pkey: {}", msg)
            }
            UFMError::InvalidConfig { msg } => {
                write!(f, "Invalid configuration: {}", msg)
            }
        }
    }
}

impl UFM {
    pub fn new() -> Result<UFM, UFMError> {
        let c = rest::RestClient::new()?;

        Ok(Self { client: c })
    }

    pub async fn create_partition(&mut self, p: &Partition) -> Result<(), UFMError> {
        let path = String::from("/ufmRest/resources/pkeys");

        #[derive(Serialize, Deserialize, Debug)]
        struct Pkey {
            pkey: String,
            ip_over_ib: bool,
            membership: String,
            index0: bool,
            guids: Vec<String>,
        }

        let mut guids = vec![];
        for pb in &p.guids {
            guids.push(pb.guid.to_string());
        }

        let pkey = Pkey {
            pkey: build_pkey(p.pkey),
            ip_over_ib: p.ipoib,
            membership: String::from("full"),
            index0: true,
            guids,
        };

        let data = serde_json::to_string(&pkey).unwrap();

        self.client.post(&path, data).await?;

        Ok(())
    }

    pub async fn get_partition(&mut self, pkey: &String) -> Result<Partition, UFMError> {
        let path = format!(
            "/ufmRest/resources/pkeys/{}?guids_data=true&qos_conf=true",
            pkey
        );
        let ps = self.client.get(&path).await?;

        if ps == "{}" {
            return Err(UFMError::NotFound {
                msg: format!("{} not found", pkey),
            });
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct Pkey {
            partition: String,
            ip_over_ib: bool,
            qos_conf: PartitionQoS,
            guids: Vec<PortBinding>,
        }
        let pk: Pkey = serde_json::from_str(&ps[..]).unwrap();

        Ok(Partition {
            name: pk.partition,
            pkey: parse_pkey(pkey)?,
            ipoib: pk.ip_over_ib,
            qos: pk.qos_conf,
            guids: pk.guids,
        })
    }

    pub async fn list_partition(&mut self) -> Result<Vec<Partition>, UFMError> {
        #[derive(Serialize, Deserialize, Debug)]
        struct Pkey {
            partition: String,
            ip_over_ib: bool,
            qos_conf: Option<PartitionQoS>,
            guids: Option<Vec<PortBinding>>,
        }

        let pkey_qos: HashMap<String, Pkey> = {
            let path = String::from("/ufmRest/resources/pkeys?qos_conf=true");
            let ps = self.client.get(&path).await?;
            log::debug!("listQoS: {}", ps);

            serde_json::from_str(&ps[..]).unwrap()
        };

        let mut pkey_guids: HashMap<String, Pkey> = {
            let path = String::from("/ufmRest/resources/pkeys?guids_data=true");
            let ps = self.client.get(&path).await?;
            log::debug!("listGUIDs: {}", ps);

            serde_json::from_str(&ps[..]).unwrap()
        };

        let mut parts = Vec::new();

        for (k, v) in pkey_qos {
            parts.push(Partition {
                name: v.partition,
                pkey: parse_pkey(&k)?,
                ipoib: v.ip_over_ib,
                qos: v.qos_conf.unwrap(),
                guids: {
                    let g = pkey_guids.remove(&k);
                    match g {
                        Some(pk) => pk.guids.unwrap_or(Vec::new()),
                        None => Vec::new(),
                    }
                },
            });
        }

        Ok(parts)
    }

    pub async fn delete_partition(&mut self, pkey: &String) -> Result<(), UFMError> {
        let path = format!("/ufmRest/resources/pkeys/{}", pkey);
        self.client.delete(&path).await?;

        Ok(())
    }

    pub async fn list_port(&mut self, filter: Option<Filter>) -> Result<Vec<Port>, UFMError> {
        let path = String::from("/ufmRest/resources/ports?sys_type=Computer");
        let resp = self.client.get(&path).await?;

        log::debug!("list ports: {}", resp);

        let ports: Vec<HashMap<String, Value>> = serde_json::from_str(&resp[..]).unwrap();
        let f = match filter {
            None => Filter { guids: None },
            Some(f) => f,
        };

        let mut res = Vec::new();
        for p in ports {
            let port = Port::from(p);
            if f.valid(&port) {
                res.push(port);
            }
        }

        Ok(res)
    }

    pub async fn version(&mut self) -> Result<String, UFMError> {
        #[derive(Serialize, Deserialize, Debug)]
        struct Version {
            ufm_release_version: String,
        }

        let path = String::from("/ufmRest/app/ufm_version");
        let resp = self.client.get(&path).await?;
        let v: Version = serde_json::from_str(&resp[..]).unwrap();

        Ok(v.ufm_release_version)
    }
}
