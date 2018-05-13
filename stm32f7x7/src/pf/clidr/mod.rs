#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CLIDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CL1R {
    bits: u8,
}
impl CL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CL2R {
    bits: u8,
}
impl CL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CL3R {
    bits: u8,
}
impl CL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CL4R {
    bits: u8,
}
impl CL4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CL5R {
    bits: u8,
}
impl CL5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CL6R {
    bits: u8,
}
impl CL6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CL7R {
    bits: u8,
}
impl CL7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOUISR {
    bits: u8,
}
impl LOUISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOCR {
    bits: u8,
}
impl LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LOUR {
    bits: u8,
}
impl LOUR {
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
    #[doc = "Bits 0:2 - CL1"]
    #[inline]
    pub fn cl1(&self) -> CL1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CL1R { bits }
    }
    #[doc = "Bits 3:5 - CL2"]
    #[inline]
    pub fn cl2(&self) -> CL2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CL2R { bits }
    }
    #[doc = "Bits 6:8 - CL3"]
    #[inline]
    pub fn cl3(&self) -> CL3R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CL3R { bits }
    }
    #[doc = "Bits 9:11 - CL4"]
    #[inline]
    pub fn cl4(&self) -> CL4R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CL4R { bits }
    }
    #[doc = "Bits 12:14 - CL5"]
    #[inline]
    pub fn cl5(&self) -> CL5R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CL5R { bits }
    }
    #[doc = "Bits 15:17 - CL6"]
    #[inline]
    pub fn cl6(&self) -> CL6R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CL6R { bits }
    }
    #[doc = "Bits 18:20 - CL7"]
    #[inline]
    pub fn cl7(&self) -> CL7R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CL7R { bits }
    }
    #[doc = "Bits 21:23 - LoUIS"]
    #[inline]
    pub fn lo_uis(&self) -> LOUISR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOUISR { bits }
    }
    #[doc = "Bits 24:26 - LoC"]
    #[inline]
    pub fn lo_c(&self) -> LOCR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOCR { bits }
    }
    #[doc = "Bits 27:29 - LoU"]
    #[inline]
    pub fn lo_u(&self) -> LOUR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LOUR { bits }
    }
}
