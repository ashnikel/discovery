#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::f32::consts::PI;

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, Direction, I16x3};

// this trait provides the `atan2` method
use m::Float;

#[entry]
fn main() -> ! {
    let (mut leds, mut lsm303dlhc, mut delay, mut _itm) = aux15::init();

    loop {
        let I16x3 { x, y, .. } = lsm303dlhc.mag().unwrap();

        let theta = (y as f32).atan2(x as f32);

        // FIXME pick a direction to point to based on `theta`
        let dir = {
            if theta < -7.0 * PI / 8.0 {
                Direction::North
            } else if theta < -5.0 * PI / 8.0 {
                Direction::Northwest
            } else if theta < -3.0 * PI / 8.0 {
                Direction::West
            } else if theta < -PI / 8.0 {
                Direction::Southwest
            } else if theta < PI / 8.0 {
                Direction::South
            } else if theta < 3.0 * PI / 8.0 {
                Direction::Southeast
            } else if theta < 5.0 * PI / 8.0 {
                Direction::East
            } else if theta < 7.0 * PI / 8.0 {
                Direction::Northeast
            } else {
                Direction::North
            }
        };

        leds.iter_mut().for_each(|led| led.off());
        leds[dir].on();

        delay.delay_ms(100_u8);
    }
}
