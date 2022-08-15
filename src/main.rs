pub mod requests;

use std::time::Duration;

use crate::requests::{
    get_dpi_data::GetDpiData,
    get_leds::GetLeds,
    get_power_data::GetPowerData,
    set_active_profile::SetActiveProfile,
    set_light_mode::{LedBrightness, LedType, LightMode, SetLightModeRequest},
    AsusRogVendorRequest,
};
use hidapi::HidDevice;

const VENDOR_ASUSTEK: u16 = 0x0b05;

use clap::{Args, Parser, Subcommand};
use requests::set_light_mode::LedData;

#[derive(Debug, Args)]
struct SetLightModeCommand {
    /// Led type
    #[clap(arg_enum, value_parser, short = 't', long)]
    pub led_type: LedType,
    /// Light mode
    #[clap(arg_enum, value_parser, short = 'm', long)]
    pub light_mode: LightMode,
    /// Percent, allowed values are [0, 25, 50, 75, 100]
    #[clap(short, long)]
    pub brightness: usize,
    /// Hex color value eg. FF00FF
    #[clap(short, long)]
    pub color: String,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Returns the battery level (%), alert mode (%) and sleep mode
    GetPowerData,

    /// Returns the current selected profile and the firmware versions.
    GetProfileAndFirmwareVersion,

    /// Returns the current LED light mode, brightness and color of detected LEDs for this profile
    GetLeds,

    /// Returns the 4 DPI settings for the current profile
    GetDpiData,

    /// Sets the LED brightness, color and mode
    /// To turn off leds, write brightness 0, color 000000, mode default to all LEDs
    SetLightMode(SetLightModeCommand),

    /// Sets the currently active profile
    /// There are 3 different possible profiles, each with it's own settings for everything
    /// (DPI, Leds, Power settings, etc.)
    SetActiveProfile {
        /// Profile to set, valid options are [0, 1, 2]
        profile: u8,
    },
}

/// CLI tool for controlling ASUS ROG mice
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    #[clap(subcommand)]
    command: Command,
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

fn get_dpi_data(device: &HidDevice) {
    loop {
        if let Some(res) = GetDpiData.execute(device) {
            println!("{res}");
            break;
        }

        std::thread::sleep(Duration::from_millis(200));
    }
}

fn get_leds(device: &HidDevice) {
    loop {
        if let Some(res) = GetLeds.execute(device) {
            println!("{res}");
            break;
        }

        std::thread::sleep(Duration::from_millis(200));
    }
}

fn get_power_data(device: &HidDevice) {
    loop {
        if let Some(res) = GetPowerData.execute(device) {
            println!("{res}");
            break;
        }

        std::thread::sleep(Duration::from_millis(200));
    }
}

fn set_light_mode(device: &HidDevice, command: SetLightModeCommand) {
    let SetLightModeCommand {
        led_type,
        light_mode,
        brightness,
        color,
    } = command;

    let raw_color = hex::decode(color).expect("Failed to parse hex color");

    if raw_color.len() != 3 {
        panic!("Failed to parse hex color");
    }

    let r = raw_color[0];
    let g = raw_color[1];
    let b = raw_color[2];

    let request = SetLightModeRequest(LedData {
        led_type,
        light_mode,
        brightness: LedBrightness::from_percent(brightness),
        r,
        g,
        b,
    });

    loop {
        if let Some(true) = request.execute(device) {
            break;
        }
    }
}

fn set_active_profile(device: &HidDevice, profile: u8) {
    if profile > 2 {
        panic!("Invalid profile, supported values are [0, 1, 2]");
    }

    let request = SetActiveProfile { profile };

    loop {
        if let Some(true) = request.execute(device) {
            break;
        }
    }
}

fn main() {
    let args = CliArgs::parse();

    let device = get_device().expect("Could not open device");

    match args.command {
        Command::GetDpiData => get_dpi_data(&device),
        Command::GetLeds => get_leds(&device),
        Command::GetPowerData => get_power_data(&device),
        Command::GetProfileAndFirmwareVersion => {}
        Command::SetActiveProfile { profile } => set_active_profile(&device, profile),
        Command::SetLightMode(command) => set_light_mode(&device, command),
    }
}
