use crate::formats::ShellcodeFormat;
use crate::string_utils::StringUtils;

const PREFIXES: &'static [&'static str] = &[
    concat!("IAIAIAIA4444", "QATAXAZAPA3QADAZABARALAYAIAQAIAQAPA5AAAPAZ1AI1AIAIAJ11AIAIAXA58AAPAZABABQI1AIQIAIQI1111AIAJQI1AYAZBABABABAB30APB944JB"),
    concat!("PPYAIAIAIAIA", "QATAXAZAPA3QADAZABARALAYAIAQAIAQAPA5AAAPAZ1AI1AIAIAJ11AIAIAXA58AAPAZABABQI1AIQIAIQI1111AIAJQI1AYAZBABABABAB30APB944JB"),
    concat!("IAIAIAIA4444", "QATAXAZAPA3QADAZABARALAYAIAQAIAQAPA5AAAPAZ1AI1AIAIAJ11AIAIAXA58AAPAZABABQI1AIQIAIQI1111AIAJQI1AYAZBABABABAB30APB944JB"),
    concat!("RRYAIAIAIAIA", "QATAXAZAPA3QADAZABARALAYAIAQAIAQAPA5AAAPAZ1AI1AIAIAJ11AIAIAXA58AAPAZABABQI1AIQIAIQI1111AIAJQI1AYAZBABABABAB30APB944JB"),
    concat!("SSYAIAIAIAIA", "QATAXAZAPA3QADAZABARALAYAIAQAIAQAPA5AAAPAZ1AI1AIAIAJ11AIAIAXA58AAPAZABABQI1AIQIAIQI1111AIAJQI1AYAZBABABABAB30APB944JB"),
    concat!("TUYAIAIAIAIA", "QATAXAZAPA3QADAZABARALAYAIAQAIAQAPA5AAAPAZ1AI1AIAIAJ11AIAIAXA58AAPAZABABQI1AIQIAIQI1111AIAJQI1AYAZBABABABAB30APB944JB"),
    concat!("UUYAIAIAIAIA", "QATAXAZAPA3QADAZABARALAYAIAQAIAQAPA5AAAPAZ1AI1AIAIAJ11AIAIAXA58AAPAZABABQI1AIQIAIQI1111AIAJQI1AYAZBABABABAB30APB944JB"),
    concat!("VVYAIAIAIAIA", "QATAXAZAPA3QADAZABARALAYAIAQAIAQAPA5AAAPAZ1AI1AIAIAJ11AIAIAXA58AAPAZABABQI1AIQIAIQI1111AIAJQI1AYAZBABABABAB30APB944JB"),
    concat!("WWYAIAIAIAIA", "QATAXAZAPA3QADAZABARALAYAIAQAIAQAPA5AAAPAZ1AI1AIAIAJ11AIAIAXA58AAPAZABABQI1AIQIAIQI1111AIAJQI1AYAZBABABABAB30APB944JB"),
    concat!("YAIAIAIAIA44", "QATAXAZAPA3QADAZABARALAYAIAQAIAQAPA5AAAPAZ1AI1AIAIAJ11AIAIAXA58AAPAZABABQI1AIQIAIQI1111AIAJQI1AYAZBABABABAB30APB944JB"),
    concat!("YUYAIAIAIAIA", "QATAXAZAPA3QADAZABARALAYAIAQAIAQAPA5AAAPAZ1AI1AIAIAJ11AIAIAXA58AAPAZABABQI1AIQIAIQI1111AIAJQI1AYAZBABABABAB30APB944JB"),
];

pub struct UppercaseUnicodeFormat {
    //
}

impl UppercaseUnicodeFormat {
    fn internal_decode(&self, code: &str) -> Vec<u8> {
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

impl ShellcodeFormat for UppercaseUnicodeFormat {
    fn get_name(&self) -> &'static str {
        "Uppercase Unicode"
    }

    fn decode(&self, code: &str) -> Option<Vec<u8>> {
        if code.len() % 2 != 0 {
            return None;
        }

        for known_prefix in PREFIXES {
            if code.starts_with(known_prefix) {
                let encoded_section = code.substring(known_prefix.len(), code.len() - known_prefix.len());
                return Some(self.internal_decode(encoded_section));
            }
        }

        None
    }
}