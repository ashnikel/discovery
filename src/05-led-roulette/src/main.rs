#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let tick = 50_u16;


    loop {
        for i in 0..8 {
            leds[i].on();
            delay.delay_ms(tick);
            let next = (i + 1) % 8;
            leds[next].on();
            delay.delay_ms(tick);
            leds[i].off();
        }
    }
}
