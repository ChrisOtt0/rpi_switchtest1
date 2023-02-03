use std::{thread, time::Duration};
use rppal::gpio::Gpio;

fn main() {
    let gpio = match Gpio::new() {
        Ok(v) => v,
        Err(e) => panic!("Error creating Gpio: {e}"),
    };

    let input = gpio.get(18)
        .unwrap_or_else(|e| panic!("Error creating Pin: {e}"))
        .into_input_pulldown();

    let mut out1 = gpio.get(26)
        .unwrap_or_else(|e| panic!("Error creating Pin: {e}"))
        .into_output();

    let mut out2 = gpio.get(21)
        .unwrap_or_else(|e| panic!("Error creating Pin: {e}"))
        .into_output();

    let y = 200;
    let x = 200;
    let mut input_state = true;

    while input_state {
        out1.set_high();
        out2.set_low();
        thread::sleep(Duration::from_millis(y));

        out1.set_low();
        out2.set_high();
        thread::sleep(Duration::from_millis(x));
        input_state = input.is_high();
    }

    out2.set_low();
}
