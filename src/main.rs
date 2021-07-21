#![no_main]
#![no_std]

use cortex_m::{
    self,
    delay::Delay,
};
use cortex_m_rt::entry;
use panic_halt as _;

use stm32_hal2::{
    clocks::Clocks,
    gpio::{Pin, Port, PinMode},
    pac,
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut dp = pac::Peripherals::take().unwrap();

    let clock_cfg = Clocks::default();
    clock_cfg.setup(&mut dp.RCC, &mut dp.FLASH).unwrap();

    let mut led = Pin::new(Port::A, 5, PinMode::Output);

    let mut delay = Delay::new(cp.SYST, clock_cfg.systick());
    
    let mut count = 0;
    
    loop {
        if count%2 == 1 {
            led.set_high();
        }else{
            led.set_low();
        }

        count += 1;
        delay.delay_ms(500);

    }
}
