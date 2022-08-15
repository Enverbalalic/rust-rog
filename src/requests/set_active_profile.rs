use hidapi::HidDevice;

use super::AsusRogVendorRequest;

pub struct SetActiveProfile {
    pub profile: u8,
}

impl AsusRogVendorRequest<bool> for SetActiveProfile {
    
    fn as_byte_vec(&self) -> [u8; 64] {
        let mut buf = [0; 64];
        
        buf[0] = 0x50;
        buf[1] = 0x02;
        buf[2] = self.profile;

        buf
    }

    fn execute(&self, device: &HidDevice) -> Option<bool> {
        let req_buf = self.as_byte_vec();

        device.write(&req_buf).ok().map(|_| ())?;

        let mut res = [0;64];

        device.read(&mut res).ok()?;

        if !res.starts_with(&req_buf[0..2]) {
            return Some(false);
        }

        Some(true)
    }
}