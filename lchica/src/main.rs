#![no_std]
#![no_main]

use arduino_hal::simple_pwm::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let delay = 20;
    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    let mut red   = pins.d6.into_output().into_pwm(&timer0);
    let mut green = pins.d5.into_output().into_pwm(&timer0);
    let mut blue  = pins.d3.into_output().into_pwm(&timer2);

    red.enable();
    green.enable();
    blue.enable();

    loop {
        for (r, g) in (0..=255).rev().zip(0..=255) {
            red.set_duty(r);
            green.set_duty(g);
            arduino_hal::delay_ms(delay);
        }

        for (g, b) in (0..=255).rev().zip(0..=255) {
            green.set_duty(g);
            blue.set_duty(b);
            arduino_hal::delay_ms(delay);
        }

        for (b, r) in (0..=255).rev().zip(0..=255) {
            blue.set_duty(b);
            red.set_duty(r);
            arduino_hal::delay_ms(delay);
        }
    }
}
