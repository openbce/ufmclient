use ufmclient::{UFMError, UFM};

#[tokio::main]
async fn main() -> Result<(), UFMError> {
    let mut ufm = UFM::new()?;
    let p = ufm.get_partition(&"0x7fff".to_string()).await?;

    println!("{} {}\n", p.name, p.pkey);

    Ok(())
}
