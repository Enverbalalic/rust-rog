use hidapi::HidDevice;

use super::{set_light_mode::{LedData}, AsusRogVendorRequest};

pub struct GetLeds;

impl AsusRogVendorRequest<Vec<LedData>> for GetLeds {
    fn as_byte_vec(&self) -> [u8; 64] {
        let mut buf = [0; 64];

        buf[0] = 0x12;
        buf[1] = 0x03;

        buf
    }

    fn execute(&self, device: &HidDevice) -> Option<Vec<LedData>> {
        let req_buf = self.as_byte_vec();

        device.write(&req_buf).ok()?;

        let mut res = [0u8; 64];

        device.read(&mut res).ok()?;

        if !res.starts_with(&req_buf[0..2]) {
            return None;
        }

        let bufs: [&[u8]; 4] = [
            &req_buf[4..=8],
            &req_buf[9..=13],
            &req_buf[14..=18],
            &req_buf[19..=23],
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

        Some(leds)
    }
}
