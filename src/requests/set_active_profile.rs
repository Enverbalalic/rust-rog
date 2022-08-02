use hidapi::HidDevice;

use super::AsusRogVendorRequest;

pub struct SetActiveProfile {
    pub profile: u8,
}

impl AsusRogVendorRequest<()> for SetActiveProfile {
    
    fn as_byte_vec(&self) -> [u8; 64] {
        let mut buf = [0; 64];
        
        buf[0] = 0x50;
        buf[1] = 0x02;
        buf[2] = self.profile;

        buf
    }

    fn execute(&self, device: &HidDevice) -> Option<()> {
        let req_buf = self.as_byte_vec();

        device.write(&req_buf).ok().map(|_| ())
    }
}