use std::ops::Deref;

use hidapi::HidDevice;

use super::AsusRogVendorRequest;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum LedType {
    Logo = 0,
    Wheel = 1,
    Bottom = 2,
    All = 3,
}

impl From<usize> for LedType {
    fn from(val: usize) -> Self {
        match val {
            0 => Self::Logo,
            1 => Self::Wheel,
            2 => Self::Bottom,
            3 => Self::All,
            _ => panic!("Unknown LedType value {val}"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum LightMode {
    Default = 0,
    Breathing = 1,
    ColorCycle = 2,
    Wave = 3,
    Reactive = 4,
    Flasher = 5,
    Battery = 6,
}

impl From<u8> for LightMode {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Default,
            1 => Self::Breathing,
            2 => Self::ColorCycle,
            3 => Self::Wave,
            4 => Self::Reactive,
            5 => Self::Flasher,
            6 => Self::Battery,
            _ => panic!("Unknown LightMode value {val}"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum LedBrightness {
    Percent0 = 0,
    Percent25 = 1,
    Percent50 = 2,
    Percent75 = 3,
    Percent100 = 4,
}

impl From<u8> for LedBrightness {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::Percent0,
            1 => Self::Percent25,
            2 => Self::Percent50,
            3 => Self::Percent75,
            4 => Self::Percent100,
            _ => panic!("Unknown LedBrightness value {val}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LedData {
    pub led_type: LedType,
    pub light_mode: LightMode,
    pub brightness: LedBrightness,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct SetLightModeRequest(pub LedData);

impl From<LedData> for SetLightModeRequest {
    fn from(data: LedData) -> Self {
        Self(data)
    }
}

impl Deref for SetLightModeRequest {
    type Target = LedData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsusRogVendorRequest<()> for SetLightModeRequest {
    fn as_byte_vec(&self) -> [u8; 64] {
        let mut buf = [0; 64];

        buf[0] = 0x51;
        buf[1] = 0x28;
        buf[2] = self.led_type as u8;
        buf[4] = self.light_mode as u8;
        buf[5] = self.brightness as u8;
        buf[6] = self.r;
        buf[7] = self.g;
        buf[8] = self.b;

        buf
    }

    fn execute(&self, device: &HidDevice) -> Option<()> {
        let req_buf = self.as_byte_vec();

        device.write(&req_buf).ok().map(|_| ())
    }
}

#[test]
fn set_light_mode_request() {
    let device = crate::get_device().expect("Could not open device");

    let request = SetLightModeRequest::from(LedData {
        led_type: LedType::Logo,
        brightness: LedBrightness::Percent0,
        light_mode: LightMode::Breathing,
        r: 255,
        g: 255,
        b: 0,
    })
    .as_byte_vec();

    let res = device.write(&request);
    assert_eq!(res.is_ok(), true);
}
