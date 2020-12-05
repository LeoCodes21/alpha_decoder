use std::fmt::{Display, Formatter};

use crate::format::*;

pub mod ascii;
pub mod unicode;

pub struct Alpha2Code {
    prefix: String,
    decoder_body: String,
    encoded_data: String,
    code_type: String,
    decoded_data: Vec<u8>,
}

impl Alpha2Code {
    fn new(prefix: &str, decoder_body: &str, encoded_data: &str, code_type: &str, decoded_data: Vec<u8>) -> Self {
        Alpha2Code {
            prefix: prefix.to_string(),
            decoder_body: decoder_body.to_string(),
            encoded_data: encoded_data.to_string(),
            code_type: code_type.to_string(),
            decoded_data,
        }
    }
}

impl Display for Alpha2Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.code_type);
    }
}

impl ParsedCode for Alpha2Code {
    fn decode(&self) -> Vec<u8> {
        self.decoded_data.clone()
    }
}

trait Alpha2CodeFactory {
    fn parse(&self, code: &str) -> Option<Box<Alpha2Code>>;
}

pub struct Alpha2CodeFormat {
    //
}

impl CodeFormat for Alpha2CodeFormat {
    fn get_name(&self) -> &'static str {
        "Alpha2"
    }

    fn parse(&self, code: &str) -> Option<Box<dyn ParsedCode>> {
        let factories: Vec<Box<dyn Alpha2CodeFactory>> = vec![
            Box::new(ascii::Alpha2AsciiFactory {})
        ];

        for factory in factories {
            let parsed = factory.parse(code);

            if parsed.is_some() {
                return Some(parsed.unwrap());
            }
        }

        None
    }
}