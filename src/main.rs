#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
// use port_expander::dev::pcal9714;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
// use embedded_hal::spi::SpiDevice;
use embassy_nrf::{
    bind_interrupts,
    gpio::{self, AnyPin, Input, Level, Output, OutputDrive},
    peripherals, spim,
    spis::{self, MODE_0},
};
// use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use core::cell::RefCell;
use embassy_sync::blocking_mutex::{NoopMutex, raw::NoopRawMutex};
use embassy_sync::mutex::Mutex;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    // Communication for IO Expander
    SERIAL2 => spim::InterruptHandler<peripherals::SERIAL2>;
});

// static SPI_BUS: StaticCell<Mutex<NoopRawMutex, spim::Spim>> = StaticCell::new();
static SPI_BUS: StaticCell<NoopMutex<RefCell<spim::Spim>>> = StaticCell::new();

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

    // let spi_bus = Mutex::new(spim);
    // let spi_bus = SPI_BUS.init(spi_bus);
    let spi_bus = NoopMutex::new(RefCell::new(spim));
    let spi_bus = SPI_BUS.init(spi_bus);
    let cs_pin1 = Output::new(p.P0_27, Level::High, OutputDrive::Standard);
    let spi_dev1 = SpiDevice::new(spi_bus, cs_pin1);

    let pcal9714 = port_expander::PCAL9714::new_PCAL9714(spi_dev1, false);

    loop {
        defmt::info!("Blink");
        Timer::after(Duration::from_millis(100)).await;
    }
}
