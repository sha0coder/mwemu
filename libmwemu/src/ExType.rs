
#[derive(Clone, Copy)]
pub enum ExceptionType {
    INT3,
    DIV0,
    SIGN_CHANGE, //sign change exception
    POPF,
    WRITE,
    RIP_SET_TO_NON_MAPPED, // rip is set to a non mapped address
    QWORD_DEREFERENCE, // qword dereference
    DWORD_DEREFERENCE, // dword dereference
    WORD_DEREFERENCE, // word dereference
    BYTE_DEREFERENCE, // byte dereference
    BAD_ADDRESS_DEREFERENCE, // bad address dereference
    XMM_OPERAND_READ, // xmm operand read
}

// support partial eq
impl PartialEq for ExceptionType {
    fn eq(&self, other: &Self) -> bool {
        return *self as u32 == *other as u32;
    }
}


impl std::fmt::Display for ExceptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExceptionType::INT3 => write!(f, "INT3"),
            ExceptionType::DIV0 => write!(f, "DIV0"),
            ExceptionType::SIGN_CHANGE => write!(f, "SIGN_CHANGE"),
            ExceptionType::POPF => write!(f, "POPF"),
            ExceptionType::WRITE => write!(f, "WRITE"),
            ExceptionType::RIP_SET_TO_NON_MAPPED => write!(f, "RIP_SET_TO_NON_MAPPED"),
            ExceptionType::QWORD_DEREFERENCE => write!(f, "QWORD_DEREFERENCE"),
            ExceptionType::DWORD_DEREFERENCE => write!(f, "DWORD_DEREFERENCE"),
            ExceptionType::WORD_DEREFERENCE => write!(f, "WORD_DEREFERENCE"),
            ExceptionType::BYTE_DEREFERENCE => write!(f, "BYTE_DEREFERENCE"),
            ExceptionType::BAD_ADDRESS_DEREFERENCE => write!(f, "BAD_ADDRESS_DEREFERENCE"),
            ExceptionType::XMM_OPERAND_READ => write!(f, "XMM_OPERAND_READ"),
        }
    }
}