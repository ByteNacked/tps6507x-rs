use super::*;

bitfield! {
    // 0x16
    pub struct LDO_CTRL1(u8);
    impl Debug;

    pub from into LDO1Voltage, ldo1, set_ldo1: 3, 0;
    pub ldo_seq, set_ldo_seq: 7, 5;
}

bitfield! {
    // 0x17
    pub struct DEFLDO2(u8);
    impl Debug;

    pub from into DCDCVoltage, ldo2, set_ldo2: 5, 0;
    pub tracking, set_tracking: 6;
}
