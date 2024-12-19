//! A loopback device. Send characters, and you should see
//! the exact same characters sent back. The LED toggles for
//! every exchanged character.
//!
//! Baud: 115200bps.

#![no_main]
#![no_std]

#[rtic::app(device = board, peripherals = false)]
mod app {
    use core::num::NonZero;

    use board::Console;
    use imxrt_dma::DMA3Channel;
    use imxrt_hal as hal;
    use imxrt_hal::dma::channel::DmaChannel;
    use imxrt_hal::dma::DMA3;
    use imxrt_hal::lpuart::{Interrupts, Watermark};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local) {
        let (
            board::Common { mut dma, .. },
            board::Specifics {
                led, mut console, ..
            },
        ) = board::new();

        // let u = unsafe { imxrt_ral::lpuart::LPUART1::instance() };
        // defmt::println!("BAUD: {:x}", u.BAUD.read());

        // console.enable_dma_transmit();
        // console.enable_dma_receive();
        // console.enable_idle_dma_receive();

        // console.disable(|d| {
        //     d.set_idle(imxrt_hal::lpuart::IdleTimeCharacter::_16);
        //     d.enable_fifo(Watermark::rx(NonZero::new(1).unwrap()));
        // });

        // defmt::println!("BAUD: {:x}", u.BAUD.read());

        led.set();

        let mut rx_channel = dma[board::BOARD_DMA_A_INDEX].take().unwrap();

        rx::spawn(console, rx_channel).ok();

        defmt::println!("Init end");

        (Shared {}, Local {})
    }

    #[task]
    async fn rx(cx: rx::Context, mut console: Console, mut rx_channel: DMA3Channel) {
        loop {
            defmt::println!("Starting DMA read");
            let mut buffer = [0u8; 4];
            rx_channel.set_interrupt_on_completion(true);
            console
                .dma_read(&mut rx_channel, &mut buffer)
                .await
                .unwrap();
        }
    }

    // #[task]
    // async fn tx(cx: tx::Context) {}

    #[task(binds = BOARD_DMA_A, priority = 1)]
    fn dma_a(cx: dma_a::Context) {
        defmt::println!("DMA A IRQ");
        unsafe { DMA3.channel(board::BOARD_DMA_A_INDEX).on_interrupt() };
    }
}
