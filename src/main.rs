#![no_std]
#![no_main]

use panic_halt as _;

//use embedded_hal::digital::OutputPin;
use hal::pac;
use trinket_m0::entry;
use trinket_m0::hal::{self as hal, hal::digital::v2::OutputPin, delay::Delay};
use trinket_m0::prelude::_embedded_hal_blocking_delay_DelayMs;
use trinket_m0::prelude::_embedded_hal_blocking_delay_DelayUs;

use rtt_target::*;

pub struct Beeper<P> {
    delay: Delay,
    pin: P,
}

impl<P: OutputPin> Beeper<P> {
    pub fn start(&mut self) {
        self.pin.set_high().ok();
        self.delay.delay_us(1500u32);
        self.pin.set_low().ok();
        self.delay.delay_us(741u32);
    }
    pub fn send(&mut self, output: &str) {
        for character in output.chars() {
            if character == '1' {
                self.pin.set_high().ok();
                self.delay.delay_us(741u32);
                self.pin.set_low().ok();
                self.delay.delay_us(247u32);
            } else if character == '0' {
                self.pin.set_high().ok();
                self.delay.delay_us(247u32);
                self.pin.set_low().ok();
                self.delay.delay_us(741u32);
            }
        }
        self.delay.delay_us(7000u32);
    }
}

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut clock = hal::clock::GenericClockController::with_internal_8mhz(pac.GCLK, &mut pac.PM, &mut pac.SYSCTRL, &mut pac.NVMCTRL);

    rtt_init_print!();

    let pins = hal::gpio::v2::Pins::new(pac.PORT);
    let mut led_pin = pins.pa00.into_push_pull_output();

    let radio_pin = pins.pa01.into_push_pull_output();
    //let delay = cortex_m::delay::Delay::new(core.SYST, clock.system_clock.freq().to_Hz());
    let delay = Delay::new(core.SYST, &mut clock);
    let mut b = Beeper { delay: delay, pin: radio_pin };

    loop {
        led_pin.set_high().unwrap();
        b.delay.delay_ms(500u32);
        led_pin.set_low().unwrap();
        b.delay.delay_ms(500u32);
        rprintln!("Loop!");

        for _ in 0..5 {
            b.start();
            // high power v
            // b.send("1 000 0010 00110101001000100 1100100 1011 111 00");
            // low power v
            // b.send("1 000 0010 00110101001000100 0000100 1011 111 00");
            // low power s
            b.send("1 000 0001 00110101001000100 0000001 0111 111 00");
        }

        rprintln!("Sent!");
        b.delay.delay_ms(2000u32);
    }
}
