#![no_std]
#![no_main]

mod blinky;

#[path = "counters/limit_counter.rs"]
mod limit_counter;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pull, Output};
use blinky::{init_leds, COUNTER_LIMIT};
use limit_counter::{LimitCounter, Reset, Count, Show};
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
    let mut limit_counter = LimitCounter::new(0, COUNTER_LIMIT);
    let button = Input::new(p.PC13, Pull::Down);
    blinking_loop(&mut leds, &mut limit_counter, &button).await;
}

async fn blinking_loop<T>(leds: &mut [Output<'_>; 8], limit_counter: &mut T, button: &Input<'_>)
where
    T: Count + Reset + Show,
{
    if button.is_high() {
        limit_counter.reset();
    }
    else {
        blinky::blinking_loop(leds, limit_counter).await;
    }
}