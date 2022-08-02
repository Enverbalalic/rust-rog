use hidapi::HidDevice;

use super::{AsusRogVendorRequest};

#[derive(Debug, Clone, Copy)]
pub struct DpiDataResponse {
    pub dpi1: usize,
    pub dpi2: usize,
    pub dpi3: usize,
    pub dpi4: usize,
}

pub struct GetDpiData;

impl AsusRogVendorRequest<DpiDataResponse> for GetDpiData {
    
    fn as_byte_vec(&self) -> [u8; 64] {
        let mut buf = [0; 64];

        buf[0] = 0x12;
        buf[1] = 0x04;

        buf
    }

    fn execute(&self, device: &HidDevice) -> Option<DpiDataResponse> {
        let req_buf = self.as_byte_vec();
        device.write(&req_buf).ok()?;

        let mut buf: [u8; 64] = [0; 64];
        device.read(&mut buf).ok()?;

        if !buf.starts_with(&req_buf[0..2]) {
            return None;
        }

        Some(DpiDataResponse {
            dpi1: (buf[4] as usize * 50 + 50) * 2,
            dpi2: (buf[6] as usize * 50 + 50) * 2,
            dpi3: (buf[8] as usize * 50 + 50) * 2,
            dpi4: (buf[10] as usize * 50 + 50) * 2,
        })
    }
}

#[test]
fn test_get_dpi_data() {
    let device = crate::get_device().expect("Could not open device");

    // Write data to device
    let res = GetDpiData.execute(&device);

    assert_eq!(res.is_some(), true);
}
