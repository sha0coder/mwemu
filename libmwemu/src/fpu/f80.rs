use serde::{Serialize,Deserialize};

pub const FPU_80_BITS_MAX: u128 = (1u128 << 80) - 1;
pub const QNAN: u128 = 0xffffc000000000000000 & FPU_80_BITS_MAX;
pub const SIGN_MASK: u128 = 1 << 79;
pub const EXP_MASK: u128 = 0x7FFF;
pub const MANTISSA_MASK: u128 = (1u128 << 64) - 1;
pub const MANTISSA_MASK_NOINT: u128 = 0x7FFF_FFFF_FFFF_FFFF;
pub const INT_BIT_MASK: u128 = 1 << 63;
pub const BCD_SIGN_POSITIVE: u8 = 0x0A;
pub const BCD_SIGN_NEGATIVE: u8 = 0x0B;
pub const F64_EXP_BIAS: i32 = 1023;
pub const F80_EXP_BIAS: i32 = 16383;

// f80 emulation
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct F80 {
    pub st:u128,
}

impl F80 {
    pub fn new() -> Self {
        F80 {
            st: 0
        }
    }

    pub fn set_bytes(&mut self, bytes: &[u8; 10]) {
        let mut val: u128 = 0;
        for (i, &b) in bytes.iter().enumerate() {
            val |= (b as u128) << (8 * i);
        }
        self.st = val & FPU_80_BITS_MAX;
    }

    pub fn get_bytes(&self) -> [u8; 10] {
        let mut bytes = [0u8; 10];
        for i in 0..10 {
            bytes[i] = ((self.st >> (8 * i)) & 0xFF) as u8;
            
        }
        bytes
    }

    // I keep this getter, but it make no sense, fpu instruccions will write to memory in bytes.
    pub fn get(&self) -> u128 {
        self.st & FPU_80_BITS_MAX
    }

    // I keep this getter, but it make no sense, fpu instruccions will derref memory in bytes.
    pub fn set(&mut self, value:u128) {
        self.st = value & FPU_80_BITS_MAX;
    }

    pub fn fix(&mut self) {
        self.st = self.st & FPU_80_BITS_MAX;
    }

    pub fn is_negative(&self) -> bool {
        self.st & SIGN_MASK != 0
    }

    pub fn is_zero(&self) -> bool {
        let exponent = (self.st >> 64) & 0x7FFF;
        let mantissa = self.st & ((1u128 << 64) - 1);
        exponent == 0 && mantissa == 0
    }

    pub fn get_exponent(self) -> u16 {
        // Bits 78–64: exponent con bias 16383
        (self.st >> 64 & EXP_MASK) as u16
    }

    pub fn get_mantissa(&self) -> u64 {
        let mantissa = (self.st >> 11) &  MANTISSA_MASK_NOINT;
        let bit_integer = (self.st >> 63) & 1;
        (mantissa | (bit_integer << 52)) as u64
    }

    fn get_mantissa_with_integer_bit(&self) -> u64 {
        let mantissa = (self.st & ((1u128 << 63) - 1)) as u64; // 63 bits
        let bit_integer = ((self.st >> 63) & 1) as u64;
        mantissa | (bit_integer << 63)
    }


    pub fn is_integer(&self) -> bool {
        self.get_mantissa() == 0
    }

    pub fn is_normal(&self) -> bool {
        let exponent = (self.st >> 64) & 0x7FFF;
        exponent != 0 && exponent != 0x7FFF
    }

    pub fn bit_integer(&self) -> bool {
       (self.st & INT_BIT_MASK) & 1 == 1
    }

    pub fn is_denormal(&self) -> bool {
        let exponent = (self.st >> 64) & 0x7FFF;
        exponent == 0
    }

    pub fn is_infinite(&self) -> bool {
        let exponent = (self.st >> 64) & 0x7FFF;
        let mantissa = self.st & ((1u128 << 64) - 1);
        exponent == 0x7FFF && mantissa == 0
    }

    pub fn is_nan(&self) -> bool {
        let exponent = (self.st >> 64) & 0x7FFF;
        let mantissa = self.st & MANTISSA_MASK;
        exponent == 0x7FFF && mantissa != 0
    }

    pub fn to_bcd_packed(&self) -> [u8; 10] {
        let mut val = self.to_integer_u128();
        let mut bcd = [0u8; 10];

        for i in 0..9 {
            let lo = (val % 10) as u8;
            val /= 10;
            let hi = (val % 10) as u8;
            val /= 10;

            bcd[i] = (hi << 4) | lo;
        }

        bcd[9] = if self.is_negative() {
            BCD_SIGN_NEGATIVE
        } else {
            BCD_SIGN_POSITIVE
        };

        bcd
    }

