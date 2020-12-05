use crate::alpha2::{Alpha2Code, Alpha2CodeFactory};
use crate::string_utils::StringUtils;

mod mixed_case;
mod uppercase;

pub struct Alpha2AsciiFactory {}

impl Alpha2CodeFactory for Alpha2AsciiFactory {
    fn get_options(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        let options: &mut Vec<(&'static str, &'static str, &'static str)> = &mut vec![];
        options.extend_from_slice(mixed_case::OPTIONS);
        options.extend_from_slice(uppercase::OPTIONS);
        options.to_vec()
    }
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