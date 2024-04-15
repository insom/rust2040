#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::i2c::{self, Config};
use embassy_time::Timer;
use embedded_hal_1::i2c::I2c;
use panic_probe as _;

pub struct Lcd1602 {}

#[allow(dead_code)]
mod lcd1602 {
    pub const ADDR: u8 = 0x27;
    pub const LCD_ENABLE: u8 = 0x04;
    pub const LCD_BACKLIGHT: u8 = 0x08;
    pub const LCD_COMMAND: u8 = 0x00;
    pub const LCD_CHARACTER: u8 = 0x01;
    pub const LCD_CLEARDISPLAY: u8 = 0x01;
    pub const LCD_RETURNHOME: u8 = 0x02;
    pub const LCD_ENTRYMODESET: u8 = 0x04;
    pub const LCD_DISPLAYCONTROL: u8 = 0x08;
    pub const LCD_CURSORSHIFT: u8 = 0x10;
    pub const LCD_FUNCTIONSET: u8 = 0x20;
    pub const LCD_SETCGRAMADDR: u8 = 0x40;
    pub const LCD_SETDDRAMADDR: u8 = 0x80;
    pub const LCD_ENTRYSHIFTINCREMENT: u8 = 0x01;
    pub const LCD_BLINKON: u8 = 0x01;
    pub const LCD_CURSORON: u8 = 0x02;
    pub const LCD_DISPLAYON: u8 = 0x04;
    pub const LCD_MOVERIGHT: u8 = 0x04;
    pub const LCD_DISPLAYMOVE: u8 = 0x08;
    pub const LCD_5X10DOTS: u8 = 0x04;
    pub const LCD_2LINE: u8 = 0x08;
    pub const LCD_8BITMODE: u8 = 0x10;
    pub const LCD_ENTRYLEFT: u8 = 0x02;
}

impl Lcd1602 {
    async fn init<I: I2c>(self, i2c: &mut I) {
        use lcd1602::*;
        Self::send_byte(i2c, 3, LCD_COMMAND).await;
        Self::send_byte(i2c, 3, LCD_COMMAND).await;
        Self::send_byte(i2c, 3, LCD_COMMAND).await;
        Self::send_byte(i2c, 2, LCD_COMMAND).await;
        Self::send_byte(i2c, LCD_ENTRYMODESET | LCD_ENTRYLEFT, LCD_COMMAND).await;
        Self::send_byte(i2c, LCD_FUNCTIONSET | LCD_2LINE, LCD_COMMAND).await;
        Self::send_byte(i2c, LCD_DISPLAYCONTROL | LCD_DISPLAYON, LCD_COMMAND).await;
        Self::send_byte(i2c, LCD_CLEARDISPLAY, LCD_COMMAND).await;
        Self::send_byte(i2c, 'B' as u8, LCD_CHARACTER).await;
    }

    async fn send_byte<I: I2c>(i2c: &mut I, value: u8, mode: u8) {
        use lcd1602::*;
        let high: u8 = mode | (value & 0xf0u8) | LCD_BACKLIGHT;
        let low: u8 = mode | ((value << 4) & 0xf0u8) | LCD_BACKLIGHT;
        i2c.write(0x27u8, &[high]).unwrap();
        Self::toggle_enable(i2c, high).await;
        i2c.write(0x27u8, &[low]).unwrap();
        Self::toggle_enable(i2c, low).await;
    }

    async fn toggle_enable<I: I2c>(i2c: &mut I, value: u8) {
        use lcd1602::*;
        Timer::after_micros(600).await;
        i2c.write(0x27u8, &[value | LCD_ENABLE]).unwrap();
        Timer::after_micros(600).await;
        i2c.write(0x27u8, &[value & !LCD_ENABLE]).unwrap();
        Timer::after_micros(600).await;
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let sda = p.PIN_4;
    let scl = p.PIN_5;

    let mut i2c = i2c::I2c::new_blocking(p.I2C0, scl, sda, Config::default());

    info!("lo");
    // i2c.write(ADDR, &[GPPUB, 0xff]).unwrap(); // pullups

    let l = Lcd1602 { };
    l.init(&mut i2c).await;
    loop {
        info!("1s");
        Timer::after_secs(1).await;
    }
}
