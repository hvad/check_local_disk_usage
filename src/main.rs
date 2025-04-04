use clap::Parser;
use psutil::disk;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Disk path to check (e.g., "/")
    #[arg(short, long)]
    disk: String,

    /// Critical threshold for disk usage percentage
    #[arg(short, long, default_value_t = 75)]
    critical: u8,

    /// Warning threshold for disk usage percentage
    #[arg(short, long, default_value_t = 50)]
    warning: u8,
}

fn main() {
    let args = Args::parse();

    // Retrieve disk usage information
    match disk::disk_usage(&args.disk) {
        Ok(usage) => {
            let percent = usage.percent() as u8;

            // Determine status and message based on thresholds
            let (status, message) = match percent {
                _ if percent > args.critical => (
                    2,
                    format!("CRITICAL - Disk '{}' usage is at {}%", args.disk, percent),
                ),
                _ if percent > args.warning => (
                    1,
                    format!("WARNING - Disk '{}' usage is at {}%", args.disk, percent),
                ),
                _ => (
                    0,
                    format!("OK - Disk '{}' usage is at {}%", args.disk, percent),
                ),
            };

            println!("{}", message);
            process::exit(status);
        }
        Err(e) => {
            // Handle errors when retrieving disk usage
            eprintln!("ERROR - Failed to retrieve disk usage for '{}': {}", args.disk, e);
            process::exit(3); // Exit with an unknown error code
        }
    }
}
