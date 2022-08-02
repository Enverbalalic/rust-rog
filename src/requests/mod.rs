use hidapi::HidDevice;

pub mod set_light_mode;
pub mod get_dpi_data;
pub mod get_profile_and_firmware_v;
pub mod get_power_data;
pub mod set_active_profile;
pub mod confirm;
pub mod get_leds;

pub trait AsusRogVendorRequest<T> {
    fn as_byte_vec(&self) -> [u8; 64];
    fn execute(&self, device: &HidDevice) -> Option<T>;
}