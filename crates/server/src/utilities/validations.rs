use validator::ValidationError;

pub fn validate_pubkey(pubkey: &str) -> Result<(), ValidationError> {
    let res = bs58::decode(pubkey)
        .into_vec()
        .map_err(|_| ValidationError::new("terrible_pubkey"))?;

    if res.len() == 33 {
        Ok(())
    } else {
        Err(ValidationError::new("terrible_pubkey"))
    }
}
