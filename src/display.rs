use display_interface_spi::SPIInterface;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::spi::MODE_0;
use embedded_time::rate::*;
use hal::pac;
use hal::spi::Spi;
use log::info;
use rp2040_hal as hal;
use rp2040_hal::gpio::dynpin::DynPin;
use st7789::ST7789;

pub type RealDisplay =
    ST7789<SPIInterface<Spi<hal::spi::Enabled, pac::SPI0, 8>, DynPin, DynPin>, DynPin>;

pub struct Display {
    st7789: RealDisplay,
    backlight_pin: DynPin,
}

impl Display {
    pub fn new(
        mut backlight_pin: DynPin,
        mut lcd_dc_pin: DynPin,
        mut lcd_cs_pin: DynPin,
        mut lcd_reset_pin: DynPin,
        spi_device: pac::SPI0,
        resets: &mut pac::RESETS,
        delay_source: &mut impl DelayUs<u32>,
    ) -> Display {
        info!("Initializing display");
        backlight_pin.into_push_pull_output();
        lcd_dc_pin.into_push_pull_output();
        lcd_cs_pin.into_push_pull_output();
        lcd_reset_pin.into_push_pull_output();
        let spi = Spi::<_, _, 8>::new(spi_device).init(
            resets,
            125_000_000u32.Hz(),
            16_000_000u32.Hz(),
            &MODE_0,
        );
        let di = SPIInterface::new(spi, lcd_dc_pin, lcd_cs_pin);
        let mut st7789 = ST7789::new(di, lcd_reset_pin, 240, 240);
        st7789.init(delay_source).unwrap();
        Display {
            st7789,
            backlight_pin,
        }
    }

    pub fn draw_target(&mut self) -> &mut RealDisplay {
        &mut self.st7789
    }

    pub fn enable_backlight(&mut self) {
        self.backlight_pin.set_high().unwrap();
    }
}