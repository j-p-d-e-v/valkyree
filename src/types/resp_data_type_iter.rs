use crate::types::resp_data_kind::RespDataType;

#[derive(Debug, Clone)]
pub struct RespDataTypeIterator<'a> {
    values: &'a [u8],
    current: usize,
}

impl<'a> RespDataTypeIterator<'a> {
    pub fn new(values: &'a [u8]) -> Self {
        Self { values, current: 0 }
    }

    pub fn nnext(&mut self, n: usize) -> Option<Vec<u8>> {
        let mut current = self.current;
        let length = self.values.len();
        let start = current;
        let end = current + n;
        let mut data: Vec<u8> = Vec::new();
        for i in start..end {
            if i < length {
                data.push(self.values[current]);
            }
            current += 1;
        }
        self.current = current;
        Some(data)
    }

    pub fn npeek(&self, n: usize) -> Option<Vec<u8>> {
        let current = self.current;
        let length = self.values.len();
        let start = current;
        let end = current + n;
        let mut data: Vec<u8> = Vec::new();
        for i in start..end {
            if i < length {
                data.push(self.values[i]);
            }
        }
        if data.is_empty() { None } else { Some(data) }
    }

    pub fn get_data_type(&self) -> Option<RespDataType> {
        if self.current < self.values.len()
            && let value = &self.values[self.current]
            && let Ok(kind) = RespDataType::identify(*value)
        {
            Some(kind)
        } else {
            None
        }
    }
}

impl<'a> Iterator for RespDataTypeIterator<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        let value = if current < self.values.len() {
            Some(&self.values[current])
        } else {
            None
        };
        self.current += 1;
        value
    }
}
