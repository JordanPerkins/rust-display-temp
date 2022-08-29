#![no_std]
#![no_main]

use panic_halt as _;
use ag_lcd::{Display, Blink, Cursor, LcdDisplay};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let delay = arduino_hal::Delay::new();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    // Pressable Button Pins
    let button = pins.d6.into_pull_up_input();

    // LCD Display Pins
    let rs = pins.d7.into_output().downgrade();
    let en = pins.d8.into_output().downgrade();
    let d4 = pins.d9.into_output().downgrade();
    let d5 = pins.d10.into_output().downgrade();
    let d6 = pins.d11.into_output().downgrade();
    let d7 = pins.d12.into_output().downgrade();

    // LCD Display package
    let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
        .with_half_bus(d4, d5, d6, d7)
        .with_display(Display::On)
        .with_blink(Blink::On)
        .with_cursor(Cursor::On)
        .build();

    lcd.set_cursor(Cursor::Off);
    lcd.set_blink(Blink::Off);

    loop {
        if (button.is_low()) {
            lcd.print("Button pressed!");
            arduino_hal::delay_ms(1000);
            lcd.clear();
        }
    }
}
