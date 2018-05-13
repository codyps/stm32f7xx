#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache Level ID register"]
    pub clidr: CLIDR,
    #[doc = "0x04 - Cache Type register"]
    pub ctr: CTR,
    #[doc = "0x08 - Cache Size ID register"]
    pub ccsidr: CCSIDR,
}
#[doc = "Cache Level ID register"]
pub struct CLIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Level ID register"]
pub mod clidr;
#[doc = "Cache Type register"]
pub struct CTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Type register"]
pub mod ctr;
#[doc = "Cache Size ID register"]
pub struct CCSIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cache Size ID register"]
pub mod ccsidr;
