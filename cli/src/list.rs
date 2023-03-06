use ufmclient::{UFMError, UFM};

pub async fn list() -> Result<(), UFMError> {
    let mut ufm = UFM::new()?;
    let ps = ufm.list_partition().await?;

    println!(
        "{}{}{}{}{}{}{}",
        "Name", "Pkey", "IPoIB", "MTU", "RateLimit", "ServiceLevel", "GUIDS#"
    );

    for p in ps {
        println!(
            "{}{}{}{}{}{}{}",
            p.name,
            p.pkey,
            p.ipoib,
            p.qos.mtu_limit,
            p.qos.rate_limit,
            p.qos.service_level,
            p.guids.len()
        )
    }

    Ok(())
}
