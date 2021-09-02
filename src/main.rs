#![no_main]
#![no_std]

extern crate panic_halt;

use cc1101::{Cc1101, RadioMode};
use cortex_m as _;
use cortex_m_rt::entry;

use embedded_hal::digital::v2::OutputPin;
use hal::prelude::*;
use hal::spi::Spi;
use hal::stm32;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as hal;

use funksteckdose::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let mut led = {
        let gpioc = p.GPIOC.split();
        gpioc.pc13.into_push_pull_output()
    };

    let rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let gpioa = p.GPIOA.split();
    let key = gpioa.pa0.into_pull_up_input();
    // gdo0 is TX pin. gdo2 can be used for RX.
    let mut gdo0 = gpioa.pa3.into_push_pull_output();
    let spi = {
        let mode0 = hal::spi::Mode {
            polarity: hal::spi::Polarity::IdleLow,
            phase: hal::spi::Phase::CaptureOnFirstTransition,
        };

        let sck = gpioa.pa5.into_alternate_af5();
        let miso = gpioa.pa6.into_alternate_af5();
        let mosi = gpioa.pa7.into_alternate_af5();

        Spi::spi1(
            p.SPI1,
            (sck, miso, mosi),
            mode0,
            50_000u32.hz(),
            clocks.clone(),
        )
    };
    let ss = gpioa.pa4.into_push_pull_output();
    let ss_pin = ::embedded_hal::digital::v1_compat::OldOutputPin::new(ss);

    // Set up radio
    let mut cc1101 = Cc1101::new(spi, ss_pin).unwrap();
    cc1101.set_defaults().unwrap();
    cc1101.set_frequency(433_920_000u64).unwrap();
    cc1101
        .set_modulation(cc1101::Modulation::OnOffKeying)
        .unwrap();
    cc1101.set_radio_mode(RadioMode::Transmit).unwrap();

    // Set up funksteckdose.
    let mut fsd: funksteckdose::Funksteckdose<
        _,
        _,
        funksteckdose::EncodingB,
        funksteckdose::Protocol1,
    > = funksteckdose::Funksteckdose::new_with_delay(&mut gdo0, &mut delay, 10);

    // Set the switch we want to toggle by radio
    let group = "D"; // group 4
    let device: Device = Device::D; // Button 4

    rprintln!("Entering send loop. Press button to send.");

    // state of key
    let mut state = funksteckdose::State::Off;

    loop {
        if key.is_low().unwrap() {
            // toggle wantState and update LED status
            state = if state == funksteckdose::State::On {
                rprintln!("Sending ON command");
                led.set_low().unwrap();
                funksteckdose::State::Off
            } else {
                rprintln!("Sending OFF command");
                led.set_high().unwrap();
                funksteckdose::State::On
            };

            fsd.send(group, &device, &state).expect("Failed to send");

            rprintln!("Sent command");
        }
    }
}
