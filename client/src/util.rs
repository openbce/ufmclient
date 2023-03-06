use crate::UFMError;

/// Build PKey into a String.
///
/// This is a helper function to build a PKey into a String; so it
/// can be used in String context.
///
/// The return value is 16-based, started with `0x`, e.g. 0x7f.
///
/// # Example
///
/// ```
/// use ufmclient::util::build_pkey;
///
/// let pkey = build_pkey(10);
///
/// ```
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

pub fn is_default_pkey(pkey: i32) -> bool {
    pkey == 0x7fff
}
