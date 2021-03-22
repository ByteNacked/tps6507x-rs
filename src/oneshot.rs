use embedded_hal::adc;
use embedded_hal::blocking::i2c;

use crate::device::Tps6507x;
use crate::regs::{self, Registers};

impl<I2C, E, CH> adc::OneShot<Tps6507x<I2C>, u16, CH> for Tps6507x<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
    CH: adc::Channel<Tps6507x<I2C>, ID = regs::adc::AdcInputSelect> + channel::VrefParam,
{
    type Error = E;

    fn read(&mut self, _: &mut CH) -> nb::Result<u16, Self::Error> {
        let mut reg = regs::adc::ADCONFIG(self.read_register_raw(Registers::ADCONFIG)?);

        if !reg.end_of_conversion() {
            return Err(nb::Error::WouldBlock);
        }

        reg.set_ad_enable(true);
        reg.set_conversion_start(true);
        reg.set_vref_enable(CH::vref());
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

    pub struct VoltageAdIn1_VrefOn;
    pub struct VoltageAdIn2_VrefOn;
    pub struct VoltageAdIn3_VrefOn;
    pub struct VoltageAdIn4_VrefOn;
    pub struct VoltageTsPin_VrefOn;
    pub struct VoltageIsetPin_VrefOn;
    pub struct InputVoltageCharger_VrefOn;
    pub struct VoltageBatPins_VrefOn;
    pub struct VoltageAdIn5_VrefOn;
    pub struct VoltageAdIn6_VrefOn;
    pub struct VoltageAdIn7_VrefOn;
    pub struct TouchScreenAllFunc_VrefOn;
    pub struct TouchScreenXYPos_VrefOn;

    pub trait VrefParam {
        fn vref() -> bool;
    }

    macro_rules! impl_channel {
        (VREF_OFF, $CH:ident ) => {
            impl_channel!(INNER_IMPL, $CH, $CH);

            impl VrefParam for $CH {
                fn vref() -> bool {
                    false
                }
            }
        };
        (VREF_ON, $TYNAME:ident, $CH:ident) => {
            impl_channel!(INNER_IMPL, $TYNAME, $CH);

            impl VrefParam for $TYNAME {
                fn vref() -> bool {
                    true
                }
            }
        };
        (INNER_IMPL, $TYNAME:ident, $CH:ident) => {
            impl<I2C> adc::Channel<crate::Tps6507x<I2C>> for $TYNAME {
                type ID = regs::adc::AdcInputSelect;

                fn channel() -> Self::ID {
                    Self::ID::$CH
                }
            }
        };
    }

    impl_channel!(VREF_OFF, VoltageAdIn1);
    impl_channel!(VREF_OFF, VoltageAdIn2);
    impl_channel!(VREF_OFF, VoltageAdIn3);
    impl_channel!(VREF_OFF, VoltageAdIn4);
    impl_channel!(VREF_OFF, VoltageTsPin);
    impl_channel!(VREF_OFF, VoltageIsetPin);
    impl_channel!(VREF_OFF, InputVoltageCharger);
    impl_channel!(VREF_OFF, VoltageBatPins);
    impl_channel!(VREF_OFF, VoltageAdIn5);
    impl_channel!(VREF_OFF, VoltageAdIn6);
    impl_channel!(VREF_OFF, VoltageAdIn7);
    impl_channel!(VREF_OFF, TouchScreenAllFunc);
    impl_channel!(VREF_OFF, TouchScreenXYPos);

    impl_channel!(VREF_ON, VoltageAdIn1_VrefOn, VoltageAdIn1);
    impl_channel!(VREF_ON, VoltageAdIn2_VrefOn, VoltageAdIn2);
    impl_channel!(VREF_ON, VoltageAdIn3_VrefOn, VoltageAdIn3);
    impl_channel!(VREF_ON, VoltageAdIn4_VrefOn, VoltageAdIn4);
    impl_channel!(VREF_ON, VoltageTsPin_VrefOn, VoltageTsPin);
    impl_channel!(VREF_ON, VoltageIsetPin_VrefOn, VoltageIsetPin);
    impl_channel!(VREF_ON, InputVoltageCharger_VrefOn, InputVoltageCharger);
    impl_channel!(VREF_ON, VoltageBatPins_VrefOn, VoltageBatPins);
    impl_channel!(VREF_ON, VoltageAdIn5_VrefOn, VoltageAdIn5);
    impl_channel!(VREF_ON, VoltageAdIn6_VrefOn, VoltageAdIn6);
    impl_channel!(VREF_ON, VoltageAdIn7_VrefOn, VoltageAdIn7);
    impl_channel!(VREF_ON, TouchScreenAllFunc_VrefOn, TouchScreenAllFunc);
    impl_channel!(VREF_ON, TouchScreenXYPos_VrefOn, TouchScreenXYPos);
}
