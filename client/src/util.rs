use crate::UFMError;

pub fn build_pkey(pkey: i32) -> String {
    format!("0x{:x}", pkey)
}

pub fn parse_pkey(pkey: &String) -> Result<i32, UFMError> {
    let p = pkey.trim_start_matches("0x");
    let k = i32::from_str_radix(p, 16);

    match k {
        Ok(v) => Ok(v),
        Err(_e) => Err(UFMError::InvalidPKey {
            msg: pkey.to_string(),
        }),
    }
}
