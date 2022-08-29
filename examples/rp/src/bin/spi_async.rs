#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::spi::{Async, Config, Spi};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    info!("Hello World!");

    let miso = p.PIN_12;
    let mosi = p.PIN_11;
    let clk = p.PIN_10;

    let mut spi: Spi<'_, _, Async> = Spi::new(p.SPI1, p.DMA_CH0, p.DMA_CH1, clk, mosi, miso, Config::default());

    loop {
        let tx_buf = [1_u8, 2, 3, 4, 5, 6];
        let mut rx_buf = [0_u8; 6];
        spi.transfer(&mut rx_buf, &tx_buf).await.unwrap();
        info!("{:?}", rx_buf);
        Timer::after(Duration::from_secs(1)).await;
    }
}
