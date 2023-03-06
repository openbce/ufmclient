use ufmclient::{UFMError, UFM};

pub async fn run() -> Result<(), UFMError> {
    let mut ufm = UFM::new()?;
    let v = ufm.version().await?;

    println!("{}", v);

    Ok(())
}
