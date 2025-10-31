pub mod arrays;
pub mod big_numbers;
pub mod booleans;
pub mod bulk_errors;
pub mod bulk_strings;
pub mod doubles;
pub mod helpers;
pub mod integers;
pub mod nulls;
pub mod parser;
pub mod simple_errors;
pub mod simple_strings;
pub mod verbatim_strings;
pub use arrays::Arrays;
pub use big_numbers::BigNumbers;
pub use booleans::Booleans;
pub use bulk_errors::BulkErrors;
pub use bulk_strings::BulkStrings;
pub use doubles::Doubles;
pub use integers::Integers;
pub use nulls::Nulls;
pub use parser::RespParser;
pub use simple_errors::SimpleErrors;
pub use simple_strings::SimpleStrings;
pub use verbatim_strings::VerbatimStrings;

use crate::types::RespDataTypeValue;

pub trait RespDataTypeTrait<'a> {
    fn new(value: &'a [u8]) -> Self;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn build(&mut self) -> anyhow::Result<RespDataTypeValue>;
}
