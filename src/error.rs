#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum Error<E> {
    /// Failed I2C communication
    I2C(E),
    /// Failed checksum validation
    BadChecksum,
    /// Couldn't find the "magic bytes" that signal the start of a data frame
    BadMagic,
}
