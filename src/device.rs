use embedded_hal::blocking::i2c::{Write, WriteRead};

use crate::defs::*;
use crate::regs::*;

#[derive(Debug)]
pub enum Tps6507xError<E> {
    /// Performed read back mismatched with previously wrote value
    ReadBackMismatch,
    /// Propagated error from the interface
    Interface(E),
}

pub type Tps6507xResult<T, E> = Result<T, Tps6507xError<E>>;

pub struct Tps6507x<I2C> {
    i2c: I2C,
}

/// Tps6507x has single i2c slave address
pub const SLAVE_ADDR: u8 = 0x48;

impl<I2C, E> Tps6507x<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Create driver instance
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    /// Destroy driver and free interface
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    /// Config power path control register
    pub fn set_power_path(&mut self, power_path: ppath::PowerPath) -> Tps6507xResult<(), E> {
        let reg: ppath::PPATH1 = power_path.into();
        self.write_register_raw(Registers::PPATH1, reg.0)?;
        Ok(())
    }

    /// Get USB power enable flag
    pub fn usb_power(&mut self) -> Tps6507xResult<bool, E> {
        let reg = self.read_register_raw(Registers::PPATH1)?;
        Ok(ppath::PPATH1(reg).usb_power())
    }

    /// Get AC power enable flag
    pub fn acc_power(&mut self) -> Tps6507xResult<bool, E> {
        let reg = self.read_register_raw(Registers::PPATH1)?;
        Ok(ppath::PPATH1(reg).ac_power())
    }

    /// Set charger configuration
    pub fn set_charger_config(
        &mut self,
        config: chgconfig::ChargerConfig,
    ) -> Tps6507xResult<(), E> {
        let reg: chgconfig::CHGCONFIG1 = config.into();
        self.write_register_raw(Registers::CHGCONFIG1, reg.0)?;
        Ok(())
    }

    /// Assert/Deassert charger reset bit
    pub fn set_charger_reset(&mut self, assert: bool) -> Tps6507xResult<(), E> {
        let mut reg = chgconfig::CHGCONFIG1(self.read_register_raw(Registers::CHGCONFIG1)?);
        reg.set_charger_reset(assert);
        self.write_register_raw(Registers::CHGCONFIG1, reg.0)?;
        Ok(())
    }

    /// Sets the output voltage for the DCDC1 converter
    pub fn set_dcdc1(&mut self, voltage: DCDCVoltage) -> Tps6507xResult<(), E> {
        self.write_register_raw(Registers::DEFDCDC1, voltage as u8)?;
        Ok(())
    }

    /// The output voltage for DCDC2 is switched between the value defined in DEFDCDC2_LOW and
    /// DEFDCDC2_HIGH depending on the status of the DEFDCDC2 pin. If DEFDCDC2 is LOW the value in
    /// DEFDCDC2_LOW is selected, if DEFDCDC2 = HIGH, the value in DEFDCDC2_HIGH is selected.
    pub fn set_dcdc2_high(&mut self, voltage: DCDCVoltage) -> Tps6507xResult<(), E> {
        self.write_register_raw(Registers::DEFDCDC2_HIGH, voltage as u8)?;
        Ok(())
    }

    /// The output voltage for DCDC3 is switched between the value defined in DEFDCDC3_LOW and
    /// DEFDCDC3_HIGH depending on the status of the DEFDCDC3 pin. If DEFDCDC3 is LOW the value in
    /// DEFDCDC3_LOW is selected, if DEFDCDC3 = HIGH, the value in DEFDCDC3_HIGH is selected.
    pub fn set_dcdc3_high(&mut self, voltage: DCDCVoltage) -> Tps6507xResult<(), E> {
        self.write_register_raw(Registers::DEFDCDC3_HIGH, voltage as u8)?;
        Ok(())
    }

    /// Sets output voltage of LDO1
    pub fn set_ldo1(&mut self, voltage: LDO1Voltage) -> Tps6507xResult<(), E> {
        let mut reg = ldo::LDO_CTRL1(self.read_register_raw(Registers::LDO_CTRL1)?);
        reg.set_ldo1(voltage);

        self.write_register_raw(Registers::LDO_CTRL1, reg.0)?;
        Ok(())
    }

    /// The DEFLDO2 register is used to set the output voltage of LDO2
    pub fn set_ldo2(&mut self, voltage: DCDCVoltage) -> Tps6507xResult<(), E> {
        self.write_register_raw(Registers::DEFLDO2, voltage as u8)?;
        Ok(())
    }

    /// Asserts/deasserts reference voltage LDO (pin BYPASS) for ADC
    pub fn set_adc_vref(&mut self, enable: bool) -> Tps6507xResult<(), E> {
        let mut reg = adc::ADCONFIG(self.read_register_raw(Registers::ADCONFIG)?);
        reg.set_vref_enable(enable);
        self.write_register_raw(Registers::ADCONFIG, reg.0)?;
        Ok(())
    }
    
    /// Raw register write access
    pub fn write_register_raw(&mut self, register: Registers, value: u8) -> Result<(), E> {
        self.i2c
            .write(SLAVE_ADDR, &[register as u8, value])
    }

    /// Raw register read access
    pub fn read_register_raw(&mut self, register: Registers) -> Result<u8, E> {
        let mut buf = [0u8];
        self.i2c
            .write_read(SLAVE_ADDR, &[register as u8], &mut buf)?;
        Ok(buf[0])
    }
}

impl<E> From<E> for Tps6507xError<E> {
    fn from(e: E) -> Self {
        Self::Interface(e)
    }
}

// Hack impl to prevent nested `nb::Error<nb::Error<E>>` error types,
// due to unfourtunate adc::OneShot trait interface
impl<E> From<nb::Error<nb::Error<E>>> for Tps6507xError<nb::Error<E>> {
    fn from(e: nb::Error<nb::Error<E>>) -> Self {
        match e {
            nb::Error::WouldBlock => Self::Interface(nb::Error::WouldBlock),
            nb::Error::Other(nb) => Self::Interface(nb),
        }
    }
}
