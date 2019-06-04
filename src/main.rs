#![no_std]
#![no_main]

extern crate panic_halt;
extern crate stm32f4xx_hal;
use stm32f4xx_hal::prelude::*;

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m::peripheral::syst;

const LED_DELAY_MS: u32 = 100;

#[entry]
fn main() -> ! {
    asm::nop();
    let periph = stm32f4xx_hal::stm32::Peripherals::take().unwrap();
    let core_periph = stm32f4xx_hal::stm32::CorePeripherals::take().unwrap();

    let rcc = periph.RCC.constrain();
    let clocks = rcc.cfgr
        .hclk(8.mhz())
        .sysclk(168.mhz())
        .freeze();

    let mut syst = core_periph.SYST;
    syst.set_clock_source(syst::SystClkSource::Core);
    syst.set_reload(8_000_000);
    syst.clear_current();
    syst.enable_counter();
    let mut delay = stm32f4xx_hal::delay::Delay::new(syst, clocks);

    let gpiog = periph.GPIOG.split();
    let mut gpio13 = gpiog.pg13.into_push_pull_output();
    gpio13.set_high();

    loop {
        gpio13.set_high();
        delay.delay_ms(LED_DELAY_MS);
        gpio13.set_low();
        delay.delay_ms(LED_DELAY_MS);
    }
}
