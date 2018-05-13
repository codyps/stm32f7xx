#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DKCFGR2 {
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
pub struct USART1SELR {
    bits: u8,
}
impl USART1SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USART2SELR {
    bits: u8,
}
impl USART2SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USART3SELR {
    bits: u8,
}
impl USART3SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UART4SELR {
    bits: u8,
}
impl UART4SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UART5SELR {
    bits: u8,
}
impl UART5SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USART6SELR {
    bits: u8,
}
impl USART6SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UART7SELR {
    bits: u8,
}
impl UART7SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UART8SELR {
    bits: u8,
}
impl UART8SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct I2C1SELR {
    bits: u8,
}
impl I2C1SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct I2C2SELR {
    bits: u8,
}
impl I2C2SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct I2C3SELR {
    bits: u8,
}
impl I2C3SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct I2C4SELR {
    bits: u8,
}
impl I2C4SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPTIM1SELR {
    bits: u8,
}
impl LPTIM1SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CECSELR {
    bits: bool,
}
impl CECSELR {
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
pub struct CK48MSELR {
    bits: bool,
}
impl CK48MSELR {
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
pub struct SDMMCSELR {
    bits: bool,
}
impl SDMMCSELR {
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
#[doc = r" Proxy"]
pub struct _USART1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1SELW<'a> {
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
#[doc = r" Proxy"]
pub struct _USART2SELW<'a> {
    w: &'a mut W,
}
impl<'a> _USART2SELW<'a> {
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
pub struct _USART3SELW<'a> {
    w: &'a mut W,
}
impl<'a> _USART3SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UART4SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UART4SELW<'a> {
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
pub struct _UART5SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UART5SELW<'a> {
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
pub struct _USART6SELW<'a> {
    w: &'a mut W,
}
impl<'a> _USART6SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UART7SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UART7SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UART8SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UART8SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2C1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1SELW<'a> {
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
pub struct _I2C2SELW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C2SELW<'a> {
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
pub struct _I2C3SELW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C3SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2C4SELW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C4SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPTIM1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LPTIM1SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CECSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CECSELW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CK48MSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CK48MSELW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SDMMCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SDMMCSELW<'a> {
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:1 - USART 1 clock source selection"]
    #[inline]
    pub fn usart1sel(&self) -> USART1SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USART1SELR { bits }
    }
    #[doc = "Bits 2:3 - USART 2 clock source selection"]
    #[inline]
    pub fn usart2sel(&self) -> USART2SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USART2SELR { bits }
    }
    #[doc = "Bits 4:5 - USART 3 clock source selection"]
    #[inline]
    pub fn usart3sel(&self) -> USART3SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USART3SELR { bits }
    }
    #[doc = "Bits 6:7 - UART 4 clock source selection"]
    #[inline]
    pub fn uart4sel(&self) -> UART4SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UART4SELR { bits }
    }
    #[doc = "Bits 8:9 - UART 5 clock source selection"]
    #[inline]
    pub fn uart5sel(&self) -> UART5SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UART5SELR { bits }
    }
    #[doc = "Bits 10:11 - USART 6 clock source selection"]
    #[inline]
    pub fn usart6sel(&self) -> USART6SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USART6SELR { bits }
    }
    #[doc = "Bits 12:13 - UART 7 clock source selection"]
    #[inline]
    pub fn uart7sel(&self) -> UART7SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UART7SELR { bits }
    }
    #[doc = "Bits 14:15 - UART 8 clock source selection"]
    #[inline]
    pub fn uart8sel(&self) -> UART8SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UART8SELR { bits }
    }
    #[doc = "Bits 16:17 - I2C1 clock source selection"]
    #[inline]
    pub fn i2c1sel(&self) -> I2C1SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        I2C1SELR { bits }
    }
    #[doc = "Bits 18:19 - I2C2 clock source selection"]
    #[inline]
    pub fn i2c2sel(&self) -> I2C2SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        I2C2SELR { bits }
    }
    #[doc = "Bits 20:21 - I2C3 clock source selection"]
    #[inline]
    pub fn i2c3sel(&self) -> I2C3SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        I2C3SELR { bits }
    }
    #[doc = "Bits 22:23 - I2C4 clock source selection"]
    #[inline]
    pub fn i2c4sel(&self) -> I2C4SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        I2C4SELR { bits }
    }
    #[doc = "Bits 24:25 - Low power timer 1 clock source selection"]
    #[inline]
    pub fn lptim1sel(&self) -> LPTIM1SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPTIM1SELR { bits }
    }
    #[doc = "Bit 26 - HDMI-CEC clock source selection"]
    #[inline]
    pub fn cecsel(&self) -> CECSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CECSELR { bits }
    }
    #[doc = "Bit 27 - 48MHz clock source selection"]
    #[inline]
    pub fn ck48msel(&self) -> CK48MSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CK48MSELR { bits }
    }
    #[doc = "Bit 28 - SDMMC clock source selection"]
    #[inline]
    pub fn sdmmcsel(&self) -> SDMMCSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDMMCSELR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 536883200 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - USART 1 clock source selection"]
    #[inline]
    pub fn usart1sel(&mut self) -> _USART1SELW {
        _USART1SELW { w: self }
    }
    #[doc = "Bits 2:3 - USART 2 clock source selection"]
    #[inline]
    pub fn usart2sel(&mut self) -> _USART2SELW {
        _USART2SELW { w: self }
    }
    #[doc = "Bits 4:5 - USART 3 clock source selection"]
    #[inline]
    pub fn usart3sel(&mut self) -> _USART3SELW {
        _USART3SELW { w: self }
    }
    #[doc = "Bits 6:7 - UART 4 clock source selection"]
    #[inline]
    pub fn uart4sel(&mut self) -> _UART4SELW {
        _UART4SELW { w: self }
    }
    #[doc = "Bits 8:9 - UART 5 clock source selection"]
    #[inline]
    pub fn uart5sel(&mut self) -> _UART5SELW {
        _UART5SELW { w: self }
    }
    #[doc = "Bits 10:11 - USART 6 clock source selection"]
    #[inline]
    pub fn usart6sel(&mut self) -> _USART6SELW {
        _USART6SELW { w: self }
    }
    #[doc = "Bits 12:13 - UART 7 clock source selection"]
    #[inline]
    pub fn uart7sel(&mut self) -> _UART7SELW {
        _UART7SELW { w: self }
    }
    #[doc = "Bits 14:15 - UART 8 clock source selection"]
    #[inline]
    pub fn uart8sel(&mut self) -> _UART8SELW {
        _UART8SELW { w: self }
    }
    #[doc = "Bits 16:17 - I2C1 clock source selection"]
    #[inline]
    pub fn i2c1sel(&mut self) -> _I2C1SELW {
        _I2C1SELW { w: self }
    }
    #[doc = "Bits 18:19 - I2C2 clock source selection"]
    #[inline]
    pub fn i2c2sel(&mut self) -> _I2C2SELW {
        _I2C2SELW { w: self }
    }
    #[doc = "Bits 20:21 - I2C3 clock source selection"]
    #[inline]
    pub fn i2c3sel(&mut self) -> _I2C3SELW {
        _I2C3SELW { w: self }
    }
    #[doc = "Bits 22:23 - I2C4 clock source selection"]
    #[inline]
    pub fn i2c4sel(&mut self) -> _I2C4SELW {
        _I2C4SELW { w: self }
    }
    #[doc = "Bits 24:25 - Low power timer 1 clock source selection"]
    #[inline]
    pub fn lptim1sel(&mut self) -> _LPTIM1SELW {
        _LPTIM1SELW { w: self }
    }
    #[doc = "Bit 26 - HDMI-CEC clock source selection"]
    #[inline]
    pub fn cecsel(&mut self) -> _CECSELW {
        _CECSELW { w: self }
    }
    #[doc = "Bit 27 - 48MHz clock source selection"]
    #[inline]
    pub fn ck48msel(&mut self) -> _CK48MSELW {
        _CK48MSELW { w: self }
    }
    #[doc = "Bit 28 - SDMMC clock source selection"]
    #[inline]
    pub fn sdmmcsel(&mut self) -> _SDMMCSELW {
        _SDMMCSELW { w: self }
    }
}
