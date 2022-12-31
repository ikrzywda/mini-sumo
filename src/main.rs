#![no_std]
#![no_main]

mod sample;
use cortex_m_semihosting::hprintln;
use fugit::RateExtU32;
use panic_halt as _;
use rp_pico::entry;
use rp_pico::hal;
use rp_pico::hal::pac;
use vl53l0x;

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

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut time = 0;
    let mut laser_sensor = vl53l0x::VL53L0x::new(i2c).ok().unwrap();

    loop {
        if timer.get_counter() - time >= 200 {
            match laser_sensor.read_range_single_millimeters_blocking() {
                Ok(value) => {
                    hprintln!("Result: {}", value);
                }
                Err(err) => {
                    hprintln!("{:?}", err);
                    panic!();
                }
            };
            time = timer.get_counter();
        }
    }
}

// End of file
