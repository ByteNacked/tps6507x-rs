#![allow(non_camel_case_types)]

use bitfield::bitfield;

use crate::defs::*;

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

pub mod chgconfig1 {
    use super::*;

    #[derive(Debug)]
    #[repr(u8)]
    pub enum SafetyTimerTimeOut {
        V4Hours = 0x00,
        V5Hours = 0x01,
        V6Hours = 0x10,
        V8Hours = 0x11,
    }

    impl From<u8> for SafetyTimerTimeOut {
        fn from(v: u8) -> Self {
            use SafetyTimerTimeOut::*;
            match v {
                0x00 => V4Hours,
                0x01 => V5Hours,
                0x10 => V6Hours,
                0x11 => V8Hours,
                _ => unreachable!(),
            }
        }
    }

    impl From<SafetyTimerTimeOut> for u8 {
        fn from(v: SafetyTimerTimeOut) -> Self {
            v as u8
        }
    }

    bitfield! {
        // 0x04
        pub struct CHGCONFIG1(u8);
        impl Debug;

        pub charger_enable, set_charger_enable: 0;
        pub suspend_charge, set_suspend_charge: 1;
        pub charge_termination_off, set_charge_termination_off: 2;
        pub charge_reset, set_charge_reset: 3;
        pub sensor_type10k, set_sensor_type10k: 4;
        pub safety_timer_enable, set_safety_timer_enable: 5;
        pub from into SafetyTimerTimeOut, charge_safety_timer, set_charge_safety_timer: 7, 6;
    }

    #[repr(u8)]
    pub enum BatterySensorType {
        V100K = 0x00,
        V10K = 0x01,
    }

    impl From<u8> for BatterySensorType {
        fn from(v: u8) -> Self {
            use BatterySensorType::*;
            match v {
                0x00 => V100K,
                0x01 => V10K,
                _ => unreachable!(),
            }
        }
    }
}

pub mod adconfig {
    use super::*;

    #[derive(Debug)]
    #[repr(u8)]
    pub enum AdcInputSelect {
        VoltageAt_AD_IN1 = 0b0000,
        VoltageAt_AD_IN2 = 0b0001,
        VoltageAt_AD_IN3 = 0b0010,
        VoltageAt_AD_IN4 = 0b0011,
        VoltageAt_TS_PIN = 0b0100,
        VoltageAt_ISET_PIN = 0b0101,
        InputVoltageCharger = 0b1000,
        VoltageAt_BAT_PINS = 0b1001,
        VoltageAt_AD_IN5 = 0b1010,
        VoltageAt_AD_IN6 = 0b1011,
        VoltageAt_AD_IN7 = 0b1100,
        TouchScreenAllFunc = 0b1110,
        TouchScreenXYPos = 0b1111,
    }

    impl From<u8> for AdcInputSelect {
        fn from(v: u8) -> Self {
            use AdcInputSelect::*;
            match v {
                0b0000 => VoltageAt_AD_IN1,
                0b0001 => VoltageAt_AD_IN2,
                0b0010 => VoltageAt_AD_IN3,
                0b0011 => VoltageAt_AD_IN4,
                0b0100 => VoltageAt_TS_PIN,
                0b0101 => VoltageAt_ISET_PIN,
                0b1000 => InputVoltageCharger,
                0b1001 => VoltageAt_BAT_PINS,
                0b1010 => VoltageAt_AD_IN5,
                0b1011 => VoltageAt_AD_IN6,
                0b1100 => VoltageAt_AD_IN7,
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
}

pub mod defdcd {
    use super::*;

    bitfield! {
        // 0x10
        pub struct DEFDCDC1(u8);
        impl Debug;

        pub from into DCDCOutputVoltage, dcdc1, set_dcdc1: 5, 0;
        pub extadj, set_extadj: 7;
    }

    bitfield! {
        // 0x12
        pub struct DEFDCDC2_HIGH(u8);
        impl Debug;

        pub from into DCDCOutputVoltage, dcdc2, set_dcdc2: 5, 0;
        pub extadj, set_extadj: 7;
    }

    bitfield! {
        // 0x14
        pub struct DEFDCDC3_HIGH(u8);
        impl Debug;

        pub from into DCDCOutputVoltage, dcdc3, set_dcdc3: 5, 0;
        pub extadj, set_extadj: 7;
    }
}

pub mod defldo {
    use super::*;

    bitfield! {
        // 0x17
        pub struct DEFLDO2(u8);
        impl Debug;

        pub from into DCDCOutputVoltage, ldo2, set_ldo2: 5, 0;
        pub tracking, set_tracking: 6;
    }
}
