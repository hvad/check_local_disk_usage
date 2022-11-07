use clap::Parser;
use psutil::disk;
use std::process;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    disk: String,
}

fn main() {
    let args = Args::parse();

    let disk_usage = disk::disk_usage(&args.disk).unwrap();
    let disk_usage_percent = disk_usage.percent() as u8;

    if disk_usage_percent > 90 {
        println!(
            "CRITICAL - Disk {} usage {}%",
            &args.disk, disk_usage_percent
        );
        process::exit(2);
    } else if disk_usage_percent > 80 {
        println!(
            "WARNING - Disk {} usage {}%",
            &args.disk, disk_usage_percent
        );
        process::exit(1);
    } else {
        println!("OK - Disk {} usage {}%", &args.disk, disk_usage_percent);
        process::exit(0);
    }
}
