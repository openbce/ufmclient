use ufmclient::{UFMError, UFM};

pub async fn run(pkey: &String) -> Result<(), UFMError> {
    let mut ufm = UFM::new()?;
    ufm.delete_partition(pkey).await?;

    Ok(())
}
