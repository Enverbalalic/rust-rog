use hidapi::HidDevice;

use super::AsusRogVendorRequest;

pub struct GetPowerData;

impl AsusRogVendorRequest<()> for GetPowerData {
    fn as_byte_vec(&self) -> [u8; 64] {
        let mut buf = [0; 64];

        buf[0] = 0x12;
        buf[1] = 0x07;

        buf
    }

    fn execute(&self, device: &HidDevice) -> Option<()> {
        let req_buf = self.as_byte_vec();

        device.write(&req_buf).ok()?;

        let mut buf: [u8; 64] = [0; 64];

        let res = device.read(&mut buf).ok()?;

        // buf[4] = battery level? ??? 00 = 100%, 01 = 75%, 02 = 50%, 03 = 25%, 04 = 0% ?
        // buf[5] = sleep mode idle
        // buf[6] = alert ?? ( 0 = off, 01 = 25%, 02 = 50%)

        println!("{buf:#04x?}");

        None
    }
}
