//! i.MX RT 1180 EVK, supporting the Cortex-M33.

use imxrt_hal::{self as hal};
use imxrt_iomuxc::imxrt1180::{gpio_ad::*, gpio_aon::*};
use imxrt_ral as ral;

#[cfg(target_arch = "arm")]
use defmt_rtt as _;
#[cfg(target_arch = "arm")]
use imxrt1180evk_fcb as _;

use panic_probe as _;

pub unsafe fn configure() {}

/// Runs on the OSC_RC_24M by default. Lucky guess!
pub const UART_CLK_FREQUENCY: u32 = 24_000_000;
/// TODO: I'm making this up. Don't make it up.
pub const LPI2C_CLK_FREQUENCY: u32 = 24_000_000;

/// USER_LED1 on the board.
///
/// Managed through GPIO4_27.
pub type Led = imxrt_hal::rgpio::Output<GPIO_AD_27>;

pub type ConsolePins = hal::lpuart::Pins<
    GPIO_AON_08, // TX, interfaced with debug chip
    GPIO_AON_09, // RX, interfaced with debug chip
>;
const CONSOLE_INSTANCE: u8 = 1;
pub type Console = hal::lpuart::Lpuart<ConsolePins, { CONSOLE_INSTANCE }>;

pub const CONSOLE_BAUD: hal::lpuart::Baud = hal::lpuart::Baud::compute(UART_CLK_FREQUENCY, 115200);

#[non_exhaustive]
pub struct Specifics {
    pub led: Led,
    pub console: Console,
}

impl Specifics {
    pub(crate) fn new(_: &mut crate::Common) -> Self {
        let ral::Instances {
            IOMUXC,
            IOMUXC_AON,
            RGPIO4,
            ..
        } = unsafe { ral::Instances::instances() };
        let pads = imxrt_hal::iomuxc::into_pads(IOMUXC, IOMUXC_AON);

        let mut gpio4 = imxrt_hal::rgpio::Port::new(RGPIO4);
        let led = gpio4.output(pads.gpio_ad.p27);

        let console = unsafe { ral::lpuart::Instance::<{ CONSOLE_INSTANCE }>::instance() };
        let mut console = hal::lpuart::Lpuart::new(
            console,
            ConsolePins {
                tx: pads.gpio_aon.p08,
                rx: pads.gpio_aon.p09,
            },
        );
        console.disable(|console| {
            console.set_baud(&CONSOLE_BAUD);
            console.set_parity(None);
        });

        Specifics { led, console }
    }
}

pub mod interrupt {
    use crate::board_interrupts as syms;
    use crate::ral::Interrupt;

    pub const BOARD_CONSOLE: Interrupt = Interrupt::LPUART1;
    pub const BOARD_DMA_A: Interrupt = Interrupt::DMA3_CH7;
    pub const BOARD_DMA_B: Interrupt = Interrupt::DMA3_CH11;

    pub const INTERRUPTS: &[(Interrupt, syms::Vector)] = &[
        (BOARD_CONSOLE, syms::BOARD_CONSOLE),
        (BOARD_DMA_A, syms::BOARD_DMA_A),
        (BOARD_DMA_B, syms::BOARD_DMA_B),
    ];
}

pub use interrupt as Interrupt;
