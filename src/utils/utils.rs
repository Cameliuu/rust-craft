use vintor::{encode,EncodeError};
pub fn extend_with_string(vec: &mut Vec<u8>, string: String) -> Result<(),EncodeError> {
    let str_len = encode(string.len() as i32)?;
    let str_bytes = string.as_bytes();

    vec.extend(str_len);
    vec.extend(str_bytes);

    Ok(())
}
