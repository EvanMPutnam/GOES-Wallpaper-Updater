use chrono::Local;
use reqwest::blocking::get;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::os::raw::c_void;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::thread;
use std::time::Duration;
use windows::Win32::UI::WindowsAndMessaging::{
    SPI_SETDESKWALLPAPER, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, SystemParametersInfoW,
};

pub fn download_loop(url: &str, img_path: &Path, initial_wait_minutes: u64, refresh_minutes: u64) {
    println!(
        "Sleeping for initial wait ({} minutes)...",
        initial_wait_minutes
    );
    thread::sleep(Duration::from_secs(60 * initial_wait_minutes));

    loop {
        println!("Downloading image: {}", Local::now());

        match download_image(url, &img_path) {
            Ok(()) => {
                if let Err(e) = set_wallpaper(&img_path) {
                    eprintln!("Failed to set wallpaper: {e}");
                } else {
                    println!("Updated wallpaper at: {}", Local::now());
                }
            }
            Err(e) => {
                eprintln!("Failed to download image: {e}");
            }
        }

        thread::sleep(Duration::from_secs(60 * refresh_minutes));
    }
}

fn download_image(url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;
    let bytes = response.bytes()?;

    let mut file = File::create(path)?;
    file.write_all(&bytes)?;

    Ok(())
}

fn set_wallpaper(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let path_str = path.to_str().ok_or("Wallpaper path is not valid UTF-8")?;
    let wide: Vec<u16> = OsStr::new(path_str)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let result = unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(wide.as_ptr() as *mut c_void),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
    };

    if result.is_ok() {
        Ok(())
    } else {
        Err("SystemParametersInfoW failed".into())
    }
}
