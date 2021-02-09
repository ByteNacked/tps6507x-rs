use embedded_hal::adc;
use embedded_hal::blocking::i2c;

use crate::device::Tps6507x;
use crate::regs::{self, Registers};

impl<I2C, E, CH> adc::OneShot<Tps6507x<I2C>, u16, CH> for Tps6507x<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
    CH: adc::Channel<Tps6507x<I2C>, ID = regs::adc::AdcInputSelect>,
{
    type Error = E;

    fn read(&mut self, _: &mut CH) -> nb::Result<u16, Self::Error> {
        let mut reg = regs::adc::ADCONFIG(self.read_register_raw(Registers::ADCONFIG)?);

        if !reg.end_of_conversion() {
            return Err(nb::Error::WouldBlock);
        }

        reg.set_ad_enable(true);
        reg.set_conversion_start(true);
        reg.set_input_select(CH::channel());
        reg.set_end_of_conversion(false);
        self.write_register_raw(Registers::ADCONFIG, reg.0)?;

        let lowb = self.read_register_raw(Registers::ADRESULT_1)?;
        let highb = self.read_register_raw(Registers::ADRESULT_2)?;

        Ok(lowb as u16 | ((highb as u16) << 8))
    }
}

pub mod channel {
    #![allow(non_camel_case_types)]

    use super::*;

    pub struct VoltageAdIn1;
    pub struct VoltageAdIn2;
    pub struct VoltageAdIn3;
    pub struct VoltageAdIn4;
    pub struct VoltageTsPin;
    pub struct VoltageIsetPin;
    pub struct InputVoltageCharger;
    pub struct VoltageBatPins;
    pub struct VoltageAdIn5;
    pub struct VoltageAdIn6;
    pub struct VoltageAdIn7;
    pub struct TouchScreenAllFunc;
    pub struct TouchScreenXYPos;

    macro_rules! impl_channel {
        ( $CH:ident ) => {
            impl<I2C> adc::Channel<crate::Tps6507x<I2C>> for $CH {
                type ID = regs::adc::AdcInputSelect;

                fn channel() -> Self::ID {
                    Self::ID::$CH
                }
            }
        };
    }

    impl_channel!(VoltageAdIn1);
    impl_channel!(VoltageAdIn2);
    impl_channel!(VoltageAdIn3);
    impl_channel!(VoltageAdIn4);
    impl_channel!(VoltageTsPin);
    impl_channel!(VoltageIsetPin);
    impl_channel!(InputVoltageCharger);
    impl_channel!(VoltageBatPins);
    impl_channel!(VoltageAdIn5);
    impl_channel!(VoltageAdIn6);
    impl_channel!(VoltageAdIn7);
    impl_channel!(TouchScreenAllFunc);
    impl_channel!(TouchScreenXYPos);
}
