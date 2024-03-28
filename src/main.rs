#![no_std]
#![no_main]

use panic_halt as _;

use embedded_hal::digital::OutputPin;
use hal::pac;
use rp2040_hal as hal;
use rp2040_hal::clocks::Clock;

use fugit::RateExtU32;
use rtt_target::*;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    rtt_init_print!();

    let mut i2c = hal::I2C::i2c0(
        pac.I2C0,
        pins.gpio4.reconfigure(), // sda
        pins.gpio5.reconfigure(), // scl
        100.kHz(),
        &mut pac.RESETS,
        125_000_000.Hz(),
    );

    use cortex_m::prelude::_embedded_hal_blocking_i2c_Read;
    let i = 0x27u8;
    let mut readbuf: [u8; 1] = [0; 1];
    let result = i2c.read(i, &mut readbuf);
    if let Ok(_) = result {
        rprintln!("Device found at address{:?}", i)
    } else {
        rprintln!("Device not found at address{:?}", i)
    }
    delay.delay_ms(50);

    let mut led_pin = pins.gpio25.into_push_pull_output();
    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
        rprintln!("Loop!")
    }
}
