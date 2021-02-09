use super::*;

#[derive(Debug)]
pub struct PowerPath {
    pub usb_power_enable: bool,
    pub usb_input_current: UsbInputCurrent,
    pub ac_power_enable: bool,
    pub ac_input_current: AcInputCurrent,
}

impl From<PowerPath> for PPATH1 {
    fn from(power_path: PowerPath) -> Self {
        let mut reg = PPATH1(0x00);
        reg.set_usb_input_current(power_path.usb_input_current as u8);
        reg.set_ac_input_current(power_path.ac_input_current as u8);
        reg.set_ac_power_disable(!power_path.ac_power_enable);
        reg.set_usb_power_disable(!power_path.usb_power_enable);
        reg
    }
}

impl Default for PowerPath {
    fn default() -> Self {
        PowerPath {
            usb_power_enable: true,
            usb_input_current: UsbInputCurrent::V500mA,
            ac_power_enable: true,
            ac_input_current: AcInputCurrent::V2500mA,
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
pub enum AcInputCurrent {
    V100mA = 0b00,
    V500mA = 0b01,
    V1300mA = 0b10,
    V2500mA = 0b11,
}

#[derive(Debug)]
#[repr(u8)]
pub enum UsbInputCurrent {
    V100mA = 0b00,
    V500mA = 0b01,
    V800mA = 0b10,
    V1300mA = 0b11,
}

bitfield! {
    // 0x01
    pub struct PPATH1(u8);
    impl Debug;

    pub usb_input_current, set_usb_input_current: 1, 0;
    pub ac_input_current, set_ac_input_current: 3, 2;
    pub ac_disable, set_ac_power_disable: 4;
    pub usb_power_disable, set_usb_power_disable: 5;
    pub ac_power, _: 6;
    pub usb_power, _: 7;
}
