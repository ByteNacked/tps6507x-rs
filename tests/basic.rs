use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use tps6507x::ChargerConfig;
use tps6507x::DCDCVoltage;
use tps6507x::Tps6507x;

#[test]
fn test() {
    let expectations = [
        I2cTransaction::write(tps6507x::SLAVE_ADDR, vec![0x10, 0x2D]),
        I2cTransaction::write(tps6507x::SLAVE_ADDR, vec![0x12, 0x3F]),
        I2cTransaction::write(tps6507x::SLAVE_ADDR, vec![0x14, 0x1F]),
        I2cTransaction::write(tps6507x::SLAVE_ADDR, vec![0x17, 0x3F]),
        I2cTransaction::write(tps6507x::SLAVE_ADDR, vec![0x04, 0x30]),
        // Poll Usb connected
        I2cTransaction::write_read(tps6507x::SLAVE_ADDR, vec![0x01], vec![0b1000_1101]),
        // Poll Usb disconnected
        I2cTransaction::write_read(tps6507x::SLAVE_ADDR, vec![0x01], vec![0b0000_1101]),
    ];
    let i2c = I2cMock::new(&expectations);

    let mut tps = Tps6507x::new(i2c);
    tps.set_dcdc1(DCDCVoltage::V2_200).unwrap();
    tps.set_dcdc2_high(DCDCVoltage::V3_300).unwrap();
    tps.set_dcdc3_high(DCDCVoltage::V1_500).unwrap();
    tps.set_ldo2(DCDCVoltage::V3_300).unwrap();
    tps.set_charger_config(ChargerConfig {
        charger_enable: false,
        ..Default::default()
    })
    .unwrap();

    assert_eq!(tps.usb_power().unwrap(), true);
    assert_eq!(tps.usb_power().unwrap(), false);

    let mut i2c = tps.destroy();
    i2c.done();
}
