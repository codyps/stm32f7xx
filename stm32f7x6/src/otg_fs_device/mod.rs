#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS device configuration register (OTG_FS_DCFG)"]
    pub otg_fs_dcfg: OTG_FS_DCFG,
    #[doc = "0x04 - OTG_FS device control register (OTG_FS_DCTL)"]
    pub otg_fs_dctl: OTG_FS_DCTL,
    #[doc = "0x08 - OTG_FS device status register (OTG_FS_DSTS)"]
    pub otg_fs_dsts: OTG_FS_DSTS,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
    pub otg_fs_diepmsk: OTG_FS_DIEPMSK,
    #[doc = "0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
    pub otg_fs_doepmsk: OTG_FS_DOEPMSK,
    #[doc = "0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
    pub otg_fs_daint: OTG_FS_DAINT,
    #[doc = "0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
    pub otg_fs_daintmsk: OTG_FS_DAINTMSK,
    _reserved1: [u8; 8usize],
    #[doc = "0x28 - OTG_FS device VBUS discharge time register"]
    pub otg_fs_dvbusdis: OTG_FS_DVBUSDIS,
    #[doc = "0x2c - OTG_FS device VBUS pulsing time register"]
    pub otg_fs_dvbuspulse: OTG_FS_DVBUSPULSE,
    _reserved2: [u8; 4usize],
    #[doc = "0x34 - OTG_FS device IN endpoint FIFO empty interrupt mask register"]
    pub otg_fs_diepempmsk: OTG_FS_DIEPEMPMSK,
    _reserved3: [u8; 200usize],
    #[doc = "0x100 - OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
    pub otg_fs_diepctl0: OTG_FS_DIEPCTL0,
    _reserved4: [u8; 4usize],
    #[doc = "0x108 - device endpoint-x interrupt register"]
    pub otg_fs_diepint0: OTG_FS_DIEPINT0,
    _reserved5: [u8; 4usize],
    #[doc = "0x110 - device endpoint-0 transfer size register"]
    pub otg_fs_dieptsiz0: OTG_FS_DIEPTSIZ0,
    _reserved6: [u8; 4usize],
    #[doc = "0x118 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts0: OTG_FS_DTXFSTS0,
    _reserved7: [u8; 4usize],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub otg_fs_diepctl1: OTG_FS_DIEPCTL1,
    _reserved8: [u8; 4usize],
    #[doc = "0x128 - device endpoint-1 interrupt register"]
    pub otg_fs_diepint1: OTG_FS_DIEPINT1,
    _reserved9: [u8; 4usize],
    #[doc = "0x130 - device endpoint-1 transfer size register"]
    pub otg_fs_dieptsiz1: OTG_FS_DIEPTSIZ1,
    _reserved10: [u8; 4usize],
    #[doc = "0x138 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts1: OTG_FS_DTXFSTS1,
    _reserved11: [u8; 4usize],
    #[doc = "0x140 - OTG device endpoint-2 control register"]
    pub otg_fs_diepctl2: OTG_FS_DIEPCTL2,
    _reserved12: [u8; 4usize],
    #[doc = "0x148 - device endpoint-2 interrupt register"]
    pub otg_fs_diepint2: OTG_FS_DIEPINT2,
    _reserved13: [u8; 4usize],
    #[doc = "0x150 - device endpoint-2 transfer size register"]
    pub otg_fs_dieptsiz2: OTG_FS_DIEPTSIZ2,
    _reserved14: [u8; 4usize],
    #[doc = "0x158 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts2: OTG_FS_DTXFSTS2,
    _reserved15: [u8; 4usize],
    #[doc = "0x160 - OTG device endpoint-3 control register"]
    pub otg_fs_diepctl3: OTG_FS_DIEPCTL3,
    _reserved16: [u8; 4usize],
    #[doc = "0x168 - device endpoint-3 interrupt register"]
    pub otg_fs_diepint3: OTG_FS_DIEPINT3,
    _reserved17: [u8; 4usize],
    #[doc = "0x170 - device endpoint-3 transfer size register"]
    pub otg_fs_dieptsiz3: OTG_FS_DIEPTSIZ3,
    _reserved18: [u8; 4usize],
    #[doc = "0x178 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts3: OTG_FS_DTXFSTS3,
    _reserved19: [u8; 4usize],
    #[doc = "0x180 - OTG device endpoint-4 control register"]
    pub otg_fs_diepctl4: OTG_FS_DIEPCTL4,
    _reserved20: [u8; 4usize],
    #[doc = "0x188 - device endpoint-4 interrupt register"]
    pub otg_fs_diepint4: OTG_FS_DIEPINT4,
    _reserved21: [u8; 8usize],
    #[doc = "0x194 - device endpoint-4 transfer size register"]
    pub otg_fs_dieptsiz4: OTG_FS_DIEPTSIZ4,
    _reserved22: [u8; 4usize],
    #[doc = "0x19c - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts4: OTG_FS_DTXFSTS4,
    #[doc = "0x1a0 - OTG device endpoint-5 control register"]
    pub otg_fs_diepctl5: OTG_FS_DIEPCTL5,
    _reserved23: [u8; 4usize],
    #[doc = "0x1a8 - device endpoint-5 interrupt register"]
    pub otg_fs_diepint5: OTG_FS_DIEPINT5,
    _reserved24: [u8; 4usize],
    #[doc = "0x1b0 - device endpoint-5 transfer size register"]
    pub otg_fs_dieptsiz55: OTG_FS_DIEPTSIZ55,
    _reserved25: [u8; 4usize],
    #[doc = "0x1b8 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub otg_fs_dtxfsts55: OTG_FS_DTXFSTS55,
    _reserved26: [u8; 324usize],
    #[doc = "0x300 - device endpoint-0 control register"]
    pub otg_fs_doepctl0: OTG_FS_DOEPCTL0,
    _reserved27: [u8; 4usize],
    #[doc = "0x308 - device endpoint-0 interrupt register"]
    pub otg_fs_doepint0: OTG_FS_DOEPINT0,
    _reserved28: [u8; 4usize],
    #[doc = "0x310 - device OUT endpoint-0 transfer size register"]
    pub otg_fs_doeptsiz0: OTG_FS_DOEPTSIZ0,
    _reserved29: [u8; 12usize],
    #[doc = "0x320 - device endpoint-1 control register"]
    pub otg_fs_doepctl1: OTG_FS_DOEPCTL1,
    _reserved30: [u8; 4usize],
    #[doc = "0x328 - device endpoint-1 interrupt register"]
    pub otg_fs_doepint1: OTG_FS_DOEPINT1,
    _reserved31: [u8; 4usize],
    #[doc = "0x330 - device OUT endpoint-1 transfer size register"]
    pub otg_fs_doeptsiz1: OTG_FS_DOEPTSIZ1,
    _reserved32: [u8; 12usize],
    #[doc = "0x340 - device endpoint-2 control register"]
    pub otg_fs_doepctl2: OTG_FS_DOEPCTL2,
    _reserved33: [u8; 4usize],
    #[doc = "0x348 - device endpoint-2 interrupt register"]
    pub otg_fs_doepint2: OTG_FS_DOEPINT2,
    _reserved34: [u8; 4usize],
    #[doc = "0x350 - device OUT endpoint-2 transfer size register"]
    pub otg_fs_doeptsiz2: OTG_FS_DOEPTSIZ2,
    _reserved35: [u8; 12usize],
    #[doc = "0x360 - device endpoint-3 control register"]
    pub otg_fs_doepctl3: OTG_FS_DOEPCTL3,
    _reserved36: [u8; 4usize],
    #[doc = "0x368 - device endpoint-3 interrupt register"]
    pub otg_fs_doepint3: OTG_FS_DOEPINT3,
    _reserved37: [u8; 4usize],
    #[doc = "0x370 - device OUT endpoint-3 transfer size register"]
    pub otg_fs_doeptsiz3: OTG_FS_DOEPTSIZ3,
    _reserved38: [u8; 4usize],
    #[doc = "0x378 - device endpoint-4 control register"]
    pub otg_fs_doepctl4: OTG_FS_DOEPCTL4,
    _reserved39: [u8; 4usize],
    #[doc = "0x380 - device endpoint-4 interrupt register"]
    pub otg_fs_doepint4: OTG_FS_DOEPINT4,
    _reserved40: [u8; 4usize],
    #[doc = "0x388 - device OUT endpoint-4 transfer size register"]
    pub otg_fs_doeptsiz4: OTG_FS_DOEPTSIZ4,
    _reserved41: [u8; 4usize],
    #[doc = "0x390 - device endpoint-5 control register"]
    pub otg_fs_doepctl5: OTG_FS_DOEPCTL5,
    _reserved42: [u8; 4usize],
    #[doc = "0x398 - device endpoint-5 interrupt register"]
    pub otg_fs_doepint5: OTG_FS_DOEPINT5,
    _reserved43: [u8; 4usize],
    #[doc = "0x3a0 - device OUT endpoint-5 transfer size register"]
    pub otg_fs_doeptsiz5: OTG_FS_DOEPTSIZ5,
}
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)"]
pub struct OTG_FS_DCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)"]
pub mod otg_fs_dcfg;
#[doc = "OTG_FS device control register (OTG_FS_DCTL)"]
pub struct OTG_FS_DCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device control register (OTG_FS_DCTL)"]
pub mod otg_fs_dctl;
#[doc = "OTG_FS device status register (OTG_FS_DSTS)"]
pub struct OTG_FS_DSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device status register (OTG_FS_DSTS)"]
pub mod otg_fs_dsts;
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
pub struct OTG_FS_DIEPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
pub mod otg_fs_diepmsk;
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
pub struct OTG_FS_DOEPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
pub mod otg_fs_doepmsk;
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
pub struct OTG_FS_DAINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
pub mod otg_fs_daint;
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
pub struct OTG_FS_DAINTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
pub mod otg_fs_daintmsk;
#[doc = "OTG_FS device VBUS discharge time register"]
pub struct OTG_FS_DVBUSDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device VBUS discharge time register"]
pub mod otg_fs_dvbusdis;
#[doc = "OTG_FS device VBUS pulsing time register"]
pub struct OTG_FS_DVBUSPULSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device VBUS pulsing time register"]
pub mod otg_fs_dvbuspulse;
#[doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register"]
pub struct OTG_FS_DIEPEMPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register"]
pub mod otg_fs_diepempmsk;
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
pub struct OTG_FS_DIEPCTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
pub mod otg_fs_diepctl0;
#[doc = "OTG device endpoint-1 control register"]
pub struct OTG_FS_DIEPCTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-1 control register"]
pub mod otg_fs_diepctl1;
#[doc = "OTG device endpoint-2 control register"]
pub struct OTG_FS_DIEPCTL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-2 control register"]
pub mod otg_fs_diepctl2;
#[doc = "OTG device endpoint-3 control register"]
pub struct OTG_FS_DIEPCTL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-3 control register"]
pub mod otg_fs_diepctl3;
#[doc = "device endpoint-0 control register"]
pub struct OTG_FS_DOEPCTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-0 control register"]
pub mod otg_fs_doepctl0;
#[doc = "device endpoint-1 control register"]
pub struct OTG_FS_DOEPCTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-1 control register"]
pub mod otg_fs_doepctl1;
#[doc = "device endpoint-2 control register"]
pub struct OTG_FS_DOEPCTL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-2 control register"]
pub mod otg_fs_doepctl2;
#[doc = "device endpoint-3 control register"]
pub struct OTG_FS_DOEPCTL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-3 control register"]
pub mod otg_fs_doepctl3;
#[doc = "device endpoint-x interrupt register"]
pub struct OTG_FS_DIEPINT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-x interrupt register"]
pub mod otg_fs_diepint0;
#[doc = "device endpoint-1 interrupt register"]
pub struct OTG_FS_DIEPINT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-1 interrupt register"]
pub mod otg_fs_diepint1;
#[doc = "device endpoint-2 interrupt register"]
pub struct OTG_FS_DIEPINT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-2 interrupt register"]
pub mod otg_fs_diepint2;
#[doc = "device endpoint-3 interrupt register"]
pub struct OTG_FS_DIEPINT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-3 interrupt register"]
pub mod otg_fs_diepint3;
#[doc = "device endpoint-0 interrupt register"]
pub struct OTG_FS_DOEPINT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-0 interrupt register"]
pub mod otg_fs_doepint0;
#[doc = "device endpoint-1 interrupt register"]
pub struct OTG_FS_DOEPINT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-1 interrupt register"]
pub mod otg_fs_doepint1;
#[doc = "device endpoint-2 interrupt register"]
pub struct OTG_FS_DOEPINT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-2 interrupt register"]
pub mod otg_fs_doepint2;
#[doc = "device endpoint-3 interrupt register"]
pub struct OTG_FS_DOEPINT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-3 interrupt register"]
pub mod otg_fs_doepint3;
#[doc = "device endpoint-0 transfer size register"]
pub struct OTG_FS_DIEPTSIZ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-0 transfer size register"]
pub mod otg_fs_dieptsiz0;
#[doc = "device OUT endpoint-0 transfer size register"]
pub struct OTG_FS_DOEPTSIZ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint-0 transfer size register"]
pub mod otg_fs_doeptsiz0;
#[doc = "device endpoint-1 transfer size register"]
pub struct OTG_FS_DIEPTSIZ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-1 transfer size register"]
pub mod otg_fs_dieptsiz1;
#[doc = "device endpoint-2 transfer size register"]
pub struct OTG_FS_DIEPTSIZ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-2 transfer size register"]
pub mod otg_fs_dieptsiz2;
#[doc = "device endpoint-3 transfer size register"]
pub struct OTG_FS_DIEPTSIZ3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-3 transfer size register"]
pub mod otg_fs_dieptsiz3;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub struct OTG_FS_DTXFSTS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts0;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub struct OTG_FS_DTXFSTS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts1;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub struct OTG_FS_DTXFSTS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts2;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub struct OTG_FS_DTXFSTS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts3;
#[doc = "device OUT endpoint-1 transfer size register"]
pub struct OTG_FS_DOEPTSIZ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint-1 transfer size register"]
pub mod otg_fs_doeptsiz1;
#[doc = "device OUT endpoint-2 transfer size register"]
pub struct OTG_FS_DOEPTSIZ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint-2 transfer size register"]
pub mod otg_fs_doeptsiz2;
#[doc = "device OUT endpoint-3 transfer size register"]
pub struct OTG_FS_DOEPTSIZ3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint-3 transfer size register"]
pub mod otg_fs_doeptsiz3;
#[doc = "OTG device endpoint-4 control register"]
pub struct OTG_FS_DIEPCTL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-4 control register"]
pub mod otg_fs_diepctl4;
#[doc = "device endpoint-4 interrupt register"]
pub struct OTG_FS_DIEPINT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-4 interrupt register"]
pub mod otg_fs_diepint4;
#[doc = "device endpoint-4 transfer size register"]
pub struct OTG_FS_DIEPTSIZ4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-4 transfer size register"]
pub mod otg_fs_dieptsiz4;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub struct OTG_FS_DTXFSTS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts4;
#[doc = "OTG device endpoint-5 control register"]
pub struct OTG_FS_DIEPCTL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-5 control register"]
pub mod otg_fs_diepctl5;
#[doc = "device endpoint-5 interrupt register"]
pub struct OTG_FS_DIEPINT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-5 interrupt register"]
pub mod otg_fs_diepint5;
#[doc = "device endpoint-5 transfer size register"]
pub struct OTG_FS_DIEPTSIZ55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-5 transfer size register"]
pub mod otg_fs_dieptsiz55;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub struct OTG_FS_DTXFSTS55 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod otg_fs_dtxfsts55;
#[doc = "device endpoint-4 control register"]
pub struct OTG_FS_DOEPCTL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-4 control register"]
pub mod otg_fs_doepctl4;
#[doc = "device endpoint-4 interrupt register"]
pub struct OTG_FS_DOEPINT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-4 interrupt register"]
pub mod otg_fs_doepint4;
#[doc = "device OUT endpoint-4 transfer size register"]
pub struct OTG_FS_DOEPTSIZ4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint-4 transfer size register"]
pub mod otg_fs_doeptsiz4;
#[doc = "device endpoint-5 control register"]
pub struct OTG_FS_DOEPCTL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-5 control register"]
pub mod otg_fs_doepctl5;
#[doc = "device endpoint-5 interrupt register"]
pub struct OTG_FS_DOEPINT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-5 interrupt register"]
pub mod otg_fs_doepint5;
#[doc = "device OUT endpoint-5 transfer size register"]
pub struct OTG_FS_DOEPTSIZ5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint-5 transfer size register"]
pub mod otg_fs_doeptsiz5;
