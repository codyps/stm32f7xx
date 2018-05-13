#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DSI_WPCR2 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct LPRXFTR {
    bits: u8,
}
impl LPRXFTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FLPRXLPMR {
    bits: bool,
}
impl FLPRXLPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct HSTXSRCDLR {
    bits: u8,
}
impl HSTXSRCDLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HSTXSRCCLR {
    bits: u8,
}
impl HSTXSRCCLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SDCCR {
    bits: bool,
}
impl SDCCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct LPSRDLR {
    bits: u8,
}
impl LPSRDLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPSRCLR {
    bits: u8,
}
impl LPSRCLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HSTXDLLR {
    bits: u8,
}
impl HSTXDLLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HSTXDCLR {
    bits: u8,
}
impl HSTXDCLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _LPRXFTW<'a> {
    w: &'a mut W,
}
impl<'a> _LPRXFTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FLPRXLPMW<'a> {
    w: &'a mut W,
}
impl<'a> _FLPRXLPMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSTXSRCDLW<'a> {
    w: &'a mut W,
}
impl<'a> _HSTXSRCDLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSTXSRCCLW<'a> {
    w: &'a mut W,
}
impl<'a> _HSTXSRCCLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SDCCW<'a> {
    w: &'a mut W,
}
impl<'a> _SDCCW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPSRDLW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSRDLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPSRCLW<'a> {
    w: &'a mut W,
}
impl<'a> _LPSRCLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSTXDLLW<'a> {
    w: &'a mut W,
}
impl<'a> _HSTXDLLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSTXDCLW<'a> {
    w: &'a mut W,
}
impl<'a> _HSTXDCLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 25:26 - Low-Power RX low-pass Filtering Tuning"]
    #[inline]
    pub fn lprxft(&self) -> LPRXFTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPRXFTR { bits }
    }
    #[doc = "Bit 22 - Forces LP Receiver in Low-Power Mode"]
    #[inline]
    pub fn flprxlpm(&self) -> FLPRXLPMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FLPRXLPMR { bits }
    }
    #[doc = "Bits 18:19 - High-Speed Transmission Slew Rate Control on Data Lanes"]
    #[inline]
    pub fn hstxsrcdl(&self) -> HSTXSRCDLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSTXSRCDLR { bits }
    }
    #[doc = "Bits 16:17 - High-Speed Transmission Slew Rate Control on Clock Lane"]
    #[inline]
    pub fn hstxsrccl(&self) -> HSTXSRCCLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSTXSRCCLR { bits }
    }
    #[doc = "Bit 12 - SDD Control"]
    #[inline]
    pub fn sdcc(&self) -> SDCCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDCCR { bits }
    }
    #[doc = "Bits 8:9 - Low-Power transmission Slew Rate Compensation on Data Lanes"]
    #[inline]
    pub fn lpsrdl(&self) -> LPSRDLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPSRDLR { bits }
    }
    #[doc = "Bits 6:7 - Low-Power transmission Slew Rate Compensation on Clock Lane"]
    #[inline]
    pub fn lpsrcl(&self) -> LPSRCLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPSRCLR { bits }
    }
    #[doc = "Bits 2:3 - High-Speed Transmission Delay on Data Lanes"]
    #[inline]
    pub fn hstxdll(&self) -> HSTXDLLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSTXDLLR { bits }
    }
    #[doc = "Bits 0:1 - High-Speed Transmission Delay on Clock Lane"]
    #[inline]
    pub fn hstxdcl(&self) -> HSTXDCLR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HSTXDCLR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 25:26 - Low-Power RX low-pass Filtering Tuning"]
    #[inline]
    pub fn lprxft(&mut self) -> _LPRXFTW {
        _LPRXFTW { w: self }
    }
    #[doc = "Bit 22 - Forces LP Receiver in Low-Power Mode"]
    #[inline]
    pub fn flprxlpm(&mut self) -> _FLPRXLPMW {
        _FLPRXLPMW { w: self }
    }
    #[doc = "Bits 18:19 - High-Speed Transmission Slew Rate Control on Data Lanes"]
    #[inline]
    pub fn hstxsrcdl(&mut self) -> _HSTXSRCDLW {
        _HSTXSRCDLW { w: self }
    }
    #[doc = "Bits 16:17 - High-Speed Transmission Slew Rate Control on Clock Lane"]
    #[inline]
    pub fn hstxsrccl(&mut self) -> _HSTXSRCCLW {
        _HSTXSRCCLW { w: self }
    }
    #[doc = "Bit 12 - SDD Control"]
    #[inline]
    pub fn sdcc(&mut self) -> _SDCCW {
        _SDCCW { w: self }
    }
    #[doc = "Bits 8:9 - Low-Power transmission Slew Rate Compensation on Data Lanes"]
    #[inline]
    pub fn lpsrdl(&mut self) -> _LPSRDLW {
        _LPSRDLW { w: self }
    }
    #[doc = "Bits 6:7 - Low-Power transmission Slew Rate Compensation on Clock Lane"]
    #[inline]
    pub fn lpsrcl(&mut self) -> _LPSRCLW {
        _LPSRCLW { w: self }
    }
    #[doc = "Bits 2:3 - High-Speed Transmission Delay on Data Lanes"]
    #[inline]
    pub fn hstxdll(&mut self) -> _HSTXDLLW {
        _HSTXDLLW { w: self }
    }
    #[doc = "Bits 0:1 - High-Speed Transmission Delay on Clock Lane"]
    #[inline]
    pub fn hstxdcl(&mut self) -> _HSTXDCLW {
        _HSTXDCLW { w: self }
    }
}
