#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod blinky;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Input, Speed, Pull, AnyPin, Pin};
use embassy_time::Timer;
use blinky::init_leds;
use {defmt_rtt as _, panic_probe as _};

enum Pins
{
    P9,
    P10,
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("LED Blinking!");
    let mut leds = init_leds();

    let button = Input::new(p.PC13, Pull::Down);
    loop {
        if button.is_high() {
            info!("set p8 led to high");
            info!("set p9 led to low");
            leds[Pins::P9].toggle();
            leds[Pins::P10].set_low();
            Timer::after_millis(300).await;
        } else {
            info!("set p8 led to low");
            info!("set p9 led to high");
            leds[Pins::P9].set_low();
            leds[Pins::P10].toggle();
            Timer::after_millis(300).await;
        }
    }
}