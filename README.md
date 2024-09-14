# Plantower PMSA003I Driver for Embedded HAL

Driver for the I2C connected 
[Plantower PMSA003I](https://learn.adafruit.com/pmsa003i/) particulate senmsor.

Uses [embedded-hal](https://github.com/rust-embedded/embedded-hal) in order to
be platform agnostic. It also works in `no_std` environments.

Returns all data as per the [data sheet](https://cdn-shop.adafruit.com/product-files/4632/4505_PMSA003I_series_data_manual_English_V2.6.pdf)

## Features

- Blocking operation
- Async operation if the `async` feature is specified

## Example
See [Pmsa003i] for an example of how to use the library.

## Got inspiration and direction from the following... thank you!

- [SHT4X](https://crates.io/crates/sht4x)
- [SEN0177](https://crates.io/crates/sen0177)
- [Adafruit PM25AQI](https://github.com/adafruit/Adafruit_PM25AQI/)