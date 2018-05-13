#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Instruction and Data Tightly-Coupled Memory Control Registers"]
    pub itcmcr: ITCMCR,
    #[doc = "0x04 - Instruction and Data Tightly-Coupled Memory Control Registers"]
    pub dtcmcr: DTCMCR,
    #[doc = "0x08 - AHBP Control register"]
    pub ahbpcr: AHBPCR,
    #[doc = "0x0c - Auxiliary Cache Control register"]
    pub cacr: CACR,
    #[doc = "0x10 - AHB Slave Control register"]
    pub ahbscr: AHBSCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x18 - Auxiliary Bus Fault Status register"]
    pub abfsr: ABFSR,
}
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers"]
pub struct ITCMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers"]
pub mod itcmcr;
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers"]
pub struct DTCMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers"]
pub mod dtcmcr;
#[doc = "AHBP Control register"]
pub struct AHBPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHBP Control register"]
pub mod ahbpcr;
#[doc = "Auxiliary Cache Control register"]
pub struct CACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Cache Control register"]
pub mod cacr;
#[doc = "AHB Slave Control register"]
pub struct AHBSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB Slave Control register"]
pub mod ahbscr;
#[doc = "Auxiliary Bus Fault Status register"]
pub struct ABFSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Bus Fault Status register"]
pub mod abfsr;
