use crate::alpha2::{Alpha2Code, Alpha2CodeFactory};
use crate::string_utils::StringUtils;

pub struct Alpha2AsciiFactory {}

impl Alpha2AsciiFactory {
    fn decode(&self, code: &str) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let max_n = (code.len() / 2) - 1;

        for n in 0..code.len() / 2 {
            let val1 = code.chars().nth(2 * n).unwrap();

            if val1 == 'A' && n == max_n {
                break;
            }

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

// (prefix, decoder, id)
struct MatchingOption(&'static str, &'static str, &'static str);

impl Alpha2CodeFactory for Alpha2AsciiFactory {
    fn parse(&self, code: &str) -> Option<Box<Alpha2Code>> {
        let options: Vec<MatchingOption> = vec![
            MatchingOption("IIIIIIIIIIIIIIIIII7", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI", "Mixed-case ASCII (NOPs)")
        ];

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