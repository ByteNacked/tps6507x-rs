#![allow(non_camel_case_types)]

use bitfield::bitfield;

use crate::defs::*;

pub mod adc;
pub mod chgconfig;
pub mod ctrl;
pub mod dcdc;
pub mod int;
pub mod ldo;
pub mod pgood;
pub mod ppath;
pub mod tscmode;
pub mod wled;

#[repr(u8)]
pub enum Registers {
    /// Power Path Controls
    PPATH1 = 0x01,
    /// Interrupt Reporting and Masking
    INT = 0x02,
    /// Battery Charger Configuration
    CHGCONFIG0 = 0x03,
    /// Battery Charger Configuration
    CHGCONFIG1 = 0x04,
    /// Battery Charger Configuration
    CHGCONFIG2 = 0x05,
    /// Battery Charger Configuration
    CHGCONFIG3 = 0x06,
    /// ADC Configuration and Control
    ADCONFIG = 0x07,
    /// Touch Screen Interface Control
    TSCMODE = 0x08,
    /// ADC Result LSBs
    ADRESULT_1 = 0x09,
    /// ADC Result MSBs
    ADRESULT_2 = 0x0A,
    /// Power Good Reporting
    PGOOD = 0x0B,
    /// Power Good Masking
    PGOODMASK = 0x0C,
    /// Sequence and Enable Control Bits for DCDCs and LDOs
    CON_CTRL1 = 0x0D,
    /// Control Bits for Timers,UVLO,and DCDC2/ DCDC3
    CON_CTRL2 = 0x0E,
    /// Discharge Resistors and Force PWM Mode
    CON_CTRL3 = 0x0F,
    /// Output Voltage Setting for DCDC1
    DEFDCDC1 = 0x10,
    /// Output Voltage Setting for DCDC2 if DEFDCDC2 is LOW
    DEFDCDC2_LOW = 0x11,
    /// Output Voltage Setting for DCDC2 if DEFDCDC2 is HIGH
    DEFDCDC2_HIGH = 0x12,
    /// Output Voltage Setting for DCDC3 if DEFDCDC3 is LOW
    DEFDCDC3_LOW = 0x13,
    /// Output Voltage Setting for DCDC3 if DEFDCDC3 is HIGH
    DEFDCDC3_HIGH = 0x14,
    /// Define Slew Rate for DCDC2 & DCDC3 DVS
    DEFSLEW = 0x15,
    /// Sequence and Output Voltage Controlf or LDOs
    LDO_CTRL1 = 0x16,
    /// Output Voltage Control for LDO2
    DEFLDO2 = 0x17,
    /// wLED Control Bits
    WLED_CTRL1 = 0x18,
    /// wLED Control Bits
    WLED_CTRL2 = 0x19,
}

#[cfg(test)]
mod tests {
    use crate::defs::DCDCVoltage;
    use crate::regs::{chgconfig::*, ldo::*};

    #[test]
    fn test_bitfield_ldo2() {
        let reg = DEFLDO2(DCDCVoltage::V3_300 as u8);
        assert_eq!(reg.0, 0x3F);
    }

    #[test]
    fn test_bitfield_chgconfig1() {
        let mut reg = CHGCONFIG1(0x00);
        reg.set_sensor_type10k(true);
        reg.set_safety_timer_enable(true);
        assert_eq!(reg.0, 0x30);
    }
}
