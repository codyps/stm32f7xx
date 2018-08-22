#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DSI Host Version Register"]
    pub dsi_vr: DSI_VR,
    #[doc = "0x04 - DSI Host Control Register"]
    pub dsi_cr: DSI_CR,
    #[doc = "0x08 - DSI HOST Clock Control Register"]
    pub dsi_ccr: DSI_CCR,
    #[doc = "0x0c - DSI Host LTDC VCID Register"]
    pub dsi_lvcidr: DSI_LVCIDR,
    #[doc = "0x10 - DSI Host LTDC Color Coding Register"]
    pub dsi_lcolcr: DSI_LCOLCR,
    #[doc = "0x14 - DSI Host LTDC Polarity Configuration Register"]
    pub dsi_lpcr: DSI_LPCR,
    #[doc = "0x18 - DSI Host Low-Power mode Configuration Register"]
    pub dsi_lpmcr: DSI_LPMCR,
    _reserved0: [u8; 16usize],
    #[doc = "0x2c - DSI Host Protocol Configuration Register"]
    pub dsi_pcr: DSI_PCR,
    #[doc = "0x30 - DSI Host Generic VCID Register"]
    pub dsi_gvcidr: DSI_GVCIDR,
    #[doc = "0x34 - DSI Host mode Configuration Register"]
    pub dsi_mcr: DSI_MCR,
    #[doc = "0x38 - DSI Host Video mode Configuration Register"]
    pub dsi_vmcr: DSI_VMCR,
    #[doc = "0x3c - DSI Host Video Packet Configuration Register"]
    pub dsi_vpcr: DSI_VPCR,
    #[doc = "0x40 - DSI Host Video Chunks Configuration Register"]
    pub dsi_vccr: DSI_VCCR,
    #[doc = "0x44 - DSI Host Video Null Packet Configuration Register"]
    pub dsi_vnpcr: DSI_VNPCR,
    #[doc = "0x48 - DSI Host Video HSA Configuration Register"]
    pub dsi_vhsacr: DSI_VHSACR,
    #[doc = "0x4c - DSI Host Video HBP Configuration Register"]
    pub dsi_vhbpcr: DSI_VHBPCR,
    #[doc = "0x50 - DSI Host Video Line Configuration Register"]
    pub dsi_vlcr: DSI_VLCR,
    #[doc = "0x54 - DSI Host Video VSA Configuration Register"]
    pub dsi_vvsacr: DSI_VVSACR,
    #[doc = "0x58 - DSI Host Video VBP Configuration Register"]
    pub dsi_vvbpcr: DSI_VVBPCR,
    #[doc = "0x5c - DSI Host Video VFP Configuration Register"]
    pub dsi_vvfpcr: DSI_VVFPCR,
    #[doc = "0x60 - DSI Host Video VA Configuration Register"]
    pub dsi_vvacr: DSI_VVACR,
    #[doc = "0x64 - DSI Host LTDC Command Configuration Register"]
    pub dsi_lccr: DSI_LCCR,
    #[doc = "0x68 - DSI Host Command mode Configuration Register"]
    pub dsi_cmcr: DSI_CMCR,
    #[doc = "0x6c - DSI Host Generic Header Configuration Register"]
    pub dsi_ghcr: DSI_GHCR,
    #[doc = "0x70 - DSI Host Generic Payload Data Register"]
    pub dsi_gpdr: DSI_GPDR,
    #[doc = "0x74 - DSI Host Generic Packet Status Register"]
    pub dsi_gpsr: DSI_GPSR,
    #[doc = "0x78 - DSI Host Timeout Counter Configuration Register 0"]
    pub dsi_tccr0: DSI_TCCR0,
    #[doc = "0x7c - DSI Host Timeout Counter Configuration Register 1"]
    pub dsi_tccr1: DSI_TCCR1,
    #[doc = "0x80 - DSI Host Timeout Counter Configuration Register 2"]
    pub dsi_tccr2: DSI_TCCR2,
    #[doc = "0x84 - DSI Host Timeout Counter Configuration Register 3"]
    pub dsi_tccr3: DSI_TCCR3,
    #[doc = "0x88 - DSI Host Timeout Counter Configuration Register 4"]
    pub dsi_tccr4: DSI_TCCR4,
    #[doc = "0x8c - DSI Host Timeout Counter Configuration Register 5"]
    pub dsi_tccr5: DSI_TCCR5,
    _reserved1: [u8; 4usize],
    #[doc = "0x94 - DSI Host Clock Lane Configuration Register"]
    pub dsi_clcr: DSI_CLCR,
    #[doc = "0x98 - DSI Host Clock Lane Timer Configuration Register"]
    pub dsi_cltcr: DSI_CLTCR,
    #[doc = "0x9c - DSI Host Data Lane Timer Configuration Register"]
    pub dsi_dltcr: DSI_DLTCR,
    #[doc = "0xa0 - DSI Host PHY Control Register"]
    pub dsi_pctlr: DSI_PCTLR,
    #[doc = "0xa4 - DSI Host PHY Configuration Register"]
    pub dsi_pconfr: DSI_PCONFR,
    #[doc = "0xa8 - DSI Host PHY ULPS Control Register"]
    pub dsi_pucr: DSI_PUCR,
    #[doc = "0xac - DSI Host PHY TX Triggers Configuration Register"]
    pub dsi_pttcr: DSI_PTTCR,
    #[doc = "0xb0 - DSI Host PHY Status Register"]
    pub dsi_psr: DSI_PSR,
    _reserved2: [u8; 8usize],
    #[doc = "0xbc - DSI Host Interrupt & Status Register 0"]
    pub dsi_isr0: DSI_ISR0,
    #[doc = "0xc0 - DSI Host Interrupt & Status Register 1"]
    pub dsi_isr1: DSI_ISR1,
    #[doc = "0xc4 - DSI Host Interrupt Enable Register 0"]
    pub dsi_ier0: DSI_IER0,
    #[doc = "0xc8 - DSI Host Interrupt Enable Register 1"]
    pub dsi_ier1: DSI_IER1,
    _reserved3: [u8; 12usize],
    #[doc = "0xd8 - DSI Host Force Interrupt Register 0"]
    pub dsi_fir0: DSI_FIR0,
    #[doc = "0xdc - DSI Host Force Interrupt Register 1"]
    pub dsi_fir1: DSI_FIR1,
    _reserved4: [u8; 32usize],
    #[doc = "0x100 - DSI Host Video Shadow Control Register"]
    pub dsi_vscr: DSI_VSCR,
    _reserved5: [u8; 8usize],
    #[doc = "0x10c - DSI Host LTDC Current VCID Register"]
    pub dsi_lcvcidr: DSI_LCVCIDR,
    #[doc = "0x110 - DSI Host LTDC Current Color Coding Register"]
    pub dsi_lcccr: DSI_LCCCR,
    _reserved6: [u8; 4usize],
    #[doc = "0x118 - DSI Host Low-Power mode Current Configuration Register"]
    pub dsi_lpmccr: DSI_LPMCCR,
    _reserved7: [u8; 28usize],
    #[doc = "0x138 - DSI Host Video mode Current Configuration Register"]
    pub dsi_vmccr: DSI_VMCCR,
    #[doc = "0x13c - DSI Host Video Packet Current Configuration Register"]
    pub dsi_vpccr: DSI_VPCCR,
    #[doc = "0x140 - DSI Host Video Chunks Current Configuration Register"]
    pub dsi_vcccr: DSI_VCCCR,
    #[doc = "0x144 - DSI Host Video Null Packet Current Configuration Register"]
    pub dsi_vnpccr: DSI_VNPCCR,
    #[doc = "0x148 - DSI Host Video HSA Current Configuration Register"]
    pub dsi_vhsaccr: DSI_VHSACCR,
    #[doc = "0x14c - DSI Host Video HBP Current Configuration Register"]
    pub dsi_vhbpccr: DSI_VHBPCCR,
    #[doc = "0x150 - DSI Host Video Line Current Configuration Register"]
    pub dsi_vlccr: DSI_VLCCR,
    #[doc = "0x154 - DSI Host Video VSA Current Configuration Register"]
    pub dsi_vvsaccr: DSI_VVSACCR,
    #[doc = "0x158 - DSI Host Video VBP Current Configuration Register"]
    pub dsi_vvbpccr: DSI_VVBPCCR,
    #[doc = "0x15c - DSI Host Video VFP Current Configuration Register"]
    pub dsi_vvfpccr: DSI_VVFPCCR,
    #[doc = "0x160 - DSI Host Video VA Current Configuration Register"]
    pub dsi_vvaccr: DSI_VVACCR,
    _reserved8: [u8; 668usize],
    #[doc = "0x400 - DSI Wrapper Configuration Register"]
    pub dsi_wcfgr: DSI_WCFGR,
    #[doc = "0x404 - DSI Wrapper Control Register"]
    pub dsi_wcr: DSI_WCR,
    #[doc = "0x408 - DSI Wrapper Interrupt Enable Register"]
    pub dsi_wier: DSI_WIER,
    #[doc = "0x40c - DSI Wrapper Interrupt & Status Register"]
    pub dsi_wisr: DSI_WISR,
    #[doc = "0x410 - DSI Wrapper Interrupt Flag Clear Register"]
    pub dsi_wifcr: DSI_WIFCR,
    _reserved9: [u8; 4usize],
    #[doc = "0x418 - DSI Wrapper PHY Configuration Register 1"]
    pub dsi_wpcr1: DSI_WPCR1,
    #[doc = "0x41c - DSI Wrapper PHY Configuration Register 2"]
    pub dsi_wpcr2: DSI_WPCR2,
    #[doc = "0x420 - DSI Wrapper PHY Configuration Register 3"]
    pub dsi_wpcr3: DSI_WPCR3,
    #[doc = "0x424 - DSI_WPCR4"]
    pub dsi_wpcr4: DSI_WPCR4,
    #[doc = "0x428 - DSI Wrapper PHY Configuration Register 5"]
    pub dsi_wpcr5: DSI_WPCR5,
    _reserved10: [u8; 4usize],
    #[doc = "0x430 - DSI Wrapper Regulator and PLL Control Register"]
    pub dsi_wrpcr: DSI_WRPCR,
}
#[doc = "DSI Host Version Register"]
pub struct DSI_VR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Version Register"]
pub mod dsi_vr;
#[doc = "DSI Host Control Register"]
pub struct DSI_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Control Register"]
pub mod dsi_cr;
#[doc = "DSI HOST Clock Control Register"]
pub struct DSI_CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI HOST Clock Control Register"]
pub mod dsi_ccr;
#[doc = "DSI Host LTDC VCID Register"]
pub struct DSI_LVCIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host LTDC VCID Register"]
pub mod dsi_lvcidr;
#[doc = "DSI Host LTDC Color Coding Register"]
pub struct DSI_LCOLCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host LTDC Color Coding Register"]
pub mod dsi_lcolcr;
#[doc = "DSI Host LTDC Polarity Configuration Register"]
pub struct DSI_LPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host LTDC Polarity Configuration Register"]
pub mod dsi_lpcr;
#[doc = "DSI Host Low-Power mode Configuration Register"]
pub struct DSI_LPMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Low-Power mode Configuration Register"]
pub mod dsi_lpmcr;
#[doc = "DSI Host Protocol Configuration Register"]
pub struct DSI_PCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Protocol Configuration Register"]
pub mod dsi_pcr;
#[doc = "DSI Host Generic VCID Register"]
pub struct DSI_GVCIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Generic VCID Register"]
pub mod dsi_gvcidr;
#[doc = "DSI Host mode Configuration Register"]
pub struct DSI_MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host mode Configuration Register"]
pub mod dsi_mcr;
#[doc = "DSI Host Video mode Configuration Register"]
pub struct DSI_VMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video mode Configuration Register"]
pub mod dsi_vmcr;
#[doc = "DSI Host Video Packet Configuration Register"]
pub struct DSI_VPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video Packet Configuration Register"]
pub mod dsi_vpcr;
#[doc = "DSI Host Video Chunks Configuration Register"]
pub struct DSI_VCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video Chunks Configuration Register"]
pub mod dsi_vccr;
#[doc = "DSI Host Video Null Packet Configuration Register"]
pub struct DSI_VNPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video Null Packet Configuration Register"]
pub mod dsi_vnpcr;
#[doc = "DSI Host Video HSA Configuration Register"]
pub struct DSI_VHSACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video HSA Configuration Register"]
pub mod dsi_vhsacr;
#[doc = "DSI Host Video HBP Configuration Register"]
pub struct DSI_VHBPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video HBP Configuration Register"]
pub mod dsi_vhbpcr;
#[doc = "DSI Host Video Line Configuration Register"]
pub struct DSI_VLCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video Line Configuration Register"]
pub mod dsi_vlcr;
#[doc = "DSI Host Video VSA Configuration Register"]
pub struct DSI_VVSACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video VSA Configuration Register"]
pub mod dsi_vvsacr;
#[doc = "DSI Host Video VBP Configuration Register"]
pub struct DSI_VVBPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video VBP Configuration Register"]
pub mod dsi_vvbpcr;
#[doc = "DSI Host Video VFP Configuration Register"]
pub struct DSI_VVFPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video VFP Configuration Register"]
pub mod dsi_vvfpcr;
#[doc = "DSI Host Video VA Configuration Register"]
pub struct DSI_VVACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video VA Configuration Register"]
pub mod dsi_vvacr;
#[doc = "DSI Host LTDC Command Configuration Register"]
pub struct DSI_LCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host LTDC Command Configuration Register"]
pub mod dsi_lccr;
#[doc = "DSI Host Command mode Configuration Register"]
pub struct DSI_CMCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Command mode Configuration Register"]
pub mod dsi_cmcr;
#[doc = "DSI Host Generic Header Configuration Register"]
pub struct DSI_GHCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Generic Header Configuration Register"]
pub mod dsi_ghcr;
#[doc = "DSI Host Generic Payload Data Register"]
pub struct DSI_GPDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Generic Payload Data Register"]
pub mod dsi_gpdr;
#[doc = "DSI Host Generic Packet Status Register"]
pub struct DSI_GPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Generic Packet Status Register"]
pub mod dsi_gpsr;
#[doc = "DSI Host Timeout Counter Configuration Register 0"]
pub struct DSI_TCCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Timeout Counter Configuration Register 0"]
pub mod dsi_tccr0;
#[doc = "DSI Host Timeout Counter Configuration Register 1"]
pub struct DSI_TCCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Timeout Counter Configuration Register 1"]
pub mod dsi_tccr1;
#[doc = "DSI Host Timeout Counter Configuration Register 2"]
pub struct DSI_TCCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Timeout Counter Configuration Register 2"]
pub mod dsi_tccr2;
#[doc = "DSI Host Timeout Counter Configuration Register 3"]
pub struct DSI_TCCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Timeout Counter Configuration Register 3"]
pub mod dsi_tccr3;
#[doc = "DSI Host Timeout Counter Configuration Register 4"]
pub struct DSI_TCCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Timeout Counter Configuration Register 4"]
pub mod dsi_tccr4;
#[doc = "DSI Host Timeout Counter Configuration Register 5"]
pub struct DSI_TCCR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Timeout Counter Configuration Register 5"]
pub mod dsi_tccr5;
#[doc = "DSI Host Clock Lane Configuration Register"]
pub struct DSI_CLCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Clock Lane Configuration Register"]
pub mod dsi_clcr;
#[doc = "DSI Host Clock Lane Timer Configuration Register"]
pub struct DSI_CLTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Clock Lane Timer Configuration Register"]
pub mod dsi_cltcr;
#[doc = "DSI Host Data Lane Timer Configuration Register"]
pub struct DSI_DLTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Data Lane Timer Configuration Register"]
pub mod dsi_dltcr;
#[doc = "DSI Host PHY Control Register"]
pub struct DSI_PCTLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host PHY Control Register"]
pub mod dsi_pctlr;
#[doc = "DSI Host PHY Configuration Register"]
pub struct DSI_PCONFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host PHY Configuration Register"]
pub mod dsi_pconfr;
#[doc = "DSI Host PHY ULPS Control Register"]
pub struct DSI_PUCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host PHY ULPS Control Register"]
pub mod dsi_pucr;
#[doc = "DSI Host PHY TX Triggers Configuration Register"]
pub struct DSI_PTTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host PHY TX Triggers Configuration Register"]
pub mod dsi_pttcr;
#[doc = "DSI Host PHY Status Register"]
pub struct DSI_PSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host PHY Status Register"]
pub mod dsi_psr;
#[doc = "DSI Host Interrupt & Status Register 0"]
pub struct DSI_ISR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Interrupt & Status Register 0"]
pub mod dsi_isr0;
#[doc = "DSI Host Interrupt & Status Register 1"]
pub struct DSI_ISR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Interrupt & Status Register 1"]
pub mod dsi_isr1;
#[doc = "DSI Host Interrupt Enable Register 0"]
pub struct DSI_IER0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Interrupt Enable Register 0"]
pub mod dsi_ier0;
#[doc = "DSI Host Interrupt Enable Register 1"]
pub struct DSI_IER1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Interrupt Enable Register 1"]
pub mod dsi_ier1;
#[doc = "DSI Host Force Interrupt Register 0"]
pub struct DSI_FIR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Force Interrupt Register 0"]
pub mod dsi_fir0;
#[doc = "DSI Host Force Interrupt Register 1"]
pub struct DSI_FIR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Force Interrupt Register 1"]
pub mod dsi_fir1;
#[doc = "DSI Host Video Shadow Control Register"]
pub struct DSI_VSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video Shadow Control Register"]
pub mod dsi_vscr;
#[doc = "DSI Host LTDC Current VCID Register"]
pub struct DSI_LCVCIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host LTDC Current VCID Register"]
pub mod dsi_lcvcidr;
#[doc = "DSI Host LTDC Current Color Coding Register"]
pub struct DSI_LCCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host LTDC Current Color Coding Register"]
pub mod dsi_lcccr;
#[doc = "DSI Host Low-Power mode Current Configuration Register"]
pub struct DSI_LPMCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Low-Power mode Current Configuration Register"]
pub mod dsi_lpmccr;
#[doc = "DSI Host Video mode Current Configuration Register"]
pub struct DSI_VMCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video mode Current Configuration Register"]
pub mod dsi_vmccr;
#[doc = "DSI Host Video Packet Current Configuration Register"]
pub struct DSI_VPCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video Packet Current Configuration Register"]
pub mod dsi_vpccr;
#[doc = "DSI Host Video Chunks Current Configuration Register"]
pub struct DSI_VCCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video Chunks Current Configuration Register"]
pub mod dsi_vcccr;
#[doc = "DSI Host Video Null Packet Current Configuration Register"]
pub struct DSI_VNPCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video Null Packet Current Configuration Register"]
pub mod dsi_vnpccr;
#[doc = "DSI Host Video HSA Current Configuration Register"]
pub struct DSI_VHSACCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video HSA Current Configuration Register"]
pub mod dsi_vhsaccr;
#[doc = "DSI Host Video HBP Current Configuration Register"]
pub struct DSI_VHBPCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video HBP Current Configuration Register"]
pub mod dsi_vhbpccr;
#[doc = "DSI Host Video Line Current Configuration Register"]
pub struct DSI_VLCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video Line Current Configuration Register"]
pub mod dsi_vlccr;
#[doc = "DSI Host Video VSA Current Configuration Register"]
pub struct DSI_VVSACCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video VSA Current Configuration Register"]
pub mod dsi_vvsaccr;
#[doc = "DSI Host Video VBP Current Configuration Register"]
pub struct DSI_VVBPCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video VBP Current Configuration Register"]
pub mod dsi_vvbpccr;
#[doc = "DSI Host Video VFP Current Configuration Register"]
pub struct DSI_VVFPCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video VFP Current Configuration Register"]
pub mod dsi_vvfpccr;
#[doc = "DSI Host Video VA Current Configuration Register"]
pub struct DSI_VVACCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Host Video VA Current Configuration Register"]
pub mod dsi_vvaccr;
#[doc = "DSI Wrapper Configuration Register"]
pub struct DSI_WCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Wrapper Configuration Register"]
pub mod dsi_wcfgr;
#[doc = "DSI Wrapper Control Register"]
pub struct DSI_WCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Wrapper Control Register"]
pub mod dsi_wcr;
#[doc = "DSI Wrapper Interrupt Enable Register"]
pub struct DSI_WIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Wrapper Interrupt Enable Register"]
pub mod dsi_wier;
#[doc = "DSI Wrapper Interrupt & Status Register"]
pub struct DSI_WISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Wrapper Interrupt & Status Register"]
pub mod dsi_wisr;
#[doc = "DSI Wrapper Interrupt Flag Clear Register"]
pub struct DSI_WIFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Wrapper Interrupt Flag Clear Register"]
pub mod dsi_wifcr;
#[doc = "DSI Wrapper PHY Configuration Register 1"]
pub struct DSI_WPCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Wrapper PHY Configuration Register 1"]
pub mod dsi_wpcr1;
#[doc = "DSI Wrapper PHY Configuration Register 2"]
pub struct DSI_WPCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Wrapper PHY Configuration Register 2"]
pub mod dsi_wpcr2;
#[doc = "DSI Wrapper PHY Configuration Register 3"]
pub struct DSI_WPCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Wrapper PHY Configuration Register 3"]
pub mod dsi_wpcr3;
#[doc = "DSI_WPCR4"]
pub struct DSI_WPCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI_WPCR4"]
pub mod dsi_wpcr4;
#[doc = "DSI Wrapper PHY Configuration Register 5"]
pub struct DSI_WPCR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Wrapper PHY Configuration Register 5"]
pub mod dsi_wpcr5;
#[doc = "DSI Wrapper Regulator and PLL Control Register"]
pub struct DSI_WRPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DSI Wrapper Regulator and PLL Control Register"]
pub mod dsi_wrpcr;
