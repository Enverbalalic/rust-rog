use hidapi::HidDevice;

use super::AsusRogVendorRequest;

pub struct GetProfileAndFirmwareV;

impl AsusRogVendorRequest<()> for GetProfileAndFirmwareV {
    fn as_byte_vec(&self) -> [u8; 64] {
        let mut buf = [0; 64];

        buf[0] = 0x12;

        buf
    }

    fn execute(&self, device: &HidDevice) -> Option<()> {
        let req_buf = self.as_byte_vec();

        device.write(&req_buf).ok().map(|_| ())
    }
}
