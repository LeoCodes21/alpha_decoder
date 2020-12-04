use crate::formats::ShellcodeFormat;
use crate::string_utils::StringUtils;

const PREFIXES: &'static [&'static str] = &[
    concat!("IIIIIIIIIIII", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("PYIIIIIIIIIIQZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("IIIIIIIIIIIQZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("JJJJJJJJJJJRY", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("SYIIIIIIIIIIQZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("TYIIIIIIIIIIQZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("UYIIIIIIIIIIQZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("VYIIIIIIIIIIQZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("WYIIIIIIIIIIQZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("LLLLLLLLLLLLLLLLYII7QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("LLLLLLLLLLLLYIIII7QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("LLLLLLLLYIIIIII7QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("LLLL7YIIIIIIIIQZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YIIIIIIIIII7QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YYIIIIIIIIIIQZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YYYIIIIIIIII7QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YYYYIIIIIIIIIQZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YYYYYIIIIIIII7QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YYYYYYIIIIIIIIQZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YYYYYYYIIIIIII7QZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("YYYYYYYYIIIIIIIQZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
    concat!("VTX630WTX638VXH49HHHPVX5AAQQPVX5YYYYP5YYYD5KKYAPTTX638TDDNVDDX4Z4A63861816", "IIIIIIIIIIIQZ", "VTX30VX4AP0A3HH0A00ABAABTAAQ2AB2BB0BBXP8ACJJI"),
];

pub struct UppercaseAsciiFormat {
    //
}

impl UppercaseAsciiFormat {
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

impl ShellcodeFormat for UppercaseAsciiFormat {
    fn get_name(&self) -> &'static str {
        "Uppercase ASCII"
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