use ufmclient::util::is_default_pkey;
use ufmclient::{Filter, UFMError, UFM};

pub async fn run(pkey: &String) -> Result<(), UFMError> {
    let mut ufm = UFM::new()?;
    let p = ufm.get_partition(pkey).await?;

    let ps = {
        if !is_default_pkey(p.pkey) {
            let f = Filter {
                guids: {
                    let mut v = Vec::new();
                    for i in p.guids {
                        v.push(i.guid.to_string());
                    }
                    Some(v)
                },
            };
            ufm.list_port(Some(f)).await?
        } else {
            ufm.list_port(None).await?
        }
    };

    println!("{:15}: {}", "Name", p.name);
    println!("{:15}: 0x{:x}", "Pkey", p.pkey);
    println!("{:15}: {}", "IPoIB", p.ipoib);
    println!("{:15}: {}", "MTU", p.qos.mtu_limit);
    println!("{:15}: {}", "Rate Limit", p.qos.rate_limit);
    println!("{:15}: {}", "Service Level", p.qos.service_level);
    println!("{:15}: ", "Ports");

    println!(
        "    {:<25}{:<20}{:<20}{:<15}{:<15}{:<10}{:<15}{:<10}",
        "Name", "GUID", "SystemID", "SystemName", "DName", "LID", "LogState", "PhyState"
    );
    for port in ps {
        println!(
            "    {:<25}{:<20}{:<20}{:<15}{:<15}{:<10}{:<15}{:<10}",
            port.name,
            port.guid,
            port.system_id,
            port.system_name,
            port.dname,
            port.lid,
            port.logical_state,
            port.physical_state,
        );
    }

    Ok(())
}
