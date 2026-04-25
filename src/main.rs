#![no_std]
#![no_main]

use core::cell::RefCell;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embassy_executor::Spawner;
use embassy_nrf::{
    bind_interrupts,
    gpio::{Level, Output, OutputDrive},
    peripherals, spim,
    spis::MODE_0,
};
use embassy_sync::blocking_mutex::NoopMutex;
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    // Communication for IO Expander
    SERIAL2 => spim::InterruptHandler<peripherals::SERIAL2>;
});

static SPI_BUS: StaticCell<NoopMutex<RefCell<spim::Spim>>> = StaticCell::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let mut p = embassy_nrf::init(Default::default());

    let mut config_spim = spim::Config::default();
    config_spim.frequency = spim::Frequency::M4;
    config_spim.mode = MODE_0;
    config_spim.bit_order = spim::BitOrder::MSB_FIRST;

    defmt::info!("Starting initialising SPI");
    let sck = p.P0_12;
    let mosi = p.P0_14;
    let miso = p.P0_16;

    let mut spim = spim::Spim::new(p.SERIAL2, Irqs, sck, miso, mosi, config_spim);
    defmt::info!("SPI initialized");

    let spi_bus = NoopMutex::new(RefCell::new(spim));
    let spi_bus = SPI_BUS.init(spi_bus);
    let cs_pin1 = Output::new(p.P0_27, Level::High, OutputDrive::Standard);
    let spi_dev1 = SpiDevice::new(spi_bus, cs_pin1);
    defmt::info!("SPI Device initialized");

    let mut pcal9714 = port_expander::Pcal9714::new_pcal9714(spi_dev1, false);
    defmt::info!("Port expander initialized");

    let mut pins = pcal9714.split();
    defmt::info!("Split succsessfully");
    Timer::after_secs(1).await;

    loop {
        pins.gp0_1.enable_pull_down(true).expect("failed");
        pins.gp0_2.enable_pull_down(true).expect("failed");
        pins.gp0_3.enable_pull_down(true).expect("failed");
        pins.gp0_4.enable_pull_down(true).expect("failed");
        pins.gp0_5.enable_pull_down(true).expect("failed");
        pins.gp0_6.enable_pull_down(true).expect("failed");
        pins.gp0_7.enable_pull_down(true).expect("failed");
        pins.gp1_0.enable_pull_down(true).expect("failed");
        pins.gp1_1.enable_pull_down(true).expect("failed");
        pins.gp1_2.enable_pull_down(true).expect("failed");
        pins.gp1_3.enable_pull_down(true).expect("failed");
        pins.gp1_4.enable_pull_down(true).expect("failed");
        pins.gp1_5.enable_pull_down(true).expect("failed");
        pins.gp0_0.enable_pull_down(true).expect("failed");

        defmt::info!("Sett all pins high");
        Timer::after(Duration::from_millis(1000)).await;

        pins.gp0_1.enable_pull_down(false).expect("failed");
        pins.gp0_2.enable_pull_down(false).expect("failed");
        pins.gp0_3.enable_pull_down(false).expect("failed");
        pins.gp0_4.enable_pull_down(false).expect("failed");
        pins.gp0_5.enable_pull_down(false).expect("failed");
        pins.gp0_6.enable_pull_down(false).expect("failed");
        pins.gp0_7.enable_pull_down(false).expect("failed");
        pins.gp1_0.enable_pull_down(false).expect("failed");
        pins.gp1_1.enable_pull_down(false).expect("failed");
        pins.gp1_2.enable_pull_down(false).expect("failed");
        pins.gp1_3.enable_pull_down(false).expect("failed");
        pins.gp1_4.enable_pull_down(false).expect("failed");
        pins.gp1_5.enable_pull_down(false).expect("failed");
        pins.gp0_0.enable_pull_down(false).expect("failed");
        defmt::info!("Sett all pins low");
        Timer::after(Duration::from_millis(1000)).await;
    }
}
