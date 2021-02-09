use super::*;

#[derive(Debug)]
#[repr(u8)]
pub enum AdcInputSelect {
    VoltageAdIn1 = 0b0000,
    VoltageAdIn2 = 0b0001,
    VoltageAdIn3 = 0b0010,
    VoltageAdIn4 = 0b0011,
    VoltageTsPin = 0b0100,
    VoltageIsetPin = 0b0101,
    InputVoltageCharger = 0b1000,
    VoltageBatPins = 0b1001,
    VoltageAdIn5 = 0b1010,
    VoltageAdIn6 = 0b1011,
    VoltageAdIn7 = 0b1100,
    TouchScreenAllFunc = 0b1110,
    TouchScreenXYPos = 0b1111,
}

impl From<u8> for AdcInputSelect {
    fn from(v: u8) -> Self {
        use AdcInputSelect::*;
        match v {
            0b0000 => VoltageAdIn1,
            0b0001 => VoltageAdIn2,
            0b0010 => VoltageAdIn3,
            0b0011 => VoltageAdIn4,
            0b0100 => VoltageTsPin,
            0b0101 => VoltageIsetPin,
            0b1000 => InputVoltageCharger,
            0b1001 => VoltageBatPins,
            0b1010 => VoltageAdIn5,
            0b1011 => VoltageAdIn6,
            0b1100 => VoltageAdIn7,
            0b1110 => TouchScreenAllFunc,
            0b1111 => TouchScreenXYPos,
            _ => unreachable!(),
        }
    }
}

impl From<AdcInputSelect> for u8 {
    fn from(v: AdcInputSelect) -> Self {
        v as u8
    }
}

bitfield! {
    // 0x07
    pub struct ADCONFIG(u8);
    impl Debug;

    pub from into AdcInputSelect, input_select, set_input_select: 3, 0;
    pub vref_enable, set_vref_enable: 4;
    pub end_of_conversion, set_end_of_conversion: 5;
    pub conversion_start, set_conversion_start: 6;
    pub ad_enable, set_ad_enable: 7;
}
