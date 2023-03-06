use std::collections::HashMap;
use std::fmt;

use crate::types::RestError;
use serde::{Deserialize, Serialize};

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

pub struct UFM {
    client: rest::RestClient,
}

pub enum UFMError {
    Unknown { msg: String },
    NotFound { msg: String },
    InvalidPKey { msg: String },
    InvalidConfig { msg: String },
}

impl From<types::RestError> for UFMError {
    fn from(e: types::RestError) -> Self {
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
        let path = String::from("/ufmRest/resources/pkeys?qos_conf=true");

        let ps = self.client.get(&path).await?;

        log::debug!("listQoS: {}", ps);

        #[derive(Serialize, Deserialize, Debug)]
        struct Pkey {
            partition: String,
            ip_over_ib: bool,
            qos_conf: PartitionQoS,
        }
        let pks: HashMap<String, Pkey> = serde_json::from_str(&ps[..]).unwrap();

        let mut parts = Vec::new();

        for (k, v) in pks {
            parts.push(Partition {
                name: v.partition,
                pkey: parse_pkey(&k)?,
                ipoib: v.ip_over_ib,
                qos: v.qos_conf,
                // TODO(k82cn): get GUIDs by rest
                guids: Vec::new(),
            });
        }

        Ok(parts)
    }

    pub fn delete_partition(&mut self, _pkey: &String) -> Result<Vec<Port>, UFMError> {
        Err(UFMError::Unknown {
            msg: "unknown".to_string(),
        })
    }

    pub async fn get_port(&mut self, _guid: &String) -> Result<Vec<Port>, UFMError> {
        Err(UFMError::Unknown {
            msg: "unknown".to_string(),
        })
    }

    pub async fn list_port(&mut self) -> Result<Vec<Port>, UFMError> {
        Err(UFMError::Unknown {
            msg: "unknown".to_string(),
        })
    }
}
