use crate::alpha2::{Alpha2Code, Alpha2CodeFactory};
use crate::string_utils::StringUtils;

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

// (prefix, decoder, id)
struct MatchingOption(&'static str, &'static str, &'static str);

const MIXED_CASE_DECODER: &'static str = "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI";

impl Alpha2CodeFactory for Alpha2AsciiFactory {
    fn parse(&self, code: &str) -> Option<Box<Alpha2Code>> {
        let options: Vec<MatchingOption> = vec![
            // Mixed-case
            MatchingOption("IIIIIIIIIIIIIIIIII7", MIXED_CASE_DECODER, "Mixed-case ASCII (NOPs)"),
            MatchingOption("PYIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EAX)"),
            MatchingOption("IIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=ECX)"),
            MatchingOption("JJJJJJJJJJJJJJJJJ7RY", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EDX)"),
            MatchingOption("SYIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EBX)"),
            MatchingOption("TYIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=ESP)"),
            MatchingOption("UYIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EBP)"),
            MatchingOption("VYIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=ESI)"),
            MatchingOption("WYIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EDI)"),
            MatchingOption("LLLLLLLLLLLLLLLLYIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp-0x10])"),
            MatchingOption("LLLLLLLLLLLLYIIIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp-0xC])"),
            MatchingOption("LLLLLLLLYIIIIIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp-0x8])"),
            MatchingOption("LLLLYIIIIIIIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp-0x4])"),
            MatchingOption("YIIIIIIIIIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp])"),
            MatchingOption("YYIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x4])"),
            MatchingOption("YYYIIIIIIIIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x8])"),
            MatchingOption("YYYYIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0xC])"),
            MatchingOption("YYYYYIIIIIIIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x10])"),
            MatchingOption("YYYYYYIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x14])"),
            MatchingOption("YYYYYYYIIIIIIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x18])"),
            MatchingOption("YYYYYYYYIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x1C])"),
            MatchingOption(concat!("VTX630VXH49HHHPhYAAQhZYYYYAAQQDDDd36FFFFTXVj0PPTUPPa301089", "IIIIIIIIIIIIIIIII7QZ"), MIXED_CASE_DECODER, "Mixed-case ASCII (SEH, BufferRegister=ECX)")
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