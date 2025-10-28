use anyhow::anyhow;

use crate::types::{resp_data_kind::RespDataType, resp_data_type_iter::RespDataTypeIterator};

/// Get the resp data type, value part, and remove the terminator (CR,LF).
/// Parameters:
/// - value - The array unsigned integer representation of the input.
/// - remove_terminator - If true, remove the terminator.
///   Returns:
/// - value - The array unsigned integer value.
/// - id - The resp data type.
pub fn get_resp_value(
    value: &[u8],
    remove_terminator: bool,
) -> anyhow::Result<(&[u8], RespDataType)> {
    let id = if let Some(id) = value.first() {
        RespDataType::identify(*id)?
    } else {
        return Err(anyhow!("INVALID_RESP_DATA_TYPE"));
    };
    let crlf = if remove_terminator { 2 } else { 0 };
    match value.get(1..value.len() - crlf) {
        Some(data) => Ok((data, id)),
        None => Err(anyhow!("INVALID_CRLF_TERMINATOR")),
    }
}

/// Get the starting value index, resp data type, and length of a value
/// Parameters:
/// - value - The array unsigned integer representation of the input.
///   Returns:
/// - new_start_index - The index where the value will start after getting the length
/// - length - The length of the value.
/// - id - The resp data type.
pub fn get_resp_multi_values(value: &[u8]) -> anyhow::Result<(usize, isize, RespDataType)> {
    let mut length: Vec<u8> = Vec::new();
    let mut iter = RespDataTypeIterator::new(value);
    let id = if let Some(id) = iter.next() {
        RespDataType::identify(*id)?
    } else {
        return Err(anyhow!("INVALID_RESP_DATA_TYPE"));
    };
    while let Some(v) = iter.next() {
        length.push(*v);
        if let Some(peek_values) = iter.npeek(2)
            && let Some(cr) = peek_values.first()
            && is_cr(cr)
            && let Some(lf) = peek_values.get(1)
            && is_lf(lf)
        {
            break;
        }
    }
    let start = length.len() + 3;
    let length = if let Ok(n) = String::from_utf8_lossy(&length).parse::<isize>() {
        n
    } else {
        return Err(anyhow!("INVALID_LENGTH".to_string()));
    };
    Ok((start, length, id))
}

/// Checks if the value is a carriage return
/// Parameters:
/// value - An ascii decimal value to check.
pub fn is_cr(value: &u8) -> bool {
    value == &13
}

/// Checks if the value is a line feed
/// Parameters:
/// value - An ascii decimal value to check.
pub fn is_lf(value: &u8) -> bool {
    value == &10
}
