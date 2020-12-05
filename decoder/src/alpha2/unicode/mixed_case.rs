use crate::alpha2::MatchingOption;

const MIXED_CASE_DECODER: &'static str = "jXAQADAZABARALAYAIAQAIAQAIAhAAAZ1AIAIAJ11AIAIABABABQI1AIQIAIQI111AIAJQYAZBABABABABkMAGB9u4JB";

pub const OPTIONS: &'static [MatchingOption] = &[
    ("IAIAIAIAIAIAIAIAIAIAIAIAIAIA4444", MIXED_CASE_DECODER, "Mixed-case Unicode (NOPs or BufferRegister=ECX)"),
    ("PPYAIAIAIAIAIAIAIAIAIAIAIAIAIAIA", MIXED_CASE_DECODER, "Mixed-case Unicode (BufferRegister=EAX)"),
    ("RRYAIAIAIAIAIAIAIAIAIAIAIAIAIAIA", MIXED_CASE_DECODER, "Mixed-case Unicode (BufferRegister=EDX)"),
    ("SSYAIAIAIAIAIAIAIAIAIAIAIAIAIAIA", MIXED_CASE_DECODER, "Mixed-case Unicode (BufferRegister=EBX)"),
    ("TUYAIAIAIAIAIAIAIAIAIAIAIAIAIAIA", MIXED_CASE_DECODER, "Mixed-case Unicode (BufferRegister=ESP)"),
    ("UUYAIAIAIAIAIAIAIAIAIAIAIAIAIAIA", MIXED_CASE_DECODER, "Mixed-case Unicode (BufferRegister=EBP)"),
    ("VVYAIAIAIAIAIAIAIAIAIAIAIAIAIAIA", MIXED_CASE_DECODER, "Mixed-case Unicode (BufferRegister=ESI)"),
    ("WWYAIAIAIAIAIAIAIAIAIAIAIAIAIAIA", MIXED_CASE_DECODER, "Mixed-case Unicode (BufferRegister=EDI)"),
    ("YAIAIAIAIAIAIAIAIAIAIAIAIAIAIA44", MIXED_CASE_DECODER, "Mixed-case Unicode (Buffer @ [esp])"),
    ("YUYAIAIAIAIAIAIAIAIAIAIAIAIAIAIA", MIXED_CASE_DECODER, "Mixed-case Unicode (Buffer @ [esp+0x4])"),

    // --nocompress
    ("444444444444444444444444444444444444444", MIXED_CASE_DECODER, "Mixed-case Unicode (NOPS or BufferRegister=ECX, --nocompress)"),
    ("PPYA44444444444444444444444444444444444", MIXED_CASE_DECODER, "Mixed-case Unicode (BufferRegister=EAX, --nocompress)"),
    ("RRYA44444444444444444444444444444444444", MIXED_CASE_DECODER, "Mixed-case Unicode (BufferRegister=EDX, --nocompress)"),
    ("SSYA44444444444444444444444444444444444", MIXED_CASE_DECODER, "Mixed-case Unicode (BufferRegister=EBX, --nocompress)"),
    ("TUYA44444444444444444444444444444444444", MIXED_CASE_DECODER, "Mixed-case Unicode (BufferRegister=ESP, --nocompress)"),
    ("UUYA44444444444444444444444444444444444", MIXED_CASE_DECODER, "Mixed-case Unicode (BufferRegister=EBP, --nocompress)"),
    ("VVYA44444444444444444444444444444444444", MIXED_CASE_DECODER, "Mixed-case Unicode (BufferRegister=ESI, --nocompress)"),
    ("WWYA44444444444444444444444444444444444", MIXED_CASE_DECODER, "Mixed-case Unicode (BufferRegister=EDI, --nocompress)"),
    ("YA4444444444444444444444444444444444444", MIXED_CASE_DECODER, "Mixed-case Unicode (Buffer @ [esp], --nocompress)"),
    ("YUYA44444444444444444444444444444444444", MIXED_CASE_DECODER, "Mixed-case Unicode (Buffer @ [esp+0x4], --nocompress)")
];