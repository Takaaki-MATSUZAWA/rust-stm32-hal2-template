#![no_main]
#![no_std]

use cortex_m::{self, asm::delay, delay, prelude::_embedded_hal_blocking_delay_DelayMs};
use cortex_m_rt::entry;
use panic_halt as _;

use stm32_hal2::{
    clocks::Clocks,
    gpio::{GpioB, PinMode, OutputType},
    low_power,
    pac,
    timer::{Timer, TimerInterrupt},
};

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = pac::Peripherals::take().unwrap();

    let clock_cfg = Clocks::default();
    clock_cfg.setup(&mut dp.RCC, &mut dp.FLASH).unwrap();

    let mut gpiob = GpioB::new(dp.GPIOB, &mut dp.RCC);
    let mut led = gpiob.new_pin(8, PinMode::Output);
    
    let mut count = 0;

    loop {
        if count%2 == 1 {
            led.set_high();
        }else{
            led.set_low();
        }

        count += 1;
        delay(170_000_000);
    }
}
