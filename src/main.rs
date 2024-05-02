#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use panic_halt as _;

use embedded_hal::digital::OutputPin;
use hal::pac;
use rp2040_hal as hal;
use rp2040_hal::clocks::Clock;

use rtt_target::*;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

pub struct Beeper<P> {
    delay: Delay,
    pin: P,
}

impl<P: OutputPin> Beeper<P> {
    pub fn start(&mut self) {
        self.pin.set_high().unwrap();
        self.delay.delay_us(1500);
        self.pin.set_low().unwrap();
        self.delay.delay_us(741);
    }
    pub fn send(&mut self, output: &str) {
        for character in output.chars() {
            if character == '1' {
                self.pin.set_high().unwrap();
                self.delay.delay_us(741);
                self.pin.set_low().unwrap();
                self.delay.delay_us(247);
            } else if character == '0' {
                self.pin.set_high().unwrap();
                self.delay.delay_us(247);
                self.pin.set_low().unwrap();
                self.delay.delay_us(741);
            }
        }
        self.delay.delay_us(7000);
    }
}

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        12_000_000u32,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    rtt_init_print!();

    let mut led_pin = pins.gpio25.into_push_pull_output();

    let radio_pin = pins.gpio15.into_push_pull_output();
    let delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let mut b = Beeper { delay: delay, pin: radio_pin };

    loop {
        led_pin.set_high().unwrap();
        b.delay.delay_ms(500);
        led_pin.set_low().unwrap();
        b.delay.delay_ms(500);
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
        b.delay.delay_ms(2000);
    }
}
