use crate::windows::constants;

#[derive(Clone, Copy)]
pub enum ExceptionType {
    Int3,                      // int 3 breakpoint
    Div0,                      // division by zero
    SignChangeOnDivision,      // sign change exception on division
    PopfCannotReadStack,       // popf cannot read stack
    WritingWord,               // exception writing word
    SettingRipToNonMappedAddr, // setting rip to non mapped addr
    QWordDereferencing,        // error dereferencing qword
    DWordDereferencing,        // error dereferencing dword
    WordDereferencing,         // error dereferencing word
    ByteDereferencing,         // error dereferencing byte
    BadAddressDereferencing,   // exception dereferencing bad address
    SettingXmmOperand,         // exception setting xmm operand
    ReadingXmmOperand,         // exception reading xmm operand
}

impl PartialEq for ExceptionType {
    fn eq(&self, other: &Self) -> bool {
        return *self as u32 == *other as u32;
    }
}

impl std::fmt::Display for ExceptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExceptionType::Int3 => write!(f, "int 3"),
            ExceptionType::Div0 => write!(f, "division by zero"),
            ExceptionType::SignChangeOnDivision => write!(f, "sign change exception on division"),
            ExceptionType::PopfCannotReadStack => write!(f, "popf cannot read stack"),
            ExceptionType::WritingWord => write!(f, "exception writing word"),
            ExceptionType::SettingRipToNonMappedAddr => write!(f, "setting rip to non mapped addr"),
            ExceptionType::QWordDereferencing => write!(f, "error dereferencing qword"),
            ExceptionType::DWordDereferencing => write!(f, "error dereferencing dword"),
            ExceptionType::WordDereferencing => write!(f, "error dereferencing word"),
            ExceptionType::ByteDereferencing => write!(f, "error dereferencing byte"),
            ExceptionType::BadAddressDereferencing => {
                write!(f, "exception dereferencing bad address")
            }
            ExceptionType::SettingXmmOperand => write!(f, "exception setting xmm operand"),
            ExceptionType::ReadingXmmOperand => write!(f, "exception reading xmm operand"),
        }
    }
}

pub fn exception_type_code(ex_type: ExceptionType) -> u32 {
    match ex_type {
        ExceptionType::Int3 => return constants::STATUS_BREAKPOINT,
        ExceptionType::Div0 => return constants::STATUS_INTEGER_DIVIDE_BY_ZERO,
        ExceptionType::SignChangeOnDivision => return constants::STATUS_INTEGER_OVERFLOW,
        ExceptionType::PopfCannotReadStack => return constants::STATUS_POPF_CANNOT_READ_STACK,
        ExceptionType::WritingWord => return constants::STATUS_WRITING_WORD,
        ExceptionType::SettingRipToNonMappedAddr => return constants::STATUS_READING_RIP,
        ExceptionType::QWordDereferencing => return constants::STATUS_QWORD_DEREFERENCING,
        ExceptionType::DWordDereferencing => return constants::STATUS_DWORD_DEREFERENCING,
        ExceptionType::WordDereferencing => return constants::STATUS_WORD_DEREFERENCING,
        ExceptionType::ByteDereferencing => return constants::STATUS_BYTE_DEREFERENCING,
        ExceptionType::BadAddressDereferencing => {
            return constants::STATUS_BAD_ADDRESS_DEREFERENCING
        }
        ExceptionType::SettingXmmOperand => return constants::STATUS_SETTING_XMM_OPERAND,
        ExceptionType::ReadingXmmOperand => return constants::STATUS_READING_XMM_OPERAND,
    }
}

pub fn exception_type_from_code(code: u32) -> ExceptionType {
    match code {
        constants::STATUS_BREAKPOINT => ExceptionType::Int3,
        constants::STATUS_INTEGER_DIVIDE_BY_ZERO => ExceptionType::Div0,
        constants::STATUS_INTEGER_OVERFLOW => ExceptionType::SignChangeOnDivision,
        constants::STATUS_POPF_CANNOT_READ_STACK => ExceptionType::PopfCannotReadStack,
        constants::STATUS_WRITING_WORD => ExceptionType::WritingWord,
        constants::STATUS_READING_RIP => ExceptionType::SettingRipToNonMappedAddr,
        constants::STATUS_QWORD_DEREFERENCING => ExceptionType::QWordDereferencing,
        constants::STATUS_DWORD_DEREFERENCING => ExceptionType::DWordDereferencing,
        constants::STATUS_WORD_DEREFERENCING => ExceptionType::WordDereferencing,
        constants::STATUS_BYTE_DEREFERENCING => ExceptionType::ByteDereferencing,
        constants::STATUS_BAD_ADDRESS_DEREFERENCING => ExceptionType::BadAddressDereferencing,
        constants::STATUS_SETTING_XMM_OPERAND => ExceptionType::SettingXmmOperand,
        constants::STATUS_READING_XMM_OPERAND => ExceptionType::ReadingXmmOperand,
        _ => ExceptionType::BadAddressDereferencing,
    }
}
