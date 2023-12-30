#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

#[path = "my_work/limit_counter.rs"]
mod limit_counter;

use limit_counter::LimitCounter;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Pin, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

const BLINK_INTERVAL: u64 = 1000;

const COUNTER_LIMIT: usize = 60;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("LED Blinking with Limit Counter!");
    let mut leds = init_leds();
    let mut limit_counter = LimitCounter::new(0, COUNTER_LIMIT);
    loop {
        blinking_loop(&mut leds, &mut limit_counter).await;
    }
}

const LED_COUNT: usize = 8;

/*
Die Lebensdauer in Rust hilft, zu garantieren, dass eine Referenz auf ein Objekt nicht länger existiert,
als das Objekt selbst. Wenn der Compiler also eine Lebensdauer sieht, prüft er,
ob der Zugriff auf das referenzierte Objekt innerhalb dieser Lebensdauer sicher ist.

In diesem Fall bedeutet die Verwendung von 'static für die Lebensdauer in Output<'static, AnyPin>,
dass diese Objekte für die gesamte Dauer des Programms leben. Wenn Sie eine andere Lebensdauer angeben würden,
hätte der Compiler möglicherweise Bedenken, dass die Output Objekte zerstört werden könnten,
bevor die Referenzen auf sie aufgehoben werden, was zu unsicherem Verhalten führen könnte.
 */
pub fn init_leds() -> [Output<'static, AnyPin>; LED_COUNT] {
    let p = embassy_stm32::init(Default::default());
    let mut leds = [
        Output::new(p.PB8.degrade(), Level::High, Speed::Low),
        Output::new(p.PB9.degrade(), Level::High, Speed::Low),
        Output::new(p.PB10.degrade(), Level::High, Speed::Low),
        Output::new(p.PB11.degrade(), Level::High, Speed::Low),
        Output::new(p.PB12.degrade(), Level::High, Speed::Low),
        Output::new(p.PB13.degrade(), Level::High, Speed::Low),
        Output::new(p.PB14.degrade(), Level::High, Speed::Low),
        Output::new(p.PB15.degrade(), Level::High, Speed::Low),
    ];
    // alle ausschalten
    leds.iter_mut().for_each(|led| led.set_low());
    // leds Array zurückgeben
    leds
}

/*
In asynchronen Funktionen in Rust erzeugt der Compiler eine versteckte Struktur,
die den Zustand der asynchronen Funktion speichert. Diese Struktur enthält die Parameter der Funktion und andere Werte,
die im Kontext der Funktion verwendet werden. Daher benötigt der Compiler Lebensdauerinformationen für diese Parameter und Werte,
um sicherzustellen, dass sie während der gesamten Ausführungszeit der asynchronen Funktion gültig bleiben.

Insbesondere erfordert der Compiler in Ihrer Funktion blink_led,
dass die Lebensdauer des Output<'_, AnyPin>-Parameters ausdrücklich angegeben wird, um zu gewährleisten,
dass die Referenz auf led während der gesamten Ausführungszeit der Funktion gültig bleibt.
Ohne diese Information könnte der Compiler nicht garantieren, dass die Referenz auf led nicht aufgehoben wird,
bevor die Funktion vollständig ausgeführt wurde, was zu unsicheren Zugriffen auf Speicher führen könnte.

Daher ist es in Rust üblich, Lebensdauern in asynchronen Funktionen anzugeben,
auch wenn die gleiche Funktion in synchronem Code keine explizite Lebensdauer erfordern würde.

In Rust können Sie einen anonymen Lebensdauer-Spezifikator _ verwenden,
wenn Sie nicht vorhaben, die Lebensdauer ausdrücklich in Ihrer Funktion zu verwenden.

Es ist eine Art Kurzschreibweise für eine generische Lebensdauer und wird oft verwendet,
wenn die genaue Lebensdauer unwichtig ist. Es bedeutet "irgendeine Lebensdauer" und wird vom Rust-Compiler durch
die tatsächliche Lebensdauer ersetzt, die er durch seine Lebensdauer-Überprüfung ermittelt.
 */
async fn blinking_loop(leds: &mut [Output<'_, AnyPin>; 8], limit_counter: &mut LimitCounter) {
    let counter_value = limit_counter.get_counter();
    info!("counter value: {}", counter_value);
    let counter_remainder = counter_value % leds.len();

    for (i, led) in leds.iter_mut().enumerate() {
        if i == counter_remainder {
            blink_led(led, BLINK_INTERVAL).await;
        } else {
            led.set_low();
        }
    }

    limit_counter.count();
}

async fn blink_led(led: &mut Output<'_, AnyPin>, duration_ms: u64) {
    led.set_high();
    Timer::after_millis(duration_ms).await;
    led.set_low();
}
