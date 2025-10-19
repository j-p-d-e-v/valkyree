pub mod arrays;
pub mod big_numbers;
pub mod booleans;
pub mod bulk_strings;
pub mod doubles;
pub mod helpers;
pub mod integers;
pub mod nulls;
pub mod parser;
pub mod simple_errors;
pub mod simple_strings;
use crate::types::resp_data_kind::RespDataType;
use anyhow::anyhow;
pub use arrays::Arrays;
pub use big_numbers::BigNumbers;
pub use booleans::Booleans;
pub use bulk_strings::BulkStrings;
pub use doubles::Doubles;
pub use integers::Integers;
pub use nulls::Nulls;
pub use parser::RespParser;
pub use simple_errors::SimpleErrors;
pub use simple_strings::SimpleStrings;
trait RespDataTypeBase {
    fn is_data_type(value: &[u8], identifier: RespDataType) -> anyhow::Result<()> {
        let b = match value.first() {
            Some(b) => b,
            None => {
                return Err(anyhow!("DATA_TYPE_NOT_FOUND".to_string()));
            }
        };
        let data_type = RespDataType::identify(b.to_owned())?;
        if data_type == identifier {
            Ok(())
        } else {
            Err(anyhow!("RESP_DATA_TYPE_MISMATCH"))
        }
    }

    // Get the value of the input removing the data type and CRLF (terminator) at the last.
    fn get_value(value: &[u8], remove_crlf: bool) -> anyhow::Result<Vec<u8>> {
        let crlf = if remove_crlf { 2 } else { 0 };
        match value.get(1..value.len() - crlf) {
            Some(data) => Ok(data.to_vec()),
            None => Err(anyhow!("INVALID_CRLF_TERMINATOR")),
        }
    }
}
