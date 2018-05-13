#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub cr: CR,
    #[doc = "0x04 - Interrupt mask register"]
    pub imr: IMR,
    #[doc = "0x08 - Status register"]
    pub sr: SR,
    #[doc = "0x0c - Interrupt Flag Clear register"]
    pub ifcr: IFCR,
    #[doc = "0x10 - Data input register"]
    pub dr: DR,
    #[doc = "0x14 - Channel Status register"]
    pub csr: CSR,
    #[doc = "0x18 - Debug Information register"]
    pub dir: DIR,
}
#[doc = "Control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod cr;
#[doc = "Interrupt mask register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt mask register"]
pub mod imr;
#[doc = "Status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod sr;
#[doc = "Interrupt Flag Clear register"]
pub struct IFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flag Clear register"]
pub mod ifcr;
#[doc = "Data input register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data input register"]
pub mod dr;
#[doc = "Channel Status register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Status register"]
pub mod csr;
#[doc = "Debug Information register"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug Information register"]
pub mod dir;
