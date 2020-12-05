use crate::Alpha2CodeFactory;

mod mixed_case;
mod uppercase;

pub struct Alpha2UnicodeFactory {}

impl Alpha2CodeFactory for Alpha2UnicodeFactory {
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

            if val1 == 'A' && val2 == 'A' {
                break;
            }

            let ord1: u8 = val1 as u8;
            let ord2: u8 = val2 as u8;

            let d = ord1 & 0x0F;
            let e = (ord2 >> 4) & 0x0F;
            let a = (d + e) & 0x0f;
            let b = ord2 & 0x0f;

            result.push((a << 4) | b);
        }

        result
    }
}