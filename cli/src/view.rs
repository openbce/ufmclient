use ufmclient::{UFMError, UFM};

pub async fn run(pkey: &String) -> Result<(), UFMError> {
    let mut ufm = UFM::new()?;
    let p = ufm.get_partition(pkey).await?;

    println!("{:15}: {}", "Name", p.name);
    println!("{:15}: 0x{:x}", "Pkey", p.pkey);
    println!("{:15}: {}", "IPoIB", p.ipoib);
    println!("{:15}: {}", "MTU", p.qos.mtu_limit);
    println!("{:15}: {}", "Rate Limit", p.qos.rate_limit);
    println!("{:15}: {}", "Service Level", p.qos.service_level);

    Ok(())
}
