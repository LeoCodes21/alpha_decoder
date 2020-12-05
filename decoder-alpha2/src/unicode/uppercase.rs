use crate::MatchingOption;

const UPPERCASE_DECODER: &'static str = "QATAXAZAPA3QADAZABARALAYAIAQAIAQAPA5AAAPAZ1AI1AIAIAJ11AIAIAXA58AAPAZABABQI1AIQIAIQI1111AIAJQI1AYAZBABABABAB30APB944JB";

pub const OPTIONS: &'static [MatchingOption] = &[
    ("IAIAIAIA4444", UPPERCASE_DECODER, "Uppercase Unicode (NOPs or BufferRegister=ECX)"),
    ("PPYAIAIAIAIA", UPPERCASE_DECODER, "Uppercase Unicode (BufferRegister=EAX)"),
    ("RRYAIAIAIAIA", UPPERCASE_DECODER, "Uppercase Unicode (BufferRegister=EDX)"),
    ("SSYAIAIAIAIA", UPPERCASE_DECODER, "Uppercase Unicode (BufferRegister=EBX)"),
    ("TUYAIAIAIAIA", UPPERCASE_DECODER, "Uppercase Unicode (BufferRegister=ESP)"),
    ("UUYAIAIAIAIA", UPPERCASE_DECODER, "Uppercase Unicode (BufferRegister=EBP)"),
    ("VVYAIAIAIAIA", UPPERCASE_DECODER, "Uppercase Unicode (BufferRegister=ESI)"),
    ("WWYAIAIAIAIA", UPPERCASE_DECODER, "Uppercase Unicode (BufferRegister=EDI)"),
    ("YAIAIAIAIA44", UPPERCASE_DECODER, "Uppercase Unicode (Buffer @ [esp])"),
    ("YUYAIAIAIAIA", UPPERCASE_DECODER, "Uppercase Unicode (Buffer @ [esp+0x4])"),

    // --nocompress
    ("44444444444444", UPPERCASE_DECODER, "Uppercase Unicode (NOPS or BufferRegister=ECX, --nocompress)"),
    ("PPYA4444444444", UPPERCASE_DECODER, "Uppercase Unicode (BufferRegister=EAX, --nocompress)"),
    ("RRYA4444444444", UPPERCASE_DECODER, "Uppercase Unicode (BufferRegister=EDX, --nocompress)"),
    ("SSYA4444444444", UPPERCASE_DECODER, "Uppercase Unicode (BufferRegister=EBX, --nocompress)"),
    ("TUYA4444444444", UPPERCASE_DECODER, "Uppercase Unicode (BufferRegister=ESP, --nocompress)"),
    ("UUYA4444444444", UPPERCASE_DECODER, "Uppercase Unicode (BufferRegister=EBP, --nocompress)"),
    ("VVYA4444444444", UPPERCASE_DECODER, "Uppercase Unicode (BufferRegister=ESI, --nocompress)"),
    ("WWYA4444444444", UPPERCASE_DECODER, "Uppercase Unicode (BufferRegister=EDI, --nocompress)"),
    ("YA444444444444", UPPERCASE_DECODER, "Uppercase Unicode (Buffer @ [esp], --nocompress)"),
    ("YUYA4444444444", UPPERCASE_DECODER, "Uppercase Unicode (Buffer @ [esp+0x4], --nocompress)")
];