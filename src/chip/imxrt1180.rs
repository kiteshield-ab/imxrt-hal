pub use drivers::{dma, rgpio};

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

pub mod ccm {
    pub use crate::common::ccm::*;
}

mod drivers {
    pub mod dma;
    pub mod rgpio;
}

pub(crate) mod config {
    pub use imxrt_iomuxc::imxrt1180 as pads;

    /// Minimum number of channels.
    pub const DMA_CHANNEL_COUNT: usize = 32;

    /// The number of DMA channels for eDMA3.
    pub const DMA3_CHANNEL_COUNT: usize = 32;

    /// The number of DMA channels for eDMA4.
    pub const DMA4_CHANNEL_COUNT: usize = 64;
}
