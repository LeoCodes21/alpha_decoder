use crate::formats::ShellcodeFormat;
use crate::string_utils::StringUtils;

const PREFIXES: &'static [&'static str] = &[
    concat!("IIIIIIIIIIIIIIIIII7", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("PYIIIIIIIIIIIIIIII7QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("IIIIIIIIIIIIIIIII7QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("JJJJJJJJJJJJJJJJJ7RY", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("SYIIIIIIIIIIIIIIII7QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("TYIIIIIIIIIIIIIIII7QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("UYIIIIIIIIIIIIIIII7QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("VYIIIIIIIIIIIIIIII7QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("WYIIIIIIIIIIIIIIII7QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("LLLLLLLLLLLLLLLLYIIIIIIIIIQZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("LLLLLLLLLLLLYIIIIIIIIIIIQZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("LLLLLLLLYIIIIIIIIIIIIIQZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("LLLLYIIIIIIIIIIIIIIIQZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("YIIIIIIIIIIIIIIIIIQZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("YYIIIIIIIIIIIIIIII7QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("YYYIIIIIIIIIIIIIIIIQZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("YYYYIIIIIIIIIIIIIII7QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("YYYYYIIIIIIIIIIIIIIIQZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("YYYYYYIIIIIIIIIIIIII7QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("YYYYYYYIIIIIIIIIIIIIIQZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
    concat!("VTX630VXH49HHHPhYAAQhZYYYYAAQQDDDd36FFFFTXVj0PPTUPPa301089", "IIIIIIIIIIIIIIIII7QZ", "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI"),
];

pub struct MixedCaseAsciiFormat {
    //
}

impl MixedCaseAsciiFormat {
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

impl ShellcodeFormat for MixedCaseAsciiFormat {
    fn get_name(&self) -> &'static str {
        "Mixed-case ASCII"
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