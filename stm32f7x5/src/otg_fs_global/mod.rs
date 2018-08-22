#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)"]
    pub otg_fs_gotgctl: OTG_FS_GOTGCTL,
    #[doc = "0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)"]
    pub otg_fs_gotgint: OTG_FS_GOTGINT,
    #[doc = "0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
    pub otg_fs_gahbcfg: OTG_FS_GAHBCFG,
    #[doc = "0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
    pub otg_fs_gusbcfg: OTG_FS_GUSBCFG,
    #[doc = "0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)"]
    pub otg_fs_grstctl: OTG_FS_GRSTCTL,
    #[doc = "0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
    pub otg_fs_gintsts: OTG_FS_GINTSTS,
    #[doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
    pub otg_fs_gintmsk: OTG_FS_GINTMSK,
    #[doc = "0x1c - OTG_FS Receive status debug read(Device mode)"]
    pub otg_fs_grxstsr_device: OTG_FS_GRXSTSR_DEVICE,
    #[doc = "0x20 - OTG status read and pop register (Device mode)"]
    pub otg_fs_grxstsp_device: OTG_FS_GRXSTSP_DEVICE,
    #[doc = "0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
    pub otg_fs_grxfsiz: OTG_FS_GRXFSIZ,
    #[doc = "0x28 - OTG_FS Endpoint 0 Transmit FIFO size"]
    pub otg_fs_dieptxf0_device: OTG_FS_DIEPTXF0_DEVICE,
    #[doc = "0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
    pub otg_fs_hnptxsts: OTG_FS_HNPTXSTS,
    #[doc = "0x30 - OTG I2C access register"]
    pub otg_fs_gi2cctl: OTG_FS_GI2CCTL,
    _reserved0: [u8; 4usize],
    #[doc = "0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)"]
    pub otg_fs_gccfg: OTG_FS_GCCFG,
    #[doc = "0x3c - core ID register"]
    pub otg_fs_cid: OTG_FS_CID,
    _reserved1: [u8; 20usize],
    #[doc = "0x54 - OTG core LPM configuration register"]
    pub otg_fs_glpmcfg: OTG_FS_GLPMCFG,
    #[doc = "0x58 - OTG power down register"]
    pub otg_fs_gpwrdn: OTG_FS_GPWRDN,
    _reserved2: [u8; 4usize],
    #[doc = "0x60 - OTG ADP timer, control and status register"]
    pub otg_fs_gadpctl: OTG_FS_GADPCTL,
    _reserved3: [u8; 156usize],
    #[doc = "0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
    pub otg_fs_hptxfsiz: OTG_FS_HPTXFSIZ,
    #[doc = "0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF1)"]
    pub otg_fs_dieptxf1: OTG_FS_DIEPTXF1,
    #[doc = "0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
    pub otg_fs_dieptxf2: OTG_FS_DIEPTXF2,
    #[doc = "0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
    pub otg_fs_dieptxf3: OTG_FS_DIEPTXF3,
    #[doc = "0x110 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
    pub otg_fs_dieptxf4: OTG_FS_DIEPTXF4,
    #[doc = "0x114 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)"]
    pub otg_fs_dieptxf5: OTG_FS_DIEPTXF5,
}
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"]
pub struct OTG_FS_GOTGCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"]
pub mod otg_fs_gotgctl;
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"]
pub struct OTG_FS_GOTGINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"]
pub mod otg_fs_gotgint;
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
pub struct OTG_FS_GAHBCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
pub mod otg_fs_gahbcfg;
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
pub struct OTG_FS_GUSBCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
pub mod otg_fs_gusbcfg;
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"]
pub struct OTG_FS_GRSTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"]
pub mod otg_fs_grstctl;
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
pub struct OTG_FS_GINTSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
pub mod otg_fs_gintsts;
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub struct OTG_FS_GINTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub mod otg_fs_gintmsk;
#[doc = "OTG_FS Receive status debug read(Device mode)"]
pub struct OTG_FS_GRXSTSR_DEVICE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS Receive status debug read(Device mode)"]
pub mod otg_fs_grxstsr_device;
#[doc = "OTG_FS Receive status debug read(Host mode)"]
pub struct OTG_FS_GRXSTSR_HOST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS Receive status debug read(Host mode)"]
pub mod otg_fs_grxstsr_host;
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
pub struct OTG_FS_GRXFSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
pub mod otg_fs_grxfsiz;
#[doc = "OTG_FS Endpoint 0 Transmit FIFO size"]
pub struct OTG_FS_DIEPTXF0_DEVICE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS Endpoint 0 Transmit FIFO size"]
pub mod otg_fs_dieptxf0_device;
#[doc = "OTG_FS Host non-periodic transmit FIFO size register"]
pub struct OTG_FS_HNPTXFSIZ_HOST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS Host non-periodic transmit FIFO size register"]
pub mod otg_fs_hnptxfsiz_host;
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
pub struct OTG_FS_HNPTXSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
pub mod otg_fs_hnptxsts;
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"]
pub struct OTG_FS_GCCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"]
pub mod otg_fs_gccfg;
#[doc = "core ID register"]
pub struct OTG_FS_CID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "core ID register"]
pub mod otg_fs_cid;
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
pub struct OTG_FS_HPTXFSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
pub mod otg_fs_hptxfsiz;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF1)"]
pub struct OTG_FS_DIEPTXF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF1)"]
pub mod otg_fs_dieptxf1;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
pub struct OTG_FS_DIEPTXF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
pub mod otg_fs_dieptxf2;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
pub struct OTG_FS_DIEPTXF3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
pub mod otg_fs_dieptxf3;
#[doc = "OTG status read and pop register (Device mode)"]
pub struct OTG_FS_GRXSTSP_DEVICE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG status read and pop register (Device mode)"]
pub mod otg_fs_grxstsp_device;
#[doc = "OTG status read and pop register (Host mode)"]
pub struct OTG_FS_GRXSTSP_HOST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG status read and pop register (Host mode)"]
pub mod otg_fs_grxstsp_host;
#[doc = "OTG I2C access register"]
pub struct OTG_FS_GI2CCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG I2C access register"]
pub mod otg_fs_gi2cctl;
#[doc = "OTG power down register"]
pub struct OTG_FS_GPWRDN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG power down register"]
pub mod otg_fs_gpwrdn;
#[doc = "OTG ADP timer, control and status register"]
pub struct OTG_FS_GADPCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG ADP timer, control and status register"]
pub mod otg_fs_gadpctl;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
pub struct OTG_FS_DIEPTXF4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
pub mod otg_fs_dieptxf4;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)"]
pub struct OTG_FS_DIEPTXF5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF5)"]
pub mod otg_fs_dieptxf5;
#[doc = "OTG core LPM configuration register"]
pub struct OTG_FS_GLPMCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG core LPM configuration register"]
pub mod otg_fs_glpmcfg;
