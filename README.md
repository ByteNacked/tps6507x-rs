# [WIP] Rust TPS6507x series Texas Instruments power management IC

This is a platform agnostic Rust driver for the TPS6507x series
Texas Instruments power management ICs, based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

## The devices

The TPS6507x family of devices are single-chip power management ICs (PMICs) for
portable applications consisting of a battery charger with power path
management for a single Li-Ion or Li-Polymer cell. The charger can either be
supplied by a USB port on pin USB or by a DC voltage from a wall adapter
connected to pin AC. Three highly efficient 2.25-MHz step-down converters are
targeted at providing the core voltage, memory,and I/O voltage in a
processor-based system. The step-down converters enter a low power mode at
light load for maximum efficiency across the widest possible range of load
currents.


Datasheets:
- [TPS6507x](http://www.ti.com/lit/ds/symlink/ads1015.pdf)

## Usage
This crate is **WIP**

**NOT** intended for any *serious* usage.

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/buttnaked/tps6507x/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

