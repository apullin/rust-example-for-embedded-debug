#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    let mut foo : i32 = 0;
    let mut bar : f64 = 1.1;

    loop {
        // your code goes here
        foo += 1;
        bar *= 1.1;

        let mut z = test_function(foo );

        let mut baz = 0;

        for i in 1..100  {
            baz = i*10;
        }

    }
}

fn test_function( arg : i32 ) -> i32 {
    let mut temp : f64 = arg as f64;
    temp = temp * 3.1415;
    temp as i32
}