#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CTR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct _IMINLINER {
    bits: u8,
}
impl _IMINLINER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMINLINER {
    bits: u8,
}
impl DMINLINER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ERGR {
    bits: u8,
}
impl ERGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CWGR {
    bits: u8,
}
impl CWGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FORMATR {
    bits: u8,
}
impl FORMATR {
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
    #[doc = "Bits 0:3 - IminLine"]
    #[inline]
    pub fn _imin_line(&self) -> _IMINLINER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        _IMINLINER { bits }
    }
    #[doc = "Bits 16:19 - DMinLine"]
    #[inline]
    pub fn dmin_line(&self) -> DMINLINER {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMINLINER { bits }
    }
    #[doc = "Bits 20:23 - ERG"]
    #[inline]
    pub fn erg(&self) -> ERGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ERGR { bits }
    }
    #[doc = "Bits 24:27 - CWG"]
    #[inline]
    pub fn cwg(&self) -> CWGR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CWGR { bits }
    }
    #[doc = "Bits 29:31 - Format"]
    #[inline]
    pub fn format(&self) -> FORMATR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FORMATR { bits }
    }
}
