use ufmclient::util::build_pkey;
use ufmclient::{UFMError, UFM};

pub async fn run() -> Result<(), UFMError> {
    let mut ufm = UFM::new()?;
    let ps = ufm.list_partition().await?;

    println!(
        "{:<15}{:<10}{:<10}{:<10}{:<10}{:<10}{:<10}",
        "Name", "Pkey", "IPoIB", "MTU", "Rate", "Level", "GUIDS#"
    );

    for p in ps {
        println!(
            "{:<15}{:<10}{:<10}{:<10}{:<10}{:<10}{:<10}",
            p.name,
            build_pkey(p.pkey),
            p.ipoib,
            p.qos.mtu_limit,
            p.qos.rate_limit,
            p.qos.service_level,
            p.guids.len()
        )
    }

    Ok(())
}
