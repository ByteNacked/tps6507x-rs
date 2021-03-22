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
#![no_std]

mod defs;
mod device;
mod oneshot;
mod regs;

pub use defs::*;
pub use device::{Tps6507x, Tps6507xError, Tps6507xResult, SLAVE_ADDR};
pub use regs::chgconfig::{ChargerConfig, SafetyTimerTimeOut, SensorType};
pub use regs::ppath::{AcInputCurrent, PowerPath, UsbInputCurrent};

pub mod channel {
    pub use crate::oneshot::channel::*;
}
