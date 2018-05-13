#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    pub bcr1: BCR1,
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    pub btr1: BTR1,
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    pub bcr2: BCR2,
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2"]
    pub btr2: BTR2,
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 3"]
    pub bcr3: BCR3,
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3"]
    pub btr3: BTR3,
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    pub bcr4: BCR4,
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    pub btr4: BTR4,
    _reserved0: [u8; 96usize],
    #[doc = "0x80 - PC Card/NAND Flash control register"]
    pub pcr: PCR,
    #[doc = "0x84 - FIFO status and interrupt register"]
    pub sr: SR,
    #[doc = "0x88 - Common memory space timing register"]
    pub pmem: PMEM,
    #[doc = "0x8c - Attribute memory space timing register"]
    pub patt: PATT,
    _reserved1: [u8; 4usize],
    #[doc = "0x94 - ECC result register"]
    pub eccr: ECCR,
    _reserved2: [u8; 108usize],
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    pub bwtr1: BWTR1,
    _reserved3: [u8; 4usize],
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"]
    pub bwtr2: BWTR2,
    _reserved4: [u8; 4usize],
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"]
    pub bwtr3: BWTR3,
    _reserved5: [u8; 4usize],
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    pub bwtr4: BWTR4,
    _reserved6: [u8; 32usize],
    #[doc = "0x140 - SDRAM Control Register 1"]
    pub sdcr1: SDCR1,
    #[doc = "0x144 - SDRAM Control Register 2"]
    pub sdcr2: SDCR2,
    #[doc = "0x148 - SDRAM Timing register 1"]
    pub sdtr1: SDTR1,
    #[doc = "0x14c - SDRAM Timing register 2"]
    pub sdtr2: SDTR2,
    #[doc = "0x150 - SDRAM Command Mode register"]
    pub sdcmr: SDCMR,
    #[doc = "0x154 - SDRAM Refresh Timer register"]
    pub sdrtr: SDRTR,
    #[doc = "0x158 - SDRAM Status register"]
    pub sdsr: SDSR,
}
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub struct BCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bcr1;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub struct BTR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod btr1;
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub struct BCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bcr2;
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub struct BTR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub mod btr2;
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub struct BCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub mod bcr3;
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub struct BTR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub mod btr3;
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub struct BCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub mod bcr4;
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub struct BTR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub mod btr4;
#[doc = "PC Card/NAND Flash control register"]
pub struct PCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PC Card/NAND Flash control register"]
pub mod pcr;
#[doc = "FIFO status and interrupt register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO status and interrupt register"]
pub mod sr;
#[doc = "Common memory space timing register"]
pub struct PMEM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Common memory space timing register"]
pub mod pmem;
#[doc = "Attribute memory space timing register"]
pub struct PATT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Attribute memory space timing register"]
pub mod patt;
#[doc = "ECC result register"]
pub struct ECCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ECC result register"]
pub mod eccr;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub struct BWTR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bwtr1;
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub struct BWTR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub mod bwtr2;
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub struct BWTR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub mod bwtr3;
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub struct BWTR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bwtr4;
#[doc = "SDRAM Control Register 1"]
pub struct SDCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM Control Register 1"]
pub mod sdcr1;
#[doc = "SDRAM Control Register 2"]
pub struct SDCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM Control Register 2"]
pub mod sdcr2;
#[doc = "SDRAM Timing register 1"]
pub struct SDTR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM Timing register 1"]
pub mod sdtr1;
#[doc = "SDRAM Timing register 2"]
pub struct SDTR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM Timing register 2"]
pub mod sdtr2;
#[doc = "SDRAM Command Mode register"]
pub struct SDCMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM Command Mode register"]
pub mod sdcmr;
#[doc = "SDRAM Refresh Timer register"]
pub struct SDRTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM Refresh Timer register"]
pub mod sdrtr;
#[doc = "SDRAM Status register"]
pub struct SDSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAM Status register"]
pub mod sdsr;
