pub mod requests;

use std::time::Duration;

use crate::requests::{
    get_dpi_data::GetDpiData,
    get_power_data::GetPowerData,
    get_profile_and_firmware_v::GetProfileAndFirmwareV,
    set_active_profile::SetActiveProfile,
    set_light_mode::{LedBrightness, LedType, LightMode, SetLightModeRequest},
    AsusRogVendorRequest, confirm::RogConfirm, get_leds::GetLeds,
};
use hidapi::HidDevice;

const VENDOR_ASUSTEK: u16 = 0x0b05;

pub trait AsByteVec {
    fn as_byte_vec(&self) -> Vec<u8>;
}

pub fn get_device() -> Option<HidDevice> {
    let api = hidapi::HidApi::new().unwrap();
    // Print out information about all connected devices
    let device_info = api
        .device_list()
        .find(|dev| dev.vendor_id() == VENDOR_ASUSTEK)?;

    let device = device_info.open_device(&api).ok()?;

    Some(device)
}

fn main() {
    let device = crate::get_device().expect("Could not open device");

    std::thread::sleep(Duration::from_millis(500));

    // let res = SetActiveProfile { profile: 0 }.execute(&device);

    // std::thread::sleep(Duration::from_millis(3000));

    // let res = RogConfirm.execute(&device);

    // std::thread::sleep(Duration::from_millis(3000));

    let res = GetLeds.execute(&device);

    println!("{res:#?}");
}
