#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
// use port_expander::dev::pcal9714;
use embassy_nrf::{
    bind_interrupts, peripherals, spim,
    spis::{self, MODE_0},
};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    // Communication for IO Expander
    SERIAL2 => spim::InterruptHandler<peripherals::SERIAL2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_nrf::init(Default::default());

    let mut config_spim = spim::Config::default();
    config_spim.frequency = spim::Frequency::M1;
    config_spim.mode = MODE_0;
    config_spim.bit_order = spim::BitOrder::MSB_FIRST;

    defmt::info!("Initialising SPI");

    let sck = unsafe { p.P0_12.clone_unchecked() };
    let mosi = unsafe { p.P0_14.clone_unchecked() };
    let miso = unsafe { p.P0_16.clone_unchecked() };

    let mut spim = spim::Spim::new(p.SERIAL2, Irqs, sck, miso, mosi, config_spim);

    // let pcal9714 = port_expander::PCAL9714::new_PCAL9714(spim, false);

    loop {
        defmt::info!("Blink");
        Timer::after(Duration::from_millis(100)).await;
    }
}
