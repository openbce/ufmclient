use ufmclient;
use ufmclient::UFMError;

#[tokio::main]
async fn main() -> Result<(), UFMError> {
    let mut ufm = ufmclient::UFM::new()?;
    let p = ufm.get_partition(&"0x7fff".to_string()).await?;

    println!("{} {}\n", p.name, p.pkey);

    Ok(())
}
