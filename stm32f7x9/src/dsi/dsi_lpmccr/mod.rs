#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DSI_LPMCCR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct VLPSIZER {
    bits: u8,
}
impl VLPSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPSIZER {
    bits: u8,
}
impl LPSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - VACT Largest Packet Size"]
    #[inline]
    pub fn vlpsize(&self) -> VLPSIZER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VLPSIZER { bits }
    }
    #[doc = "Bits 16:23 - Largest Packet Size"]
    #[inline]
    pub fn lpsize(&self) -> LPSIZER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPSIZER { bits }
    }
}
