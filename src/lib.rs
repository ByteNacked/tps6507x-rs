//!
//! Description:
//!
//! The TPS6507x family of devices are single-chip power management ICs (PMICs) for portable
//! applications consisting of a battery charger with power path management for a single Li-Ion or
//! Li-Polymer cell. The charger can either be supplied by a USB port on pin USB or by a DC voltage
//! from a wall adapter connected to pin AC. Three highly efficient 2.25-MHz step-down converters
//! are targeted at providing the core voltage, memory,and I/O voltage in a processor-based system.
//! The step-down converters enter a low power mode at light load for maximum efficiency across the
//! widest possible range of load currents.
//!
use embedded_hal::blocking::i2c::{Write, WriteRead};

pub mod adc;
pub mod defs;
pub mod params;
pub mod regs;

use defs::DCDCOutputVoltage;
use regs::{adconfig, ppath1::*, *};

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

    /// Config power path control register
    pub fn set_power_path(&mut self, power_path: PowerPath) -> Tps6507xResult<(), E> {
        let reg: ppath1::PPATH1 = power_path.into();
        self.write_register_raw(Registers::PPATH1, reg.0)?;
        Ok(())
    }

    /// Get USB power enable flag
    pub fn usb_power(&mut self) -> Tps6507xResult<bool, E> {
        let reg = self.read_register_raw(Registers::PPATH1)?;
        Ok(ppath1::PPATH1(reg).usb_power())
    }

    /// Get AC power enable flag
    pub fn acc_power(&mut self) -> Tps6507xResult<bool, E> {
        let reg = self.read_register_raw(Registers::PPATH1)?;
        Ok(ppath1::PPATH1(reg).ac_power())
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
    pub fn set_dcdc1(&mut self, voltage: DCDCOutputVoltage) -> Tps6507xResult<(), E> {
        self.write_register_raw(Registers::DEFDCDC1, voltage as u8)?;
        Ok(())
    }

    /// The output voltage for DCDC2 is switched between the value defined in DEFDCDC2_LOW and
    /// DEFDCDC2_HIGH depending on the status of the DEFDCDC2 pin. If DEFDCDC2 is LOW the value in
    /// DEFDCDC2_LOW is selected, if DEFDCDC2 = HIGH, the value in DEFDCDC2_HIGH is selected.
    pub fn set_dcdc2_high(&mut self, voltage: DCDCOutputVoltage) -> Tps6507xResult<(), E> {
        self.write_register_raw(Registers::DEFDCDC2_HIGH, voltage as u8)?;
        Ok(())
    }

    /// The output voltage for DCDC3 is switched between the value defined in DEFDCDC3_LOW and
    /// DEFDCDC3_HIGH depending on the status of the DEFDCDC3 pin. If DEFDCDC3 is LOW the value in
    /// DEFDCDC3_LOW is selected, if DEFDCDC3 = HIGH, the value in DEFDCDC3_HIGH is selected.
    pub fn set_dcdc3_high(&mut self, voltage: DCDCOutputVoltage) -> Tps6507xResult<(), E> {
        self.write_register_raw(Registers::DEFDCDC3_HIGH, voltage as u8)?;
        Ok(())
    }

    /// The DEFLDO2 register is used to set the output voltage of LDO2
    pub fn set_ldo2_voltage(&mut self, voltage: DCDCOutputVoltage) -> Tps6507xResult<(), E> {
        self.write_register_raw(Registers::DEFLDO2, voltage as u8)?;
        Ok(())
    }

    /// Asserts/deasserts reference voltage LDO (pin BYPASS) for ADC
    pub fn set_adc_vref(&mut self, enable: bool) -> Tps6507xResult<(), E> {
        let mut reg = adconfig::ADCONFIG(self.read_register_raw(Registers::ADCONFIG)?);
        reg.set_vref_enable(enable);
        self.write_register_raw(Registers::ADCONFIG, reg.0)?;
        Ok(())
    }

    pub fn write_register_raw(&mut self, register: Registers, value: u8) -> Result<(), E> {
        self.i2c
            .write(SLAVE_ADDR, &[register as u8, value])
            .map_err(|e| e.into())
    }

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

#[cfg(test)]
mod tests {
    use crate::defs::DCDCOutputVoltage;
    use crate::regs::{chgconfig::*, defldo::*};

    #[test]
    fn test_bitfield_ldo2() {
        let reg = DEFLDO2(DCDCOutputVoltage::V3_300 as u8);
        //println!("{:x?}", reg);
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
