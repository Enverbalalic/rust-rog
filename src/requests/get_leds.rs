use std::{fmt::Display, ops::Deref};

use hidapi::HidDevice;

use super::{
    set_light_mode::LedData,
    AsusRogVendorRequest,
};

pub struct GetLedsResponse(Vec<LedData>);

impl Deref for GetLedsResponse {
    type Target = Vec<LedData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for GetLedsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_string = String::new();

        for LedData {
            led_type,
            light_mode,
            brightness,
            r,
            g,
            b,
        } in &self.0
        {
            out_string += &format!("LED Type: {led_type:?}\nLight Mode: {light_mode:?}\nBrightness: {}\nColor: #{r:02x}{g:02x}{b:02x}\n\n", brightness.as_percent());
        }

        write!(f, "{}", out_string.trim())
    }
}

pub struct GetLeds;

impl AsusRogVendorRequest<GetLedsResponse> for GetLeds {
    fn as_byte_vec(&self) -> [u8; 64] {
        let mut buf = [0; 64];

        buf[0] = 0x12;
        buf[1] = 0x03;

        buf
    }

    fn execute(&self, device: &HidDevice) -> Option<GetLedsResponse> {
        let req_buf = self.as_byte_vec();

        device.write(&req_buf).ok()?;

        let mut res = [0u8; 64];

        device.read(&mut res).ok()?;

        if !res.starts_with(&req_buf[0..2]) {
            return None;
        }

        let bufs: [&[u8]; 3] = [
            &res[4..=8],
            &res[9..=13],
            &res[14..=18],
        ];

        let mut leds = vec![];

        for (index, buf) in bufs.iter().enumerate() {
            leds.push(LedData {
                led_type: index.into(),
                light_mode: buf[0].into(),
                brightness: buf[1].into(),
                r: buf[2],
                g: buf[3],
                b: buf[4],
            })
        }

        Some(GetLedsResponse(leds))
    }
}
