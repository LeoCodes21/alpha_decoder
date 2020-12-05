use crate::alpha2::{Alpha2Code, Alpha2CodeFactory};
use crate::string_utils::StringUtils;

mod mixed_case;
mod uppercase;

type MatchingOption = (&'static str, &'static str, &'static str);

pub struct Alpha2AsciiFactory {}

impl Alpha2AsciiFactory {
    fn decode(&self, code: &str) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        for n in 0..code.len() / 2 {
            let val1 = code.chars().nth(2 * n).unwrap();
            let val2 = code.chars().nth(2 * n + 1).unwrap();
            let ord1: u8 = val1 as u8;
            let ord2: u8 = val2 as u8;

            let d = (ord1 >> 0) & 0x0f;
            let e = (ord2 >> 4) & 0x0f;
            let f = (ord2 >> 0) & 0x0f;

            let a = d ^ e;
            let b = f;

            result.push((a << 4) | b);
        }

        result
    }
}

impl Alpha2CodeFactory for Alpha2AsciiFactory {
    fn parse(&self, code: &str) -> Option<Box<Alpha2Code>> {
        let options: &mut Vec<MatchingOption> = &mut vec![];
        options.extend_from_slice(mixed_case::OPTIONS);
        options.extend_from_slice(uppercase::OPTIONS);

        for option in options {
            let full_prefix: String = option.0.to_owned() + option.1;
            if code.starts_with(full_prefix.as_str()) {
                let encoded_part = code.substring(full_prefix.len(), code.len() - full_prefix.len());

                return Some(Box::new(Alpha2Code {
                    prefix: option.0.to_string(),
                    decoder_body: option.1.to_string(),
                    encoded_data: encoded_part.to_string(),
                    code_type: option.2.to_string(),
                    decoded_data: self.decode(encoded_part),
                }));
            }
        }

        return None;
    }
}