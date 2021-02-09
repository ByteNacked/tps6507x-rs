use super::*;

bitfield! {
    // 0x10
    pub struct DEFDCDC1(u8);
    impl Debug;

    pub from into DCDCVoltage, dcdc1, set_dcdc1: 5, 0;
    pub extadj, set_extadj: 7;
}

bitfield! {
    // 0x12
    pub struct DEFDCDC2_HIGH(u8);
    impl Debug;

    pub from into DCDCVoltage, dcdc2, set_dcdc2: 5, 0;
    pub extadj, set_extadj: 7;
}

bitfield! {
    // 0x14
    pub struct DEFDCDC3_HIGH(u8);
    impl Debug;

    pub from into DCDCVoltage, dcdc3, set_dcdc3: 5, 0;
    pub extadj, set_extadj: 7;
}
