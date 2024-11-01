pub mod gpio;
pub mod ccm;
pub mod dma {}

pub(crate) mod reexports {
    pub use super::gpio;
}

pub(crate) mod iomuxc {
    pub use super::config::pads;
    use crate::ral;

    /// Transform the `imxrt-ral` IOMUXC instances into pad objects.
    pub fn into_pads(_: ral::iomuxc::IOMUXC, _: ral::iomuxc_aon::IOMUXC_AON) -> pads::Pads {
        // Safety: acquiring pads has the same safety implications
        // as acquiring the IOMUXC instances. The user has already
        // assumed the unsafety.
        unsafe { pads::Pads::new() }
    }
}

mod config {
    pub use imxrt_iomuxc::imxrt1180 as pads;
}
