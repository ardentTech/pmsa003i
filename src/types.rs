use crate::RESPONSE_LENGTH;

/// A reading from the sensor
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug)]
pub struct Reading {
    /// PM1.0 concentration in μg/𝑚3
    pub pm1: u16,
    /// PM2.5 concentration in μg/𝑚3
    pub pm2_5: u16,
    /// PM10 concentration in μg/𝑚3
    pub pm10: u16,
    /// PM1.0 concentration in μg/𝑚3 under atmospheric environment
    pub env_pm1: u16,
    /// PM2.5 concentration in μg/𝑚3 under atmospheric environment
    pub env_pm2_5: u16,
    /// PM10 concentration in μg/𝑚3 under atmospheric environment
    pub env_pm10: u16,
    /// number of particles with diameter beyond 0.3 μ𝑚 in 0.1L of air
    pub particles_larger_than_0_3: u16,
    /// number of particles with diameter beyond 0.5 μ𝑚 in 0.1L of air
    pub particles_larger_than_0_5: u16,
    /// number of particles with diameter beyond 1.0 μ𝑚 in 0.1L of air
    pub particles_larger_than_1: u16,
    /// number of particles with diameter beyond 2.5 μ𝑚 in 0.1L of air
    pub particles_larger_than_2_5: u16,
    /// number of particles with diameter beyond 5 μ𝑚 in 0.1L of air
    pub particles_larger_than_5: u16,
    /// number of particles with diameter beyond 10 μ𝑚 in 0.1L of air
    pub particles_larger_than_10: u16,
}

impl From<[u8; RESPONSE_LENGTH]> for Reading {
    fn from(value: [u8; RESPONSE_LENGTH]) -> Self {
        Self {
            pm1: u16::from_be_bytes([value[4], value[5]]),
            pm2_5: u16::from_be_bytes([value[6], value[7]]),
            pm10: u16::from_be_bytes([value[8], value[9]]),
            env_pm1: u16::from_be_bytes([value[10], value[11]]),
            env_pm2_5: u16::from_be_bytes([value[12], value[13]]),
            env_pm10: u16::from_be_bytes([value[14], value[15]]),
            particles_larger_than_0_3: u16::from_be_bytes([value[16], value[17]]),
            particles_larger_than_0_5: u16::from_be_bytes([value[18], value[19]]),
            particles_larger_than_1: u16::from_be_bytes([value[20], value[21]]),
            particles_larger_than_2_5: u16::from_be_bytes([value[22], value[23]]),
            particles_larger_than_5: u16::from_be_bytes([value[24], value[25]]),
            particles_larger_than_10: u16::from_be_bytes([value[26], value[27]]),
        }
    }
}
