use anyhow::anyhow;

/// Get the length on a given RESP input
///   Parameters:
/// - value - The array unsigned integer representation of the input.
///   Returns:
/// - new_start_index - The index where the value will start after getting the length
/// - length - The length of the value.
pub fn get_length(value: &[u8]) -> anyhow::Result<(usize, isize)> {
    let mut lengthb: Vec<u8> = Vec::new();
    for i in value {
        if i == &13 || i == &10 {
            break;
        }
        lengthb.push(i.to_owned());
    }
    let start = lengthb.len() + 2;
    let length = if let Ok(n) = String::from_utf8_lossy(&lengthb).parse::<isize>() {
        n
    } else {
        return Err(anyhow!("INVALID_LENGTH".to_string()));
    };
    Ok((start, length))
}
