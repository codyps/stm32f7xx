#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - JPEG codec configuration register 0"]
    pub jpeg_confr0: JPEG_CONFR0,
    #[doc = "0x04 - JPEG codec configuration register 1"]
    pub jpeg_confr1: JPEG_CONFR1,
    #[doc = "0x08 - JPEG codec configuration register 2"]
    pub jpeg_confr2: JPEG_CONFR2,
    #[doc = "0x0c - JPEG codec configuration register 3"]
    pub jpeg_confr3: JPEG_CONFR3,
    #[doc = "0x10 - JPEG codec configuration register 4"]
    pub jpeg_confr4: JPEG_CONFR4,
    #[doc = "0x14 - JPEG codec configuration register 5"]
    pub jpeg_confr5: JPEG_CONFR5,
    #[doc = "0x18 - JPEG codec configuration register 6"]
    pub jpeg_confr6: JPEG_CONFR6,
    #[doc = "0x1c - JPEG codec configuration register 7"]
    pub jpeg_confr7: JPEG_CONFR7,
    #[doc = "0x20 - JPEG control register"]
    pub jpeg_cr: JPEG_CR,
    #[doc = "0x24 - JPEG status register"]
    pub jpeg_sr: JPEG_SR,
    #[doc = "0x28 - JPEG clear flag register"]
    pub jpeg_cfr: JPEG_CFR,
    #[doc = "0x2c - JPEG data input register"]
    pub jpeg_dir: JPEG_DIR,
    #[doc = "0x30 - JPEG data output register"]
    pub jpeg_dor: JPEG_DOR,
}
#[doc = "JPEG codec configuration register 0"]
pub struct JPEG_CONFR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JPEG codec configuration register 0"]
pub mod jpeg_confr0;
#[doc = "JPEG codec configuration register 1"]
pub struct JPEG_CONFR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JPEG codec configuration register 1"]
pub mod jpeg_confr1;
#[doc = "JPEG codec configuration register 2"]
pub struct JPEG_CONFR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JPEG codec configuration register 2"]
pub mod jpeg_confr2;
#[doc = "JPEG codec configuration register 3"]
pub struct JPEG_CONFR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JPEG codec configuration register 3"]
pub mod jpeg_confr3;
#[doc = "JPEG codec configuration register 4"]
pub struct JPEG_CONFR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JPEG codec configuration register 4"]
pub mod jpeg_confr4;
#[doc = "JPEG codec configuration register 5"]
pub struct JPEG_CONFR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JPEG codec configuration register 5"]
pub mod jpeg_confr5;
#[doc = "JPEG codec configuration register 6"]
pub struct JPEG_CONFR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JPEG codec configuration register 6"]
pub mod jpeg_confr6;
#[doc = "JPEG codec configuration register 7"]
pub struct JPEG_CONFR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JPEG codec configuration register 7"]
pub mod jpeg_confr7;
#[doc = "JPEG control register"]
pub struct JPEG_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JPEG control register"]
pub mod jpeg_cr;
#[doc = "JPEG status register"]
pub struct JPEG_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JPEG status register"]
pub mod jpeg_sr;
#[doc = "JPEG clear flag register"]
pub struct JPEG_CFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JPEG clear flag register"]
pub mod jpeg_cfr;
#[doc = "JPEG data input register"]
pub struct JPEG_DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JPEG data input register"]
pub mod jpeg_dir;
#[doc = "JPEG data output register"]
pub struct JPEG_DOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "JPEG data output register"]
pub mod jpeg_dor;
