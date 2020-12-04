use crate::formats::ShellcodeFormat;
use crate::string_utils::StringUtils;

const PREFIXES: &'static [&'static str] = &[
    concat!("444444444444444444444444444444444444444", "jXAQADAZABARALAYAIAQAIAQAIAhAAAZ1AIAIAJ11AIAIABABABQI1AIQIAIQI111AIAJQYAZBABABABABkMAGB9u4JB"),
    concat!("PPYA44444444444444444444444444444444444", "jXAQADAZABARALAYAIAQAIAQAIAhAAAZ1AIAIAJ11AIAIABABABQI1AIQIAIQI111AIAJQYAZBABABABABkMAGB9u4JB"),
    concat!("RRYA44444444444444444444444444444444444", "jXAQADAZABARALAYAIAQAIAQAIAhAAAZ1AIAIAJ11AIAIABABABQI1AIQIAIQI111AIAJQYAZBABABABABkMAGB9u4JB"),
    concat!("SSYA44444444444444444444444444444444444", "jXAQADAZABARALAYAIAQAIAQAIAhAAAZ1AIAIAJ11AIAIABABABQI1AIQIAIQI111AIAJQYAZBABABABABkMAGB9u4JB"),
    concat!("TUYA44444444444444444444444444444444444", "jXAQADAZABARALAYAIAQAIAQAIAhAAAZ1AIAIAJ11AIAIABABABQI1AIQIAIQI111AIAJQYAZBABABABABkMAGB9u4JB"),
    concat!("UUYA44444444444444444444444444444444444", "jXAQADAZABARALAYAIAQAIAQAIAhAAAZ1AIAIAJ11AIAIABABABQI1AIQIAIQI111AIAJQYAZBABABABABkMAGB9u4JB"),
    concat!("VVYA44444444444444444444444444444444444", "jXAQADAZABARALAYAIAQAIAQAIAhAAAZ1AIAIAJ11AIAIABABABQI1AIQIAIQI111AIAJQYAZBABABABABkMAGB9u4JB"),
    concat!("WWYA44444444444444444444444444444444444", "jXAQADAZABARALAYAIAQAIAQAIAhAAAZ1AIAIAJ11AIAIABABABQI1AIQIAIQI111AIAJQYAZBABABABABkMAGB9u4JB"),
    concat!("YA4444444444444444444444444444444444444", "jXAQADAZABARALAYAIAQAIAQAIAhAAAZ1AIAIAJ11AIAIABABABQI1AIQIAIQI111AIAJQYAZBABABABABkMAGB9u4JB"),
    concat!("YUYA44444444444444444444444444444444444", "jXAQADAZABARALAYAIAQAIAQAIAhAAAZ1AIAIAJ11AIAIABABABQI1AIQIAIQI111AIAJQYAZBABABABABkMAGB9u4JB"),
];

pub struct MixedCaseNoCompressUnicodeFormat {
    //
}

impl MixedCaseNoCompressUnicodeFormat {
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

impl ShellcodeFormat for MixedCaseNoCompressUnicodeFormat {
    fn get_name(&self) -> &'static str {
        "Mixed-case Unicode (--nocompress)"
    }

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