#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - power control register"]
    pub cr1: CR1,
    #[doc = "0x04 - power control/status register"]
    pub csr1: CSR1,
    #[doc = "0x08 - power control register"]
    pub cr2: CR2,
    #[doc = "0x0c - power control/status register"]
    pub csr2: CSR2,
}
#[doc = "power control register"]
pub struct CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "power control register"]
pub mod cr1;
#[doc = "power control/status register"]
pub struct CSR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "power control/status register"]
pub mod csr1;
#[doc = "power control register"]
pub struct CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "power control register"]
pub mod cr2;
#[doc = "power control/status register"]
pub struct CSR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "power control/status register"]
pub mod csr2;
