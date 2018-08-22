#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - status register"]
    pub sr: SR,
    #[doc = "0x08 - data input register"]
    pub dinr: DINR,
    #[doc = "0x0c - data output register"]
    pub doutr: DOUTR,
    #[doc = "0x10 - key register"]
    pub keyr0: KEYR0,
    #[doc = "0x14 - key register"]
    pub keyr1: KEYR1,
    #[doc = "0x18 - key register"]
    pub keyr2: KEYR2,
    #[doc = "0x1c - key register"]
    pub keyr3: KEYR3,
    #[doc = "0x20 - initialization vector register"]
    pub ivr0: IVR0,
    #[doc = "0x24 - initialization vector register"]
    pub ivr1: IVR1,
    #[doc = "0x28 - initialization vector register"]
    pub ivr2: IVR2,
    #[doc = "0x2c - initialization vector register"]
    pub ivr3: IVR3,
    #[doc = "0x30 - key registers"]
    pub keyr4: KEYR4,
    #[doc = "0x34 - key registers"]
    pub keyr5: KEYR5,
    #[doc = "0x38 - key registers"]
    pub keyr6: KEYR6,
    #[doc = "0x3c - key registers"]
    pub keyr7: KEYR7,
    #[doc = "0x40 - Suspend registers"]
    pub susp0r: SUSP0R,
    #[doc = "0x44 - Suspend registers"]
    pub susp1r: SUSP1R,
    #[doc = "0x48 - Suspend registers"]
    pub susp2r: SUSP2R,
    #[doc = "0x4c - Suspend registers"]
    pub susp3r: SUSP3R,
    #[doc = "0x50 - Suspend registers"]
    pub susp4r: SUSP4R,
    #[doc = "0x54 - Suspend registers"]
    pub susp5r: SUSP5R,
    #[doc = "0x58 - Suspend registers"]
    pub susp6r: SUSP6R,
    #[doc = "0x5c - Suspend registers"]
    pub susp7r: SUSP7R,
}
#[doc = "control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "data input register"]
pub struct DINR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data input register"]
pub mod dinr;
#[doc = "data output register"]
pub struct DOUTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data output register"]
pub mod doutr;
#[doc = "key register"]
pub struct KEYR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register"]
pub mod keyr0;
#[doc = "key register"]
pub struct KEYR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register"]
pub mod keyr1;
#[doc = "key register"]
pub struct KEYR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register"]
pub mod keyr2;
#[doc = "key register"]
pub struct KEYR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register"]
pub mod keyr3;
#[doc = "initialization vector register"]
pub struct IVR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector register"]
pub mod ivr0;
#[doc = "initialization vector register"]
pub struct IVR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector register"]
pub mod ivr1;
#[doc = "initialization vector register"]
pub struct IVR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector register"]
pub mod ivr2;
#[doc = "initialization vector register"]
pub struct IVR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector register"]
pub mod ivr3;
#[doc = "key registers"]
pub struct KEYR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key registers"]
pub mod keyr4;
#[doc = "key registers"]
pub struct KEYR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key registers"]
pub mod keyr5;
#[doc = "key registers"]
pub struct KEYR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key registers"]
pub mod keyr6;
#[doc = "key registers"]
pub struct KEYR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key registers"]
pub mod keyr7;
#[doc = "Suspend registers"]
pub struct SUSP0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend registers"]
pub mod susp0r;
#[doc = "Suspend registers"]
pub struct SUSP1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend registers"]
pub mod susp1r;
#[doc = "Suspend registers"]
pub struct SUSP2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend registers"]
pub mod susp2r;
#[doc = "Suspend registers"]
pub struct SUSP3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend registers"]
pub mod susp3r;
#[doc = "Suspend registers"]
pub struct SUSP4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend registers"]
pub mod susp4r;
#[doc = "Suspend registers"]
pub struct SUSP5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend registers"]
pub mod susp5r;
#[doc = "Suspend registers"]
pub struct SUSP6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend registers"]
pub mod susp6r;
#[doc = "Suspend registers"]
pub struct SUSP7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend registers"]
pub mod susp7r;
