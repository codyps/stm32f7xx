#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MDIOS configuration register"]
    pub mdios_cr: MDIOS_CR,
    #[doc = "0x04 - MDIOS write flag register"]
    pub mdios_wrfr: MDIOS_WRFR,
    #[doc = "0x08 - MDIOS clear write flag register"]
    pub mdios_cwrfr: MDIOS_CWRFR,
    #[doc = "0x0c - MDIOS read flag register"]
    pub mdios_rdfr: MDIOS_RDFR,
    #[doc = "0x10 - MDIOS clear read flag register"]
    pub mdios_crdfr: MDIOS_CRDFR,
    #[doc = "0x14 - MDIOS status register"]
    pub mdios_sr: MDIOS_SR,
    #[doc = "0x18 - MDIOS clear flag register"]
    pub mdios_clrfr: MDIOS_CLRFR,
    #[doc = "0x1c - MDIOS input data register 0"]
    pub mdios_dinr0: MDIOS_DINR0,
    #[doc = "0x20 - MDIOS input data register 1"]
    pub mdios_dinr1: MDIOS_DINR1,
    #[doc = "0x24 - MDIOS input data register 2"]
    pub mdios_dinr2: MDIOS_DINR2,
    #[doc = "0x28 - MDIOS input data register 3"]
    pub mdios_dinr3: MDIOS_DINR3,
    #[doc = "0x2c - MDIOS input data register 4"]
    pub mdios_dinr4: MDIOS_DINR4,
    #[doc = "0x30 - MDIOS input data register 5"]
    pub mdios_dinr5: MDIOS_DINR5,
    #[doc = "0x34 - MDIOS input data register 6"]
    pub mdios_dinr6: MDIOS_DINR6,
    #[doc = "0x38 - MDIOS input data register 7"]
    pub mdios_dinr7: MDIOS_DINR7,
    #[doc = "0x3c - MDIOS input data register 8"]
    pub mdios_dinr8: MDIOS_DINR8,
    #[doc = "0x40 - MDIOS input data register 9"]
    pub mdios_dinr9: MDIOS_DINR9,
    #[doc = "0x44 - MDIOS input data register 10"]
    pub mdios_dinr10: MDIOS_DINR10,
    #[doc = "0x48 - MDIOS input data register 11"]
    pub mdios_dinr11: MDIOS_DINR11,
    #[doc = "0x4c - MDIOS input data register 12"]
    pub mdios_dinr12: MDIOS_DINR12,
    #[doc = "0x50 - MDIOS input data register 13"]
    pub mdios_dinr13: MDIOS_DINR13,
    #[doc = "0x54 - MDIOS input data register 14"]
    pub mdios_dinr14: MDIOS_DINR14,
    #[doc = "0x58 - MDIOS input data register 15"]
    pub mdios_dinr15: MDIOS_DINR15,
    #[doc = "0x5c - MDIOS input data register 16"]
    pub mdios_dinr16: MDIOS_DINR16,
    #[doc = "0x60 - MDIOS input data register 17"]
    pub mdios_dinr17: MDIOS_DINR17,
    #[doc = "0x64 - MDIOS input data register 18"]
    pub mdios_dinr18: MDIOS_DINR18,
    #[doc = "0x68 - MDIOS input data register 19"]
    pub mdios_dinr19: MDIOS_DINR19,
    #[doc = "0x6c - MDIOS input data register 20"]
    pub mdios_dinr20: MDIOS_DINR20,
    #[doc = "0x70 - MDIOS input data register 21"]
    pub mdios_dinr21: MDIOS_DINR21,
    #[doc = "0x74 - MDIOS input data register 22"]
    pub mdios_dinr22: MDIOS_DINR22,
    #[doc = "0x78 - MDIOS input data register 23"]
    pub mdios_dinr23: MDIOS_DINR23,
    #[doc = "0x7c - MDIOS input data register 24"]
    pub mdios_dinr24: MDIOS_DINR24,
    #[doc = "0x80 - MDIOS input data register 25"]
    pub mdios_dinr25: MDIOS_DINR25,
    #[doc = "0x84 - MDIOS input data register 26"]
    pub mdios_dinr26: MDIOS_DINR26,
    #[doc = "0x88 - MDIOS input data register 27"]
    pub mdios_dinr27: MDIOS_DINR27,
    #[doc = "0x8c - MDIOS input data register 28"]
    pub mdios_dinr28: MDIOS_DINR28,
    #[doc = "0x90 - MDIOS input data register 29"]
    pub mdios_dinr29: MDIOS_DINR29,
    #[doc = "0x94 - MDIOS input data register 30"]
    pub mdios_dinr30: MDIOS_DINR30,
    #[doc = "0x98 - MDIOS input data register 31"]
    pub mdios_dinr31: MDIOS_DINR31,
    #[doc = "0x9c - MDIOS output data register 0"]
    pub mdios_doutr0: MDIOS_DOUTR0,
    #[doc = "0xa0 - MDIOS output data register 1"]
    pub mdios_doutr1: MDIOS_DOUTR1,
    #[doc = "0xa4 - MDIOS output data register 2"]
    pub mdios_doutr2: MDIOS_DOUTR2,
    #[doc = "0xa8 - MDIOS output data register 3"]
    pub mdios_doutr3: MDIOS_DOUTR3,
    #[doc = "0xac - MDIOS output data register 4"]
    pub mdios_doutr4: MDIOS_DOUTR4,
    #[doc = "0xb0 - MDIOS output data register 5"]
    pub mdios_doutr5: MDIOS_DOUTR5,
    #[doc = "0xb4 - MDIOS output data register 6"]
    pub mdios_doutr6: MDIOS_DOUTR6,
    #[doc = "0xb8 - MDIOS output data register 7"]
    pub mdios_doutr7: MDIOS_DOUTR7,
    #[doc = "0xbc - MDIOS output data register 8"]
    pub mdios_doutr8: MDIOS_DOUTR8,
    #[doc = "0xc0 - MDIOS output data register 9"]
    pub mdios_doutr9: MDIOS_DOUTR9,
    #[doc = "0xc4 - MDIOS output data register 10"]
    pub mdios_doutr10: MDIOS_DOUTR10,
    #[doc = "0xc8 - MDIOS output data register 11"]
    pub mdios_doutr11: MDIOS_DOUTR11,
    #[doc = "0xcc - MDIOS output data register 12"]
    pub mdios_doutr12: MDIOS_DOUTR12,
    #[doc = "0xd0 - MDIOS output data register 13"]
    pub mdios_doutr13: MDIOS_DOUTR13,
    #[doc = "0xd4 - MDIOS output data register 14"]
    pub mdios_doutr14: MDIOS_DOUTR14,
    #[doc = "0xd8 - MDIOS output data register 15"]
    pub mdios_doutr15: MDIOS_DOUTR15,
    #[doc = "0xdc - MDIOS output data register 16"]
    pub mdios_doutr16: MDIOS_DOUTR16,
    #[doc = "0xe0 - MDIOS output data register 17"]
    pub mdios_doutr17: MDIOS_DOUTR17,
    #[doc = "0xe4 - MDIOS output data register 18"]
    pub mdios_doutr18: MDIOS_DOUTR18,
    #[doc = "0xe8 - MDIOS output data register 19"]
    pub mdios_doutr19: MDIOS_DOUTR19,
    #[doc = "0xec - MDIOS output data register 20"]
    pub mdios_doutr20: MDIOS_DOUTR20,
    #[doc = "0xf0 - MDIOS output data register 21"]
    pub mdios_doutr21: MDIOS_DOUTR21,
    #[doc = "0xf4 - MDIOS output data register 22"]
    pub mdios_doutr22: MDIOS_DOUTR22,
    #[doc = "0xf8 - MDIOS output data register 23"]
    pub mdios_doutr23: MDIOS_DOUTR23,
    #[doc = "0xfc - MDIOS output data register 24"]
    pub mdios_doutr24: MDIOS_DOUTR24,
    #[doc = "0x100 - MDIOS output data register 25"]
    pub mdios_doutr25: MDIOS_DOUTR25,
    #[doc = "0x104 - MDIOS output data register 26"]
    pub mdios_doutr26: MDIOS_DOUTR26,
    #[doc = "0x108 - MDIOS output data register 27"]
    pub mdios_doutr27: MDIOS_DOUTR27,
    #[doc = "0x10c - MDIOS output data register 28"]
    pub mdios_doutr28: MDIOS_DOUTR28,
    #[doc = "0x110 - MDIOS output data register 29"]
    pub mdios_doutr29: MDIOS_DOUTR29,
    #[doc = "0x114 - MDIOS output data register 30"]
    pub mdios_doutr30: MDIOS_DOUTR30,
    #[doc = "0x118 - MDIOS output data register 31"]
    pub mdios_doutr31: MDIOS_DOUTR31,
}
#[doc = "MDIOS configuration register"]
pub struct MDIOS_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS configuration register"]
pub mod mdios_cr;
#[doc = "MDIOS write flag register"]
pub struct MDIOS_WRFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS write flag register"]
pub mod mdios_wrfr;
#[doc = "MDIOS clear write flag register"]
pub struct MDIOS_CWRFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS clear write flag register"]
pub mod mdios_cwrfr;
#[doc = "MDIOS read flag register"]
pub struct MDIOS_RDFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS read flag register"]
pub mod mdios_rdfr;
#[doc = "MDIOS clear read flag register"]
pub struct MDIOS_CRDFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS clear read flag register"]
pub mod mdios_crdfr;
#[doc = "MDIOS status register"]
pub struct MDIOS_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS status register"]
pub mod mdios_sr;
#[doc = "MDIOS clear flag register"]
pub struct MDIOS_CLRFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS clear flag register"]
pub mod mdios_clrfr;
#[doc = "MDIOS input data register 0"]
pub struct MDIOS_DINR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 0"]
pub mod mdios_dinr0;
#[doc = "MDIOS input data register 1"]
pub struct MDIOS_DINR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 1"]
pub mod mdios_dinr1;
#[doc = "MDIOS input data register 2"]
pub struct MDIOS_DINR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 2"]
pub mod mdios_dinr2;
#[doc = "MDIOS input data register 3"]
pub struct MDIOS_DINR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 3"]
pub mod mdios_dinr3;
#[doc = "MDIOS input data register 4"]
pub struct MDIOS_DINR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 4"]
pub mod mdios_dinr4;
#[doc = "MDIOS input data register 5"]
pub struct MDIOS_DINR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 5"]
pub mod mdios_dinr5;
#[doc = "MDIOS input data register 6"]
pub struct MDIOS_DINR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 6"]
pub mod mdios_dinr6;
#[doc = "MDIOS input data register 7"]
pub struct MDIOS_DINR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 7"]
pub mod mdios_dinr7;
#[doc = "MDIOS input data register 8"]
pub struct MDIOS_DINR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 8"]
pub mod mdios_dinr8;
#[doc = "MDIOS input data register 9"]
pub struct MDIOS_DINR9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 9"]
pub mod mdios_dinr9;
#[doc = "MDIOS input data register 10"]
pub struct MDIOS_DINR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 10"]
pub mod mdios_dinr10;
#[doc = "MDIOS input data register 11"]
pub struct MDIOS_DINR11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 11"]
pub mod mdios_dinr11;
#[doc = "MDIOS input data register 12"]
pub struct MDIOS_DINR12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 12"]
pub mod mdios_dinr12;
#[doc = "MDIOS input data register 13"]
pub struct MDIOS_DINR13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 13"]
pub mod mdios_dinr13;
#[doc = "MDIOS input data register 14"]
pub struct MDIOS_DINR14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 14"]
pub mod mdios_dinr14;
#[doc = "MDIOS input data register 15"]
pub struct MDIOS_DINR15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 15"]
pub mod mdios_dinr15;
#[doc = "MDIOS input data register 16"]
pub struct MDIOS_DINR16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 16"]
pub mod mdios_dinr16;
#[doc = "MDIOS input data register 17"]
pub struct MDIOS_DINR17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 17"]
pub mod mdios_dinr17;
#[doc = "MDIOS input data register 18"]
pub struct MDIOS_DINR18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 18"]
pub mod mdios_dinr18;
#[doc = "MDIOS input data register 19"]
pub struct MDIOS_DINR19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 19"]
pub mod mdios_dinr19;
#[doc = "MDIOS input data register 20"]
pub struct MDIOS_DINR20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 20"]
pub mod mdios_dinr20;
#[doc = "MDIOS input data register 21"]
pub struct MDIOS_DINR21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 21"]
pub mod mdios_dinr21;
#[doc = "MDIOS input data register 22"]
pub struct MDIOS_DINR22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 22"]
pub mod mdios_dinr22;
#[doc = "MDIOS input data register 23"]
pub struct MDIOS_DINR23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 23"]
pub mod mdios_dinr23;
#[doc = "MDIOS input data register 24"]
pub struct MDIOS_DINR24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 24"]
pub mod mdios_dinr24;
#[doc = "MDIOS input data register 25"]
pub struct MDIOS_DINR25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 25"]
pub mod mdios_dinr25;
#[doc = "MDIOS input data register 26"]
pub struct MDIOS_DINR26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 26"]
pub mod mdios_dinr26;
#[doc = "MDIOS input data register 27"]
pub struct MDIOS_DINR27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 27"]
pub mod mdios_dinr27;
#[doc = "MDIOS input data register 28"]
pub struct MDIOS_DINR28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 28"]
pub mod mdios_dinr28;
#[doc = "MDIOS input data register 29"]
pub struct MDIOS_DINR29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 29"]
pub mod mdios_dinr29;
#[doc = "MDIOS input data register 30"]
pub struct MDIOS_DINR30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 30"]
pub mod mdios_dinr30;
#[doc = "MDIOS input data register 31"]
pub struct MDIOS_DINR31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS input data register 31"]
pub mod mdios_dinr31;
#[doc = "MDIOS output data register 0"]
pub struct MDIOS_DOUTR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 0"]
pub mod mdios_doutr0;
#[doc = "MDIOS output data register 1"]
pub struct MDIOS_DOUTR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 1"]
pub mod mdios_doutr1;
#[doc = "MDIOS output data register 2"]
pub struct MDIOS_DOUTR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 2"]
pub mod mdios_doutr2;
#[doc = "MDIOS output data register 3"]
pub struct MDIOS_DOUTR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 3"]
pub mod mdios_doutr3;
#[doc = "MDIOS output data register 4"]
pub struct MDIOS_DOUTR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 4"]
pub mod mdios_doutr4;
#[doc = "MDIOS output data register 5"]
pub struct MDIOS_DOUTR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 5"]
pub mod mdios_doutr5;
#[doc = "MDIOS output data register 6"]
pub struct MDIOS_DOUTR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 6"]
pub mod mdios_doutr6;
#[doc = "MDIOS output data register 7"]
pub struct MDIOS_DOUTR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 7"]
pub mod mdios_doutr7;
#[doc = "MDIOS output data register 8"]
pub struct MDIOS_DOUTR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 8"]
pub mod mdios_doutr8;
#[doc = "MDIOS output data register 9"]
pub struct MDIOS_DOUTR9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 9"]
pub mod mdios_doutr9;
#[doc = "MDIOS output data register 10"]
pub struct MDIOS_DOUTR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 10"]
pub mod mdios_doutr10;
#[doc = "MDIOS output data register 11"]
pub struct MDIOS_DOUTR11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 11"]
pub mod mdios_doutr11;
#[doc = "MDIOS output data register 12"]
pub struct MDIOS_DOUTR12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 12"]
pub mod mdios_doutr12;
#[doc = "MDIOS output data register 13"]
pub struct MDIOS_DOUTR13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 13"]
pub mod mdios_doutr13;
#[doc = "MDIOS output data register 14"]
pub struct MDIOS_DOUTR14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 14"]
pub mod mdios_doutr14;
#[doc = "MDIOS output data register 15"]
pub struct MDIOS_DOUTR15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 15"]
pub mod mdios_doutr15;
#[doc = "MDIOS output data register 16"]
pub struct MDIOS_DOUTR16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 16"]
pub mod mdios_doutr16;
#[doc = "MDIOS output data register 17"]
pub struct MDIOS_DOUTR17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 17"]
pub mod mdios_doutr17;
#[doc = "MDIOS output data register 18"]
pub struct MDIOS_DOUTR18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 18"]
pub mod mdios_doutr18;
#[doc = "MDIOS output data register 19"]
pub struct MDIOS_DOUTR19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 19"]
pub mod mdios_doutr19;
#[doc = "MDIOS output data register 20"]
pub struct MDIOS_DOUTR20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 20"]
pub mod mdios_doutr20;
#[doc = "MDIOS output data register 21"]
pub struct MDIOS_DOUTR21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 21"]
pub mod mdios_doutr21;
#[doc = "MDIOS output data register 22"]
pub struct MDIOS_DOUTR22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 22"]
pub mod mdios_doutr22;
#[doc = "MDIOS output data register 23"]
pub struct MDIOS_DOUTR23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 23"]
pub mod mdios_doutr23;
#[doc = "MDIOS output data register 24"]
pub struct MDIOS_DOUTR24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 24"]
pub mod mdios_doutr24;
#[doc = "MDIOS output data register 25"]
pub struct MDIOS_DOUTR25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 25"]
pub mod mdios_doutr25;
#[doc = "MDIOS output data register 26"]
pub struct MDIOS_DOUTR26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 26"]
pub mod mdios_doutr26;
#[doc = "MDIOS output data register 27"]
pub struct MDIOS_DOUTR27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 27"]
pub mod mdios_doutr27;
#[doc = "MDIOS output data register 28"]
pub struct MDIOS_DOUTR28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 28"]
pub mod mdios_doutr28;
#[doc = "MDIOS output data register 29"]
pub struct MDIOS_DOUTR29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 29"]
pub mod mdios_doutr29;
#[doc = "MDIOS output data register 30"]
pub struct MDIOS_DOUTR30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 30"]
pub mod mdios_doutr30;
#[doc = "MDIOS output data register 31"]
pub struct MDIOS_DOUTR31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MDIOS output data register 31"]
pub mod mdios_doutr31;
