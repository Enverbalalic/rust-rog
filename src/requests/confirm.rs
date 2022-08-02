use hidapi::HidDevice;

use super::AsusRogVendorRequest;

pub struct RogConfirm;

impl AsusRogVendorRequest<()> for RogConfirm {
    fn as_byte_vec(&self) -> [u8; 64] {
        let mut buf = [0; 64];
        
        buf[0] = 0x50;
        buf[1] = 0x03;
        
        buf
    }

    fn execute(&self, device: &HidDevice) -> Option<()> {
        let req_buf = self.as_byte_vec();

        device.write(&req_buf).ok().map(|_| ())
    }
}
