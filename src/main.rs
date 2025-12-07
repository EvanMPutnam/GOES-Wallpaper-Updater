mod executor;

use std::env;

use crate::executor::download_loop;

const GOES_EAST_URL: &str =
    "https://cdn.star.nesdis.noaa.gov/GOES16/ABI/FD/GEOCOLOR/5424x5424.jpg";
const GOES_WEST_URL: &str =
    "https://cdn.star.nesdis.noaa.gov/GOES18/ABI/FD/GEOCOLOR/5424x5424.jpg";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!(
            "Usage: {} <east|west> <refresh_minutes> <initial_wait_minutes>",
            args.get(0).map(String::as_str).unwrap_or("goes_wallpaper")
        );
        std::process::exit(1);
    }

    let goes_sat = args[1].to_lowercase();

    let refresh_minutes: u64 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("refresh_minutes must be a positive integer (minutes)");
        std::process::exit(1);
    });

    let initial_wait_minutes: u64 = args[3].parse().unwrap_or_else(|_| {
        eprintln!("initial_wait_minutes must be a positive integer (minutes)");
        std::process::exit(1);
    });

    let url = match goes_sat.as_str() {
        "east" => GOES_EAST_URL,
        "west" => GOES_WEST_URL,
        _ => {
            eprintln!("First argument must be either 'east' or 'west'");
            std::process::exit(1);
        }
    };

    // Build the full path to "latest.jpg" in the current working directory
    let img_path = env::current_dir()
        .expect("Failed to get current directory")
        .join("latest.jpg");

    println!(
        "Selected GOES-{} with refresh every {} minutes (initial wait: {} minutes)",
        goes_sat, refresh_minutes, initial_wait_minutes
    );

    download_loop(url, img_path.as_path(), initial_wait_minutes, refresh_minutes);
}