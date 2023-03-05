use ufmclient::util::parse_pkey;
use ufmclient::{Partition, PartitionQoS, PortBinding, UFMError, UFM};

pub struct CreateOptions {
    pub pkey: String,
    pub mtu: i32,
    pub ipoib: bool,
    pub index0: bool,
    pub membership: String,
    pub service_level: i32,
    pub rate_limit: f64,
    pub guids: Vec<String>,
}

pub async fn run(opt: CreateOptions) -> Result<(), UFMError> {
    let mut ufm = UFM::new()?;

    let mut pbs = vec![];
    for g in opt.guids {
        pbs.push(PortBinding {
            guid: g.to_string(),
            index0: opt.index0,
            membership: opt.membership.to_string(),
        })
    }

    let p = Partition {
        name: "".to_string(),
        pkey: parse_pkey(&opt.pkey)?,
        ipoib: opt.ipoib,
        qos: PartitionQoS {
            mtu_limit: opt.mtu,
            service_level: opt.service_level,
            rate_limit: opt.rate_limit,
        },
        guids: pbs,
    };

    ufm.create_partition(&p).await?;

    Ok(())
}
