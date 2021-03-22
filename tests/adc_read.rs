use embedded_hal::adc::OneShot;
use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use tps6507x::channel;
use tps6507x::Tps6507x;

#[test]
fn test() {
    let expectations = [
        // set_adc_vref(true)
        I2cTransaction::write_read(tps6507x::SLAVE_ADDR, vec![0x07], vec![0b0010_0000]),
        I2cTransaction::write(tps6507x::SLAVE_ADDR, vec![0x07, 0b0011_0000]),
        // poll till conversion finished
        I2cTransaction::write_read(tps6507x::SLAVE_ADDR, vec![0x07], vec![0b0011_0000]),
        // enable adc, set channel and start measure
        I2cTransaction::write(tps6507x::SLAVE_ADDR, vec![0x07, 0b1101_1001]),
        // read conversion results
        I2cTransaction::write_read(tps6507x::SLAVE_ADDR, vec![0x09], vec![0xA5]),
        I2cTransaction::write_read(tps6507x::SLAVE_ADDR, vec![0x0A], vec![0x02]),
    ];
    let i2c = I2cMock::new(&expectations);

    let mut tps = Tps6507x::new(i2c);
    tps.set_adc_vref(true).unwrap();
    let sample = tps.read(&mut channel::VoltageBatPins_VrefOn).unwrap();

    assert_eq!(sample, (0x02 << 8) | 0xA5);

    let mut i2c = tps.destroy();
    i2c.done();
}
