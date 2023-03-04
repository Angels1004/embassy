#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{assert_eq, *};
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_nrf::buffered_uarte::BufferedUarte;
use embassy_nrf::{interrupt, uarte};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut config = uarte::Config::default();
    config.parity = uarte::Parity::EXCLUDED;
    config.baudrate = uarte::Baudrate::BAUD1M;

    let mut tx_buffer = [0u8; 1024];
    let mut rx_buffer = [0u8; 1024];

    let mut u = BufferedUarte::new(
        p.UARTE0,
        p.TIMER0,
        p.PPI_CH0,
        p.PPI_CH1,
        p.PPI_GROUP0,
        interrupt::take!(UARTE0_UART0),
        p.P1_03,
        p.P1_02,
        config.clone(),
        &mut rx_buffer,
        &mut tx_buffer,
    );

    info!("uarte initialized!");

    let (mut rx, mut tx) = u.split();

    const COUNT: usize = 40_000;

    let tx_fut = async {
        let mut tx_buf = [0; 215];
        let mut i = 0;
        while i < COUNT {
            let n = tx_buf.len().min(COUNT - i);
            let tx_buf = &mut tx_buf[..n];
            for (j, b) in tx_buf.iter_mut().enumerate() {
                *b = (i + j) as u8;
            }
            let n = unwrap!(tx.write(tx_buf).await);
            i += n;
        }
    };
    let rx_fut = async {
        let mut i = 0;
        while i < COUNT {
            let buf = unwrap!(rx.fill_buf().await);

            for &b in buf {
                assert_eq!(b, i as u8);
                i = i + 1;
            }

            let n = buf.len();
            rx.consume(n);
        }
    };

    join(rx_fut, tx_fut).await;

    info!("Test OK");
    cortex_m::asm::bkpt();
}
