use chrono::Local;
use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;
use wallpaper;

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
    wallpaper::set_from_path(path_str)
}
