#![no_std]
#![no_main]

mod math;
use cortex_m_semihosting::hprintln;
use embedded_hal::PwmPin;
use fugit::RateExtU32;
use math::map_value;
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::pac;
use vl53l0x;

const LOW: u16 = 0;
const HIGH: u16 = 25000;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let i2c = hal::I2C::i2c0(
        pac.I2C0,
        pins.gpio16.into_mode(),
        pins.gpio17.into_mode(),
        400_u32.kHz(),
        &mut pac.RESETS,
        125_000_000_u32.Hz(),
    );

    let mut laser_sensor = vl53l0x::VL53L0x::new(i2c).ok().unwrap();

    let led_pin = pins.gpio18.into_mode::<hal::gpio::FunctionPwm>();
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    let pwm = &mut pwm_slices.pwm1;
    pwm.set_ph_correct();
    pwm.enable();

    let channel = &mut pwm.channel_a;
    channel.output_to(led_pin);

    loop {
        match laser_sensor.read_range_single_millimeters_blocking() {
            Ok(value) => {
                if value <= 2_000 {
                    let mapped = value * (HIGH / 2_000);
                    channel.set_duty(mapped);
                } else {
                    channel.set_duty(0);
                }
            }
            Err(err) => {
                hprintln!("{:?}", err);
                panic!();
            }
        };
    }
}

// End of file
