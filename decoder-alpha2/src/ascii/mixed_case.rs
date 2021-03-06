use crate::MatchingOption;

const MIXED_CASE_DECODER: &'static str = "jAXP0A0AkAAQ2AB2BB0BBABXP8ABuJI";

pub const OPTIONS: &'static [MatchingOption] = &[
    ("IIIIIIIIIIIIIIIIII7", MIXED_CASE_DECODER, "Mixed-case ASCII (NOPs)"),
    ("PYIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EAX)"),
    ("IIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=ECX)"),
    ("JJJJJJJJJJJJJJJJJ7RY", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EDX)"),
    ("SYIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EBX)"),
    ("TYIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=ESP)"),
    ("UYIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EBP)"),
    ("VYIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=ESI)"),
    ("WYIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EDI)"),
    ("LLLLLLLLLLLLLLLLYIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp-0x10])"),
    ("LLLLLLLLLLLLYIIIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp-0xC])"),
    ("LLLLLLLLYIIIIIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp-0x8])"),
    ("LLLLYIIIIIIIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp-0x4])"),
    ("YIIIIIIIIIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp])"),
    ("YYIIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x4])"),
    ("YYYIIIIIIIIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x8])"),
    ("YYYYIIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0xC])"),
    ("YYYYYIIIIIIIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x10])"),
    ("YYYYYYIIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x14])"),
    ("YYYYYYYIIIIIIIIIIIIIIQZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x18])"),
    ("YYYYYYYYIIIIIIIIIIIII7QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x1C])"),
    (concat!("VTX630VXH49HHHPhYAAQhZYYYYAAQQDDDd36FFFFTXVj0PPTUPPa301089", "IIIIIIIIIIIIIIIII7QZ"), MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=ECX, SEH)"),

    // --nocompress
    ("7777777777777777777777777777777777777", MIXED_CASE_DECODER, "Mixed-case ASCII (NOPS, --nocompress)"),
    ("PY777777777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EAX, --nocompress)"),
    ("77777777777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=ECX, --nocompress)"),
    ("77777777777777777777777777777777777RY", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EDX, --nocompress)"),
    ("SY777777777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EBX, --nocompress)"),
    ("TY777777777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=ESP, --nocompress)"),
    ("UY777777777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EBP, --nocompress)"),
    ("VY777777777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=ESI, --nocompress)"),
    ("WY777777777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=EDI, --nocompress)"),
    ("LLLLLLLLLLLLLLLLY777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp-0x10], --nocompress)"),
    ("LLLLLLLLLLLLY7777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp-0xC], --nocompress)"),
    ("LLLLLLLLY77777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp-0x8], --nocompress)"),
    ("LLLL7Y77777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp-0x4], --nocompress)"),
    ("Y7777777777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp], --nocompress)"),
    ("YY777777777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x4], --nocompress)"),
    ("YYY77777777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x8], --nocompress)"),
    ("YYYY7777777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0xC], --nocompress)"),
    ("YYYYY777777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x10], --nocompress)"),
    ("YYYYYY77777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x14], --nocompress)"),
    ("YYYYYYY7777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x18], --nocompress)"),
    ("YYYYYYYY777777777777777777777777777QZ", MIXED_CASE_DECODER, "Mixed-case ASCII (Buffer @ [esp+0x1C], --nocompress)"),
    (concat!("VTX630VXH49HHHPhYAAQhZYYYYAAQQDDDd36FFFFTXVj0PPTUPPa301089", "77777777777777777777777777777777777QZ"), MIXED_CASE_DECODER, "Mixed-case ASCII (BufferRegister=ECX, SEH, --nocompress)"),
];