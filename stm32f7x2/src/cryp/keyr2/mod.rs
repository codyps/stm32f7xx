#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::KEYR2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct KEYR2R {
    bits: u32,
}
impl KEYR2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:30 - Data output register"]
    #[inline]
    pub fn keyr2(&self) -> KEYR2R {
        let bits = {
            const MASK: u32 = 2147483647;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        KEYR2R { bits }
    }
}