    pub fn from_bcd_packed(&mut self, bcd: &[u8; 10]) {
        let mut value: u128 = 0;

        for i in (0..9).rev() {
            let byte = bcd[i];
            let hi = (byte >> 4) & 0x0F;
            let lo = byte & 0x0F;

            assert!(hi <= 9 && lo <= 9, "Invalid BCD digit");

            value = value * 100 + (hi as u128) * 10 + (lo as u128);
        }

        let is_negative = match bcd[9] & 0x0F {
            BCD_SIGN_NEGATIVE => true,
            BCD_SIGN_POSITIVE => false,
            _ => panic!("Invalid BCD sign"),
        };

        self.st = F80::encode_from_u128(value, is_negative);
    }

    pub fn to_integer_u128(&self) -> u128 {
        let exp = self.get_exponent();
        if exp == 0 || exp == 0x7FFF {
            return 0; // NaN, infinite, or cero
        }

        let bias = 16383;
        let actual_exp = exp as i32 - bias;
        if actual_exp < 0 {
            return 0; // less than 1
        }

        let mantissa = self.get_mantissa_with_integer_bit() as u128;
        if actual_exp > 63 {
            mantissa << (actual_exp as u32 - 63)
        } else {
            mantissa >> (63 - actual_exp) as u32
        }
    }

    pub fn encode_from_u128(value: u128, is_negative: bool) -> u128 {
        if value == 0 {
            return if is_negative { 1u128 << 79 } else { 0 };
        }

        let msb = 127 - value.leading_zeros() as u16;
        let exponent = 16383 + msb;
        let shift = msb as i32 - 63;

        let mantissa = if shift > 0 {
            value >> shift
        } else {
            value << (-shift) as u32
        };

        let sign_bit = if is_negative { 1u128 << 79 } else { 0 };
        let exp_bits = (exponent as u128) << 64;
        let mantissa_bits = mantissa & 0xFFFFFFFFFFFFFFFF;

        sign_bit | exp_bits | mantissa_bits
    }

    pub fn set_f64(&mut self, value: f64) {
        let bits = value.to_bits();
        let sign = (bits >> 63) & 1;
        let exp = ((bits >> 52) & 0x7FF) as u16;
        let mantissa = bits & 0xFFFFFFFFFFFFF;

        let st = if exp == 0 {
            // Subnormal or zero in f64 → represent as 0.0 en FPU
            (sign as u128) << 79
        } else if exp == 0x7FF {
            // Inf o NaN
            let is_nan = mantissa != 0;
            let extended_exp = 0x7FFFu128;
            let extended_mantissa = (mantissa as u128) << 11;
            let nan_bit = if is_nan { 1u128 << 62 } else { 0 }; // QNaN bit (optional)
            ((sign as u128) << 79) | (extended_exp << 64) | extended_mantissa | nan_bit
        } else {
            // Normal number
            let adjusted_exp = (exp as i32 - F64_EXP_BIAS) + F80_EXP_BIAS;
            let extended_exp = adjusted_exp as u128;
            let full_mantissa = ((1u64 << 52) | mantissa) as u128; // add implicit bit
            let extended_mantissa = full_mantissa << 11; // 63 bits align
            ((sign as u128) << 79) | (extended_exp << 64) | extended_mantissa
        };

        self.set(st); // masked setter
    }

    pub fn get_f64(&self) -> f64 {
        let value = self.get();
        let sign = ((value >> 79) & 1) as u64;
        let exp = ((value >> 64) & 0x7FFF) as u16;
        let mantissa = value & 0xFFFFFFFFFFFFFFFF;

        let f64_bits: u64 = if exp == 0 {
            // Zero or denormal extended → 0.0
            sign << 63
        } else if exp == 0x7FFF {
            // Inf or NaN
            let f64_mantissa = (mantissa >> 11) as u64;
            let nan_mask = if f64_mantissa != 0 { 1 << 51 } else { 0 }; // set QNaN bit if needed
            (sign << 63) | (0x7FF << 52) | f64_mantissa | nan_mask
        } else {
            let unbiased_exp = exp as i32 - F80_EXP_BIAS;
            let f64_exp = unbiased_exp + F64_EXP_BIAS;

            if f64_exp <= 0 {
                // Subnormal in f64 → round to 0.0
                sign << 63
            } else if f64_exp >= 0x7FF {
                // Exponent too large → ∞
                (sign << 63) | (0x7FF << 52)
            } else {
                let f64_mantissa = ((mantissa >> 11) & 0xFFFFFFFFFFFFF) as u64;
                (sign << 63) | ((f64_exp as u64) << 52) | f64_mantissa
            }
        };

        f64::from_bits(f64_bits)
    }
}
