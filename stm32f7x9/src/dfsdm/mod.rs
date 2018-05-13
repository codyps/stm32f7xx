#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DFSDM channel configuration 0 register 1"]
    pub dfsdm_chcfg0r1: DFSDM_CHCFG0R1,
    #[doc = "0x04 - DFSDM channel configuration 1 register 1"]
    pub dfsdm_chcfg1r1: DFSDM_CHCFG1R1,
    #[doc = "0x08 - DFSDM channel configuration 2 register 1"]
    pub dfsdm_chcfg2r1: DFSDM_CHCFG2R1,
    #[doc = "0x0c - DFSDM channel configuration 3 register 1"]
    pub dfsdm_chcfg3r1: DFSDM_CHCFG3R1,
    #[doc = "0x10 - DFSDM channel configuration 4 register 1"]
    pub dfsdm_chcfg4r1: DFSDM_CHCFG4R1,
    #[doc = "0x14 - DFSDM channel configuration 5 register 1"]
    pub dfsdm_chcfg5r1: DFSDM_CHCFG5R1,
    #[doc = "0x18 - DFSDM channel configuration 6 register 1"]
    pub dfsdm_chcfg6r1: DFSDM_CHCFG6R1,
    #[doc = "0x1c - DFSDM channel configuration 7 register 1"]
    pub dfsdm_chcfg7r1: DFSDM_CHCFG7R1,
    #[doc = "0x20 - DFSDM channel configuration 0 register 2"]
    pub dfsdm_chcfg0r2: DFSDM_CHCFG0R2,
    #[doc = "0x24 - DFSDM channel configuration 1 register 2"]
    pub dfsdm_chcfg1r2: DFSDM_CHCFG1R2,
    #[doc = "0x28 - DFSDM channel configuration 2 register 2"]
    pub dfsdm_chcfg2r2: DFSDM_CHCFG2R2,
    #[doc = "0x2c - DFSDM channel configuration 3 register 2"]
    pub dfsdm_chcfg3r2: DFSDM_CHCFG3R2,
    #[doc = "0x30 - DFSDM channel configuration 4 register 2"]
    pub dfsdm_chcfg4r2: DFSDM_CHCFG4R2,
    #[doc = "0x34 - DFSDM channel configuration 5 register 2"]
    pub dfsdm_chcfg5r2: DFSDM_CHCFG5R2,
    #[doc = "0x38 - DFSDM channel configuration 6 register 2"]
    pub dfsdm_chcfg6r2: DFSDM_CHCFG6R2,
    #[doc = "0x3c - DFSDM channel configuration 7 register 2"]
    pub dfsdm_chcfg7r2: DFSDM_CHCFG7R2,
    #[doc = "0x40 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd0r: DFSDM_AWSCD0R,
    #[doc = "0x44 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd1r: DFSDM_AWSCD1R,
    #[doc = "0x48 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd2r: DFSDM_AWSCD2R,
    #[doc = "0x4c - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd3r: DFSDM_AWSCD3R,
    #[doc = "0x50 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd4r: DFSDM_AWSCD4R,
    #[doc = "0x54 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd5r: DFSDM_AWSCD5R,
    #[doc = "0x58 - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd6r: DFSDM_AWSCD6R,
    #[doc = "0x5c - DFSDM analog watchdog and short-circuit detector register"]
    pub dfsdm_awscd7r: DFSDM_AWSCD7R,
    #[doc = "0x60 - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat0r: DFSDM_CHWDAT0R,
    #[doc = "0x64 - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat1r: DFSDM_CHWDAT1R,
    #[doc = "0x68 - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat2r: DFSDM_CHWDAT2R,
    #[doc = "0x6c - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat3r: DFSDM_CHWDAT3R,
    #[doc = "0x70 - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat4r: DFSDM_CHWDAT4R,
    #[doc = "0x74 - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat5r: DFSDM_CHWDAT5R,
    #[doc = "0x78 - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat6r: DFSDM_CHWDAT6R,
    #[doc = "0x7c - DFSDM channel watchdog filter data register"]
    pub dfsdm_chwdat7r: DFSDM_CHWDAT7R,
    #[doc = "0x80 - DFSDM channel data input register"]
    pub dfsdm_chdatin0r: DFSDM_CHDATIN0R,
    #[doc = "0x84 - DFSDM channel data input register"]
    pub dfsdm_chdatin1r: DFSDM_CHDATIN1R,
    #[doc = "0x88 - DFSDM channel data input register"]
    pub dfsdm_chdatin2r: DFSDM_CHDATIN2R,
    #[doc = "0x8c - DFSDM channel data input register"]
    pub dfsdm_chdatin3r: DFSDM_CHDATIN3R,
    #[doc = "0x90 - DFSDM channel data input register"]
    pub dfsdm_chdatin4r: DFSDM_CHDATIN4R,
    #[doc = "0x94 - DFSDM channel data input register"]
    pub dfsdm_chdatin5r: DFSDM_CHDATIN5R,
    #[doc = "0x98 - DFSDM channel data input register"]
    pub dfsdm_chdatin6r: DFSDM_CHDATIN6R,
    #[doc = "0x9c - DFSDM channel data input register"]
    pub dfsdm_chdatin7r: DFSDM_CHDATIN7R,
    #[doc = "0xa0 - DFSDM control register 1"]
    pub dfsdm0_cr1: DFSDM0_CR1,
    #[doc = "0xa4 - DFSDM control register 1"]
    pub dfsdm1_cr1: DFSDM1_CR1,
    #[doc = "0xa8 - DFSDM control register 1"]
    pub dfsdm2_cr1: DFSDM2_CR1,
    #[doc = "0xac - DFSDM control register 1"]
    pub dfsdm3_cr1: DFSDM3_CR1,
    #[doc = "0xb0 - DFSDM control register 2"]
    pub dfsdm0_cr2: DFSDM0_CR2,
    #[doc = "0xb4 - DFSDM control register 2"]
    pub dfsdm1_cr2: DFSDM1_CR2,
    #[doc = "0xb8 - DFSDM control register 2"]
    pub dfsdm2_cr2: DFSDM2_CR2,
    #[doc = "0xbc - DFSDM control register 2"]
    pub dfsdm3_cr2: DFSDM3_CR2,
    #[doc = "0xc0 - DFSDM interrupt and status register"]
    pub dfsdm0_isr: DFSDM0_ISR,
    #[doc = "0xc4 - DFSDM interrupt and status register"]
    pub dfsdm1_isr: DFSDM1_ISR,
    #[doc = "0xc8 - DFSDM interrupt and status register"]
    pub dfsdm2_isr: DFSDM2_ISR,
    #[doc = "0xcc - DFSDM interrupt and status register"]
    pub dfsdm3_isr: DFSDM3_ISR,
    #[doc = "0xd0 - DFSDM interrupt flag clear register"]
    pub dfsdm0_icr: DFSDM0_ICR,
    #[doc = "0xd4 - DFSDM interrupt flag clear register"]
    pub dfsdm1_icr: DFSDM1_ICR,
    #[doc = "0xd8 - DFSDM interrupt flag clear register"]
    pub dfsdm2_icr: DFSDM2_ICR,
    #[doc = "0xdc - DFSDM interrupt flag clear register"]
    pub dfsdm3_icr: DFSDM3_ICR,
    #[doc = "0xe0 - DFSDM injected channel group selection register"]
    pub dfsdm0_jchgr: DFSDM0_JCHGR,
    #[doc = "0xe4 - DFSDM injected channel group selection register"]
    pub dfsdm1_jchgr: DFSDM1_JCHGR,
    #[doc = "0xe8 - DFSDM injected channel group selection register"]
    pub dfsdm2_jchgr: DFSDM2_JCHGR,
    #[doc = "0xec - DFSDM injected channel group selection register"]
    pub dfsdm3_jchgr: DFSDM3_JCHGR,
    #[doc = "0xf0 - DFSDM filter control register"]
    pub dfsdm0_fcr: DFSDM0_FCR,
    #[doc = "0xf4 - DFSDM filter control register"]
    pub dfsdm1_fcr: DFSDM1_FCR,
    #[doc = "0xf8 - DFSDM filter control register"]
    pub dfsdm2_fcr: DFSDM2_FCR,
    #[doc = "0xfc - DFSDM filter control register"]
    pub dfsdm3_fcr: DFSDM3_FCR,
    #[doc = "0x100 - DFSDM data register for injected group"]
    pub dfsdm0_jdatar: DFSDM0_JDATAR,
    #[doc = "0x104 - DFSDM data register for injected group"]
    pub dfsdm1_jdatar: DFSDM1_JDATAR,
    #[doc = "0x108 - DFSDM data register for injected group"]
    pub dfsdm2_jdatar: DFSDM2_JDATAR,
    #[doc = "0x10c - DFSDM data register for injected group"]
    pub dfsdm3_jdatar: DFSDM3_JDATAR,
    #[doc = "0x110 - DFSDM data register for the regular channel"]
    pub dfsdm0_rdatar: DFSDM0_RDATAR,
    #[doc = "0x114 - DFSDM data register for the regular channel"]
    pub dfsdm1_rdatar: DFSDM1_RDATAR,
    #[doc = "0x118 - DFSDM data register for the regular channel"]
    pub dfsdm2_rdatar: DFSDM2_RDATAR,
    #[doc = "0x11c - DFSDM data register for the regular channel"]
    pub dfsdm3_rdatar: DFSDM3_RDATAR,
    #[doc = "0x120 - DFSDM analog watchdog high threshold register"]
    pub dfsdm0_awhtr: DFSDM0_AWHTR,
    #[doc = "0x124 - DFSDM analog watchdog high threshold register"]
    pub dfsdm1_awhtr: DFSDM1_AWHTR,
    #[doc = "0x128 - DFSDM analog watchdog high threshold register"]
    pub dfsdm2_awhtr: DFSDM2_AWHTR,
    #[doc = "0x12c - DFSDM analog watchdog high threshold register"]
    pub dfsdm3_awhtr: DFSDM3_AWHTR,
    #[doc = "0x130 - DFSDM analog watchdog low threshold register"]
    pub dfsdm0_awltr: DFSDM0_AWLTR,
    #[doc = "0x134 - DFSDM analog watchdog low threshold register"]
    pub dfsdm1_awltr: DFSDM1_AWLTR,
    #[doc = "0x138 - DFSDM analog watchdog low threshold register"]
    pub dfsdm2_awltr: DFSDM2_AWLTR,
    #[doc = "0x13c - DFSDM analog watchdog low threshold register"]
    pub dfsdm3_awltr: DFSDM3_AWLTR,
    #[doc = "0x140 - DFSDM analog watchdog status register"]
    pub dfsdm0_awsr: DFSDM0_AWSR,
    #[doc = "0x144 - DFSDM analog watchdog status register"]
    pub dfsdm1_awsr: DFSDM1_AWSR,
    #[doc = "0x148 - DFSDM analog watchdog status register"]
    pub dfsdm2_awsr: DFSDM2_AWSR,
    #[doc = "0x14c - DFSDM analog watchdog status register"]
    pub dfsdm3_awsr: DFSDM3_AWSR,
    #[doc = "0x150 - DFSDM analog watchdog clear flag register"]
    pub dfsdm0_awcfr: DFSDM0_AWCFR,
    #[doc = "0x154 - DFSDM analog watchdog clear flag register"]
    pub dfsdm1_awcfr: DFSDM1_AWCFR,
    #[doc = "0x158 - DFSDM analog watchdog clear flag register"]
    pub dfsdm2_awcfr: DFSDM2_AWCFR,
    #[doc = "0x15c - DFSDM analog watchdog clear flag register"]
    pub dfsdm3_awcfr: DFSDM3_AWCFR,
    #[doc = "0x160 - DFSDM Extremes detector maximum register"]
    pub dfsdm0_exmax: DFSDM0_EXMAX,
    #[doc = "0x164 - DFSDM Extremes detector maximum register"]
    pub dfsdm1_exmax: DFSDM1_EXMAX,
    #[doc = "0x168 - DFSDM Extremes detector maximum register"]
    pub dfsdm2_exmax: DFSDM2_EXMAX,
    #[doc = "0x16c - DFSDM Extremes detector maximum register"]
    pub dfsdm3_exmax: DFSDM3_EXMAX,
    #[doc = "0x170 - DFSDM Extremes detector minimum register"]
    pub dfsdm0_exmin: DFSDM0_EXMIN,
    #[doc = "0x174 - DFSDM Extremes detector minimum register"]
    pub dfsdm1_exmin: DFSDM1_EXMIN,
    #[doc = "0x178 - DFSDM Extremes detector minimum register"]
    pub dfsdm2_exmin: DFSDM2_EXMIN,
    #[doc = "0x17c - DFSDM Extremes detector minimum register"]
    pub dfsdm3_exmin: DFSDM3_EXMIN,
    #[doc = "0x180 - DFSDM conversion timer register"]
    pub dfsdm0_cnvtimr: DFSDM0_CNVTIMR,
    #[doc = "0x184 - DFSDM conversion timer register"]
    pub dfsdm1_cnvtimr: DFSDM1_CNVTIMR,
    #[doc = "0x188 - DFSDM conversion timer register"]
    pub dfsdm2_cnvtimr: DFSDM2_CNVTIMR,
    #[doc = "0x18c - DFSDM conversion timer register"]
    pub dfsdm3_cnvtimr: DFSDM3_CNVTIMR,
}
#[doc = "DFSDM channel configuration 0 register 1"]
pub struct DFSDM_CHCFG0R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 0 register 1"]
pub mod dfsdm_chcfg0r1;
#[doc = "DFSDM channel configuration 1 register 1"]
pub struct DFSDM_CHCFG1R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 1 register 1"]
pub mod dfsdm_chcfg1r1;
#[doc = "DFSDM channel configuration 2 register 1"]
pub struct DFSDM_CHCFG2R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 2 register 1"]
pub mod dfsdm_chcfg2r1;
#[doc = "DFSDM channel configuration 3 register 1"]
pub struct DFSDM_CHCFG3R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 3 register 1"]
pub mod dfsdm_chcfg3r1;
#[doc = "DFSDM channel configuration 4 register 1"]
pub struct DFSDM_CHCFG4R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 4 register 1"]
pub mod dfsdm_chcfg4r1;
#[doc = "DFSDM channel configuration 5 register 1"]
pub struct DFSDM_CHCFG5R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 5 register 1"]
pub mod dfsdm_chcfg5r1;
#[doc = "DFSDM channel configuration 6 register 1"]
pub struct DFSDM_CHCFG6R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 6 register 1"]
pub mod dfsdm_chcfg6r1;
#[doc = "DFSDM channel configuration 7 register 1"]
pub struct DFSDM_CHCFG7R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 7 register 1"]
pub mod dfsdm_chcfg7r1;
#[doc = "DFSDM channel configuration 0 register 2"]
pub struct DFSDM_CHCFG0R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 0 register 2"]
pub mod dfsdm_chcfg0r2;
#[doc = "DFSDM channel configuration 1 register 2"]
pub struct DFSDM_CHCFG1R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 1 register 2"]
pub mod dfsdm_chcfg1r2;
#[doc = "DFSDM channel configuration 2 register 2"]
pub struct DFSDM_CHCFG2R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 2 register 2"]
pub mod dfsdm_chcfg2r2;
#[doc = "DFSDM channel configuration 3 register 2"]
pub struct DFSDM_CHCFG3R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 3 register 2"]
pub mod dfsdm_chcfg3r2;
#[doc = "DFSDM channel configuration 4 register 2"]
pub struct DFSDM_CHCFG4R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 4 register 2"]
pub mod dfsdm_chcfg4r2;
#[doc = "DFSDM channel configuration 5 register 2"]
pub struct DFSDM_CHCFG5R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 5 register 2"]
pub mod dfsdm_chcfg5r2;
#[doc = "DFSDM channel configuration 6 register 2"]
pub struct DFSDM_CHCFG6R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 6 register 2"]
pub mod dfsdm_chcfg6r2;
#[doc = "DFSDM channel configuration 7 register 2"]
pub struct DFSDM_CHCFG7R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel configuration 7 register 2"]
pub mod dfsdm_chcfg7r2;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub struct DFSDM_AWSCD0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd0r;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub struct DFSDM_AWSCD1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd1r;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub struct DFSDM_AWSCD2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd2r;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub struct DFSDM_AWSCD3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd3r;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub struct DFSDM_AWSCD4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd4r;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub struct DFSDM_AWSCD5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd5r;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub struct DFSDM_AWSCD6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd6r;
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub struct DFSDM_AWSCD7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog and short-circuit detector register"]
pub mod dfsdm_awscd7r;
#[doc = "DFSDM channel watchdog filter data register"]
pub struct DFSDM_CHWDAT0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat0r;
#[doc = "DFSDM channel watchdog filter data register"]
pub struct DFSDM_CHWDAT1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat1r;
#[doc = "DFSDM channel watchdog filter data register"]
pub struct DFSDM_CHWDAT2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat2r;
#[doc = "DFSDM channel watchdog filter data register"]
pub struct DFSDM_CHWDAT3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat3r;
#[doc = "DFSDM channel watchdog filter data register"]
pub struct DFSDM_CHWDAT4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat4r;
#[doc = "DFSDM channel watchdog filter data register"]
pub struct DFSDM_CHWDAT5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat5r;
#[doc = "DFSDM channel watchdog filter data register"]
pub struct DFSDM_CHWDAT6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat6r;
#[doc = "DFSDM channel watchdog filter data register"]
pub struct DFSDM_CHWDAT7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel watchdog filter data register"]
pub mod dfsdm_chwdat7r;
#[doc = "DFSDM channel data input register"]
pub struct DFSDM_CHDATIN0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin0r;
#[doc = "DFSDM channel data input register"]
pub struct DFSDM_CHDATIN1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin1r;
#[doc = "DFSDM channel data input register"]
pub struct DFSDM_CHDATIN2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin2r;
#[doc = "DFSDM channel data input register"]
pub struct DFSDM_CHDATIN3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin3r;
#[doc = "DFSDM channel data input register"]
pub struct DFSDM_CHDATIN4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin4r;
#[doc = "DFSDM channel data input register"]
pub struct DFSDM_CHDATIN5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin5r;
#[doc = "DFSDM channel data input register"]
pub struct DFSDM_CHDATIN6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin6r;
#[doc = "DFSDM channel data input register"]
pub struct DFSDM_CHDATIN7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM channel data input register"]
pub mod dfsdm_chdatin7r;
#[doc = "DFSDM control register 1"]
pub struct DFSDM0_CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM control register 1"]
pub mod dfsdm0_cr1;
#[doc = "DFSDM control register 1"]
pub struct DFSDM1_CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM control register 1"]
pub mod dfsdm1_cr1;
#[doc = "DFSDM control register 1"]
pub struct DFSDM2_CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM control register 1"]
pub mod dfsdm2_cr1;
#[doc = "DFSDM control register 1"]
pub struct DFSDM3_CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM control register 1"]
pub mod dfsdm3_cr1;
#[doc = "DFSDM control register 2"]
pub struct DFSDM0_CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM control register 2"]
pub mod dfsdm0_cr2;
#[doc = "DFSDM control register 2"]
pub struct DFSDM1_CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM control register 2"]
pub mod dfsdm1_cr2;
#[doc = "DFSDM control register 2"]
pub struct DFSDM2_CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM control register 2"]
pub mod dfsdm2_cr2;
#[doc = "DFSDM control register 2"]
pub struct DFSDM3_CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM control register 2"]
pub mod dfsdm3_cr2;
#[doc = "DFSDM interrupt and status register"]
pub struct DFSDM0_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm0_isr;
#[doc = "DFSDM interrupt and status register"]
pub struct DFSDM1_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm1_isr;
#[doc = "DFSDM interrupt and status register"]
pub struct DFSDM2_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm2_isr;
#[doc = "DFSDM interrupt and status register"]
pub struct DFSDM3_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm3_isr;
#[doc = "DFSDM interrupt flag clear register"]
pub struct DFSDM0_ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm0_icr;
#[doc = "DFSDM interrupt flag clear register"]
pub struct DFSDM1_ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm1_icr;
#[doc = "DFSDM interrupt flag clear register"]
pub struct DFSDM2_ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm2_icr;
#[doc = "DFSDM interrupt flag clear register"]
pub struct DFSDM3_ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm3_icr;
#[doc = "DFSDM injected channel group selection register"]
pub struct DFSDM0_JCHGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm0_jchgr;
#[doc = "DFSDM injected channel group selection register"]
pub struct DFSDM1_JCHGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm1_jchgr;
#[doc = "DFSDM injected channel group selection register"]
pub struct DFSDM2_JCHGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm2_jchgr;
#[doc = "DFSDM injected channel group selection register"]
pub struct DFSDM3_JCHGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm3_jchgr;
#[doc = "DFSDM filter control register"]
pub struct DFSDM0_FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM filter control register"]
pub mod dfsdm0_fcr;
#[doc = "DFSDM filter control register"]
pub struct DFSDM1_FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM filter control register"]
pub mod dfsdm1_fcr;
#[doc = "DFSDM filter control register"]
pub struct DFSDM2_FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM filter control register"]
pub mod dfsdm2_fcr;
#[doc = "DFSDM filter control register"]
pub struct DFSDM3_FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM filter control register"]
pub mod dfsdm3_fcr;
#[doc = "DFSDM data register for injected group"]
pub struct DFSDM0_JDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm0_jdatar;
#[doc = "DFSDM data register for injected group"]
pub struct DFSDM1_JDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm1_jdatar;
#[doc = "DFSDM data register for injected group"]
pub struct DFSDM2_JDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm2_jdatar;
#[doc = "DFSDM data register for injected group"]
pub struct DFSDM3_JDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm3_jdatar;
#[doc = "DFSDM data register for the regular channel"]
pub struct DFSDM0_RDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm0_rdatar;
#[doc = "DFSDM data register for the regular channel"]
pub struct DFSDM1_RDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm1_rdatar;
#[doc = "DFSDM data register for the regular channel"]
pub struct DFSDM2_RDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm2_rdatar;
#[doc = "DFSDM data register for the regular channel"]
pub struct DFSDM3_RDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm3_rdatar;
#[doc = "DFSDM analog watchdog high threshold register"]
pub struct DFSDM0_AWHTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm0_awhtr;
#[doc = "DFSDM analog watchdog high threshold register"]
pub struct DFSDM1_AWHTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm1_awhtr;
#[doc = "DFSDM analog watchdog high threshold register"]
pub struct DFSDM2_AWHTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm2_awhtr;
#[doc = "DFSDM analog watchdog high threshold register"]
pub struct DFSDM3_AWHTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm3_awhtr;
#[doc = "DFSDM analog watchdog low threshold register"]
pub struct DFSDM0_AWLTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm0_awltr;
#[doc = "DFSDM analog watchdog low threshold register"]
pub struct DFSDM1_AWLTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm1_awltr;
#[doc = "DFSDM analog watchdog low threshold register"]
pub struct DFSDM2_AWLTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm2_awltr;
#[doc = "DFSDM analog watchdog low threshold register"]
pub struct DFSDM3_AWLTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm3_awltr;
#[doc = "DFSDM analog watchdog status register"]
pub struct DFSDM0_AWSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm0_awsr;
#[doc = "DFSDM analog watchdog status register"]
pub struct DFSDM1_AWSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm1_awsr;
#[doc = "DFSDM analog watchdog status register"]
pub struct DFSDM2_AWSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm2_awsr;
#[doc = "DFSDM analog watchdog status register"]
pub struct DFSDM3_AWSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm3_awsr;
#[doc = "DFSDM analog watchdog clear flag register"]
pub struct DFSDM0_AWCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm0_awcfr;
#[doc = "DFSDM analog watchdog clear flag register"]
pub struct DFSDM1_AWCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm1_awcfr;
#[doc = "DFSDM analog watchdog clear flag register"]
pub struct DFSDM2_AWCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm2_awcfr;
#[doc = "DFSDM analog watchdog clear flag register"]
pub struct DFSDM3_AWCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm3_awcfr;
#[doc = "DFSDM Extremes detector maximum register"]
pub struct DFSDM0_EXMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm0_exmax;
#[doc = "DFSDM Extremes detector maximum register"]
pub struct DFSDM1_EXMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm1_exmax;
#[doc = "DFSDM Extremes detector maximum register"]
pub struct DFSDM2_EXMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm2_exmax;
#[doc = "DFSDM Extremes detector maximum register"]
pub struct DFSDM3_EXMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm3_exmax;
#[doc = "DFSDM Extremes detector minimum register"]
pub struct DFSDM0_EXMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm0_exmin;
#[doc = "DFSDM Extremes detector minimum register"]
pub struct DFSDM1_EXMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm1_exmin;
#[doc = "DFSDM Extremes detector minimum register"]
pub struct DFSDM2_EXMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm2_exmin;
#[doc = "DFSDM Extremes detector minimum register"]
pub struct DFSDM3_EXMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm3_exmin;
#[doc = "DFSDM conversion timer register"]
pub struct DFSDM0_CNVTIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm0_cnvtimr;
#[doc = "DFSDM conversion timer register"]
pub struct DFSDM1_CNVTIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm1_cnvtimr;
#[doc = "DFSDM conversion timer register"]
pub struct DFSDM2_CNVTIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm2_cnvtimr;
#[doc = "DFSDM conversion timer register"]
pub struct DFSDM3_CNVTIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm3_cnvtimr;
