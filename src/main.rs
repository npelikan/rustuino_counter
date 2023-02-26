#![no_std]
#![no_main]

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

    // defining seven-segment pins
    let mut disp_a = pins.d3.into_output();
    let mut disp_b = pins.d2.into_output();
    let mut disp_c = pins.d4.into_output();
    let mut disp_d = pins.d5.into_output();
    let mut disp_dp = pins.d7.into_output();
    let mut disp_e = pins.d6.into_output();
    let mut disp_f = pins.d8.into_output();
    let mut disp_g = pins.d9.into_output();

    // start with decimal point set to off
    disp_dp.set_low();

    // where the magic happens
    let mut counter = 0;

    // setting start points of the button pins
    let mut last_switchup_state = false;
    let mut last_switchdown_state = false;
    
    // loop {
    //     disp_a.toggle();
    //     disp_b.toggle();
    //     disp_c.toggle();
    //     disp_d.toggle();
    //     disp_e.toggle();
    //     disp_f.toggle();
    //     arduino_hal::delay_ms(2000);
    // }
    
    loop {

        // defining switch pins
        let switch_up_pin = pins.d13.is_high();
        let switch_down_pin = pins.d12.is_high();

        if (switch_up_pin != last_switchup_state) | (switch_down_pin != last_switchdown_state){
            if switch_up_pin {
                if counter == 9 {
                    counter = -1;
                }
                counter += 1;
            } else if switch_down_pin {
                if counter == 0 {
                    counter = 10;
                }
                counter -= 1;
            }
            last_switchup_state = switch_up_pin;
            last_switchdown_state = switch_down_pin;

            disp_a.set_low();
            disp_b.set_low();
            disp_c.set_low();
            disp_d.set_low();
            disp_e.set_low();
            disp_f.set_low();
            disp_g.set_low();

            let segments= match counter {
                0 => "abcdef",
                1 => "bc",
                2 => "abdeg",
                3 => "abcdg",
                4 => "bcfg",
                5 => "acdfg",
                6 => "acdefg",
                7 => "abc",
                8 => "abcdefg",
                9 => "abcfg",
                _ => ""
            };

            for ch in segments.chars() {
                match ch {
                    'a' => disp_a.set_high(),
                    'b' => disp_b.set_high(),
                    'c' => disp_c.set_high(),
                    'd' => disp_d.set_high(),
                    'e' => disp_e.set_high(),
                    'f' => disp_f.set_high(),
                    'g' => disp_g.set_high(),
                    _ => (),
                };
            };
            arduino_hal::delay_ms(250);
        };
        arduino_hal::delay_ms(50);
    }
}
