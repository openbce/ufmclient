use ufmclient::util::parse_pkey;
use ufmclient::{Partition, PartitionQoS, PortBinding, UFMError, UFM};

pub async fn run(
    pkey: &String,
    mtu: &i32,
    ipoib: &bool,
    index0: &bool,
    membership: &String,
    service_level: &i32,
    rate_limit: &f64,
    guids: &Vec<String>,
) -> Result<(), UFMError> {
    let mut ufm = UFM::new()?;

    let mut pbs = vec![];
    for g in guids {
        pbs.push(PortBinding {
            guid: g.to_string(),
            index0: *index0,
            membership: membership.to_string(),
        })
    }

    let p = Partition {
        name: "".to_string(),
        pkey: parse_pkey(pkey)?,
        ipoib: *ipoib,
        qos: PartitionQoS {
            mtu_limit: *mtu,
            service_level: *service_level,
            rate_limit: *rate_limit,
        },
        guids: pbs,
    };

    ufm.create_partition(p).await?;

    Ok(())
}
