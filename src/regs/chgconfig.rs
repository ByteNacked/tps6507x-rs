use super::*;

#[derive(Debug)]
pub struct ChargerConfig {
    pub charger_enable: bool,
    pub suspend_charge: bool,
    pub charge_termination: bool,
    pub charger_reset: bool,
    pub sensor_type: SensorType,
    pub safety_timer_enable: bool,
    pub charger_safety_timer_timeout: SafetyTimerTimeOut,
}

impl Default for ChargerConfig {
    fn default() -> Self {
        Self {
            charger_enable: true,
            suspend_charge: false,
            charge_termination: true,
            charger_reset: false,
            sensor_type: SensorType::V10K,
            safety_timer_enable: true,
            charger_safety_timer_timeout: SafetyTimerTimeOut::V4Hours,
        }
    }
}

impl From<ChargerConfig> for CHGCONFIG1 {
    fn from(v: ChargerConfig) -> Self {
        let mut reg = CHGCONFIG1(0x00);
        reg.set_charger_enable(v.charger_enable);
        reg.set_suspend_charge(v.suspend_charge);
        reg.set_charge_termination_off(!v.charge_termination);
        reg.set_charger_reset(v.charger_reset);
        reg.set_sensor_type10k(v.sensor_type.into());
        reg.set_safety_timer_enable(v.safety_timer_enable);
        reg.set_charge_safety_timer(v.charger_safety_timer_timeout);

        reg
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum SensorType {
    V100K = 0b0,
    V10K = 0b1,
}

impl From<SensorType> for bool {
    fn from(v: SensorType) -> Self {
        use SensorType::*;
        match v {
            V100K => false,
            V10K => true,
        }
    }
}

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
    pub charger_reset, set_charger_reset: 3;
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
