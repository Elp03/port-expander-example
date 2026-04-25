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

    let pins = pcal9714.split();
    defmt::info!("Split succsessfully");
    Timer::after_secs(1).await;

    let mut pin0_0_inn = pins.gp0_0;
    let mut pin0_1 = pins.gp0_1.into_output().expect("Pin 0.1 failed");
    let mut pin0_1_inn = pin0_1.into_input().expect("did not work");
    let mut pin0_2 = pins.gp0_2.into_output().expect("Pin 0.2 failed");
    let mut pin0_2_inn = pin0_2.into_input().expect("did not work");
    let mut pin0_3 = pins.gp0_3.into_output().expect("Pin 0.3 failed");
    let mut pin0_3_inn = pin0_3.into_input().expect("did not work");
    let mut pin0_4 = pins.gp0_4.into_output().expect("Pin 0.4 failed");
    let mut pin0_4_inn = pin0_4.into_input().expect("did not work");
    let mut pin0_5 = pins.gp0_5.into_output().expect("Pin 0.5 failed");
    let mut pin0_5_inn = pin0_5.into_input().expect("did not work");
    let mut pin0_6 = pins.gp0_6.into_output().expect("Pin 0.6 failed");
    let mut pin0_6_inn = pin0_6.into_input().expect("did not work");
    let mut pin0_7 = pins.gp0_7.into_output().expect("Pin 0.7 failed");
    let mut pin0_7_inn = pin0_7.into_input().expect("did not work");
    let mut pin1_0 = pins.gp1_0.into_output().expect("Pin 1.0 failed");
    let mut pin1_0_inn = pin1_0.into_input().expect("did not work");
    let mut pin1_1 = pins.gp1_1.into_output().expect("Pin 1.1 failed");
    let mut pin1_1_inn = pin1_1.into_input().expect("did not work");
    let mut pin1_2 = pins.gp1_2.into_output().expect("Pin 1.2 failed");
    let mut pin1_2_inn = pin1_2.into_input().expect("did not work");
    let mut pin1_3 = pins.gp1_3.into_output().expect("Pin 1.3 failed");
    let mut pin1_3_inn = pin1_3.into_input().expect("did not work");
    let mut pin1_4 = pins.gp1_4.into_output().expect("Pin 1.4 failed");
    let mut pin1_4_inn = pin1_4.into_input().expect("did not work");
    let mut pin1_5 = pins.gp1_5.into_output().expect("Pin 1.4 failed");
    let mut pin1_5_inn = pin1_5.into_input().expect("failed");
    defmt::info!("All pins initialized");
    Timer::after_secs(1).await;

    loop {
        defmt::info!(
            " | {} | {:?} | {:#?} | {:#?} | {:#?} | {:#?} | {:#?} | {:#?} | {:#?} | {:#?} | {:#?} | {:#?} | {:#?} | {:#?} |",
            pin0_0_inn.is_high().expect("help"),
            pin0_1_inn.is_high().expect("Help"),
            pin0_2_inn.is_high().expect("Help"),
            pin0_3_inn.is_high().expect("Help"),
            pin0_4_inn.is_high().expect("Help"),
            pin0_5_inn.is_high().expect("Help"),
            pin0_6_inn.is_high().expect("Help"),
            pin0_7_inn.is_high().expect("Help"),
            pin1_0_inn.is_high().expect("Help"),
            pin1_1_inn.is_high().expect("Help"),
            pin1_2_inn.is_high().expect("Help"),
            pin1_3_inn.is_high().expect("Help"),
            pin1_4_inn.is_high().expect("Help"),
            pin1_5_inn.is_high().expect("Help"),
        );

        defmt::info!("Sett all pins high");
        Timer::after(Duration::from_millis(400)).await;
    }
}
