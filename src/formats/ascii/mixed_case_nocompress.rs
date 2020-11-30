use crate::formats::ShellcodeFormat;
use crate::string_utils::StringUtils;

const PREFIXES: &'static [&'static str] = &[
    concat!("7777777777777777777777777777777777777", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("PY777777777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("77777777777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("77777777777777777777777777777777777RY", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("SY777777777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("TY777777777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("UY777777777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("VY777777777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("WY777777777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("LLLLLLLLLLLLLLLLY777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("LLLLLLLLLLLLY7777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("LLLLLLLLY77777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("LLLL7Y77777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("Y7777777777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("YY777777777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("YYY77777777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("YYYY7777777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("YYYYY777777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("YYYYYY77777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("YYYYYYY7777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("YYYYYYYY777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("VTX630VXH49HHHPhYAAQhZYYYYAAQQDDDd36FFFFTXVj0PPTUPPa301089", "77777777777777777777777777777777777QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
];

pub struct MixedCaseNoCompressAsciiFormat {
    //
}

impl MixedCaseNoCompressAsciiFormat {
    fn internal_decode(&self, code: &str) -> Vec<u8> {
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

impl ShellcodeFormat for MixedCaseNoCompressAsciiFormat {
    fn get_name(&self) -> &'static str {
        "Mixed-case ASCII (--nocompress)"
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