#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m::delay::Delay;
use embedded_hal::digital::OutputPin;
use embedded_hal::i2c::I2c;
use hal::pac;
use rp2040_hal as hal;
use rp2040_hal::clocks::Clock;

use fugit::RateExtU32;
use rtt_target::*;

const LCD_ENABLE: u8 = 0x04;
const LCD_BACKLIGHT: u8 = 0x08;

const LCD_COMMAND: u8 = 0x00;
const LCD_CHARACTER: u8 = 0x01;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;


// commands
const LCD_CLEARDISPLAY: u8 = 0x01;
const LCD_RETURNHOME: u8 = 0x02;
const LCD_ENTRYMODESET: u8 = 0x04;
const LCD_DISPLAYCONTROL: u8 = 0x08;
const LCD_CURSORSHIFT: u8 = 0x10;
const LCD_FUNCTIONSET: u8 = 0x20;
const LCD_SETCGRAMADDR: u8 = 0x40;
const LCD_SETDDRAMADDR: u8 = 0x80;

// flags for display entry mode
const LCD_ENTRYSHIFTINCREMENT: u8 = 0x01;

const LCD_BLINKON: u8 = 0x01;
const LCD_CURSORON: u8 = 0x02;
const LCD_DISPLAYON: u8 = 0x04;

// flags for display and cursor shift
const LCD_MOVERIGHT: u8 = 0x04;
const LCD_DISPLAYMOVE: u8 = 0x08;

// flags for function set
const LCD_5X10DOTS: u8 = 0x04;
const LCD_2LINE: u8 = 0x08;
const LCD_8BITMODE: u8 = 0x10;

const LCD_ENTRYLEFT: u8 = 0x02;

fn init_lcd<I: I2c>(i2c: &mut I, d: &mut Delay) {
    send_byte(i2c, d, 3, LCD_COMMAND);
    send_byte(i2c, d, 3, LCD_COMMAND);
    send_byte(i2c, d, 3, LCD_COMMAND);
    send_byte(i2c, d, 2, LCD_COMMAND);
    send_byte(i2c, d, LCD_ENTRYMODESET | LCD_ENTRYLEFT, LCD_COMMAND);
    send_byte(i2c, d, LCD_FUNCTIONSET | LCD_2LINE, LCD_COMMAND);
    send_byte(i2c, d, LCD_DISPLAYCONTROL | LCD_DISPLAYON, LCD_COMMAND);
    send_byte(i2c, d, LCD_CLEARDISPLAY, LCD_COMMAND);
    send_byte(i2c, d, 'A' as u8, LCD_CHARACTER);
}

fn send_byte<I: I2c>(i2c: &mut I, d: &mut Delay, value: u8, mode: u8) {
    let high: u8 = mode | (value & 0xf0u8) | LCD_BACKLIGHT;
    let low: u8 = mode | ((value << 4) & 0xf0u8) | LCD_BACKLIGHT;
    i2c.write(0x27u8, &[high]).unwrap();
    toggle_enable(i2c, d, high);
    i2c.write(0x27u8, &[low]).unwrap();
    toggle_enable(i2c, d, low);
}

fn toggle_enable<I: I2c>(i2c: &mut I, d: &mut Delay, value: u8) {
    d.delay_us(600);
    i2c.write(0x27u8, &[value | LCD_ENABLE]).unwrap();
    d.delay_us(600);
    i2c.write(0x27u8, &[value & !LCD_ENABLE]).unwrap();
    d.delay_us(600);
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

    let i = 0x27u8;
    let mut readbuf: [u8; 1] = [0; 1];
    let result = I2c::read(&mut i2c, i, &mut readbuf);
    if let Ok(_) = result {
        rprintln!("Device found at address{:?}", i)
    } else {
        rprintln!("Device not found at address{:?}", i)
    }
    delay.delay_ms(50);

    init_lcd(&mut i2c, &mut delay);

    let mut led_pin = pins.gpio25.into_push_pull_output();
    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
        rprintln!("Loop!")
    }
}
