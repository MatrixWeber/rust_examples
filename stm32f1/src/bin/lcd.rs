#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("LCD Example!");
    Timer::after_millis(30).await;

    let mut db0 = Output::new(p.PC7, Level::High, Speed::VeryHigh);
    let mut db1 = Output::new(p.PC6, Level::High, Speed::VeryHigh);
    let mut db2 = Output::new(p.PC5, Level::High, Speed::VeryHigh);
    let mut db3 = Output::new(p.PC4, Level::High, Speed::VeryHigh);
    let mut db4 = Output::new(p.PC3, Level::High, Speed::VeryHigh);
    let mut db5 = Output::new(p.PC2, Level::High, Speed::VeryHigh);
    let mut db6 = Output::new(p.PC1, Level::High, Speed::VeryHigh);
    let mut db7 = Output::new(p.PC0, Level::High, Speed::VeryHigh);
    let mut rw = Output::new(p.PC11, Level::High, Speed::VeryHigh);
    let mut rs = Output::new(p.PC12, Level::High, Speed::VeryHigh);
    let mut e = Output::new(p.PC10, Level::High, Speed::VeryHigh);
    rs.set_low();
    rw.set_low();
    // e.set_low();
    db7.set_low();
    db6.set_low();
    db4.set_high();
    db5.set_high();
    db3.set_high();
    db2.set_low();

    Timer::after_micros(50).await;
    rs.set_low();
    rw.set_low();
    // e.set_low();
    db7.set_low();
    db6.set_low();
    db4.set_low();
    db5.set_low();
    db3.set_high();
    db2.set_high();
    db1.set_high();
    db0.set_high();

    loop {
        info!("LDC set DB7 to low");
        Timer::after_millis(2000).await;
    }
}
