use ufmclient::{UFMError, UFMConfig};

pub async fn run(conf: UFMConfig) -> Result<(), UFMError> {
    let ufm = ufmclient::connect(conf)?;
    let v = ufm.version().await?;

    println!("{}", v);

    Ok(())
}
