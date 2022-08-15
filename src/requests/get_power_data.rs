use std::fmt::{Display, Formatter, Result};

use hidapi::HidDevice;

use super::AsusRogVendorRequest;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum SleepModeIdle {
    Minute1 = 0,
    Minute2 = 1,
    Minute3 = 2,
    Minute5 = 3,
    Minute10 = 4,
    NoSleep = 255,
}

impl From<u8> for SleepModeIdle {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Minute1,
            1 => Self::Minute2,
            2 => Self::Minute3,
            3 => Self::Minute5,
            4 => Self::Minute10,
            255 => Self::NoSleep,
            _ => panic!("Unknown sleep mode value {val}"),
        }
    }
}

impl Display for SleepModeIdle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                SleepModeIdle::Minute1 => "1 minute",
                SleepModeIdle::Minute2 => "2 minutes",
                SleepModeIdle::Minute3 => "3 minutes",
                SleepModeIdle::Minute5 => "5 minutes",
                SleepModeIdle::Minute10 => "10 minutes",
                SleepModeIdle::NoSleep => "Off",
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PowerResponse {
    pub battery_level: usize,
    pub battery_alert: usize,
    pub sleep_mode: SleepModeIdle,
}

impl Display for PowerResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self {
            battery_level,
            battery_alert,
            sleep_mode,
        } = self;

        write!(
            f,
            "Battery Level: {battery_level}%\nBattery Alert: {battery_alert}%\nSleep Mode: {sleep_mode}",
        )
    }
}

pub struct GetPowerData;

impl AsusRogVendorRequest<PowerResponse> for GetPowerData {
    fn as_byte_vec(&self) -> [u8; 64] {
        let mut buf = [0; 64];

        buf[0] = 0x12;
        buf[1] = 0x07;

        buf
    }

    fn execute(&self, device: &HidDevice) -> Option<PowerResponse> {
        let req_buf = self.as_byte_vec();

        device.write(&req_buf).ok()?;

        let mut buf: [u8; 64] = [0; 64];

        device.read(&mut buf).ok()?;

        if !buf.starts_with(&req_buf[0..2]) {
            return None;
        }

        Some(PowerResponse {
            battery_level: buf[4] as usize * 25,
            battery_alert: buf[6] as usize * 25,
            sleep_mode: buf[5].into(),
        })
    }
}
