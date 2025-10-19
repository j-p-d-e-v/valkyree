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
    fn get_value(value: &[u8]) -> anyhow::Result<Vec<u8>> {
        match value.get(1..value.len() - 2) {
            Some(data) => Ok(data.to_vec()),
            None => Err(anyhow!("INVALID_CRLF_TERMINATOR")),
        }
    }
}
