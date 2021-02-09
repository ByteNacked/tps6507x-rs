use super::*;

bitfield! {
    // 0x17
    pub struct DEFLDO2(u8);
    impl Debug;

    pub from into DCDCVoltage, ldo2, set_ldo2: 5, 0;
    pub tracking, set_tracking: 6;
}
