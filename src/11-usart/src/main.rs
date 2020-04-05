#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln};
use heapless::{consts, Vec};

#[entry]
fn main() -> ! {
    let (usart1, mono_timer, itm) = aux11::init();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, consts::U32> = Vec::new();

    loop {
        buffer.clear();
        // Receive a user request. Each user request ends with ENTER
        // NOTE `buffer.push` returns a `Result`. Handle the error by responding
        // with an error message.
        loop {
            // wait until there's data available
            while usart1.isr.read().rxne().bit_is_clear() {}

            // retrieve the data
            let byte = usart1.rdr.read().rdr().bits() as u8;

            if buffer.push(byte).is_err() {
                for c in b"Error: buffer is full\n\r".iter() {
                    while usart1.isr.read().txe().bit_is_clear() {}
                    usart1.tdr.write(|w| w.tdr().bits(u16::from(*c)));
                }
                break;
            }

            // Enter pressed
            if byte == 13 {
                // Send back the reversed string
                usart1.tdr.write(|w| w.tdr().bits(u16::from(b'\n')));
                usart1.tdr.write(|w| w.tdr().bits(u16::from(b'\r')));
                while let Some(c) = buffer.pop() {
                    usart1.tdr.write(|w| w.tdr().bits(u16::from(c)));
                }

                break;
            }
        }
    }
}
