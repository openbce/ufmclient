use ufmclient::{UFMError, UFMConfig};

pub async fn run(conf: UFMConfig, pkey: &String) -> Result<(), UFMError> {
    let ufm = ufmclient::connect(conf)?;
    ufm.delete_partition(pkey).await?;

    Ok(())
}
