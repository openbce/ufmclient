use ufmclient::{UFMConfig, UFMError};

pub async fn run(conf: UFMConfig) -> Result<(), UFMError> {
    let ufm = ufmclient::connect(conf)?;
    let ps = ufm.list_partition().await?;

    println!(
        "{:<15}{:<10}{:<10}{:<10}{:<10}{:<10}",
        "Name", "Pkey", "IPoIB", "MTU", "Rate", "Level"
    );

    for p in ps {
        println!(
            "{:<15}{:<10}{:<10}{:<10}{:<10}{:<10}",
            p.name,
            p.pkey.to_string(),
            p.ipoib,
            p.qos.mtu_limit,
            p.qos.rate_limit,
            p.qos.service_level
        )
    }

    Ok(())
}
