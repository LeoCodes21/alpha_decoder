use crate::formats::ShellcodeFormat;
use crate::string_utils::StringUtils;

const PREFIXES: &'static [&'static str] = &[
    concat!("777777777777777777777777", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("PY77777777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("7777777777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("7777777777777777777777RY", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("SY77777777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("TY77777777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("UY77777777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("VY77777777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("WY77777777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("LLLLLLLLLLLLLLLLY77777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("LLLLLLLLLLLLY777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("LLLLLLLLY7777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("Y777777777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YY77777777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YYY7777777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YYYY777777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YYYYY77777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YYYYYY7777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YYYYYYY777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YYYYYYYY77777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("VTX630WTX638VXH49HHHPVX5AAQQPVX5YYYYP5YYYD5KKYAPTTX638TDDNVDDX4Z4A63861816", "7777777777777777777777QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
];

pub struct UppercaseNoCompressAsciiFormat {
    //
}

impl UppercaseNoCompressAsciiFormat {
    fn internal_decode(&self, code: &str) -> Vec<u8> {
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

            let nb = (a << 4) | b;
            result.push(nb);
        }

        result
    }
}

impl ShellcodeFormat for UppercaseNoCompressAsciiFormat {
    fn get_name(&self) -> &'static str {
        "Uppercase ASCII (--nocompress)"
    }

    // TODO can we eliminate this duplicated code?
    fn decode(&self, code: &str) -> Option<Vec<u8>> {
        for known_prefix in PREFIXES {
            if code.starts_with(known_prefix) {
                let encoded_section = code.substring(known_prefix.len(), code.len() - known_prefix.len());
                return Some(self.internal_decode(encoded_section));
            }
        }

        None
    }
}