use clap::Parser;
use psutil::disk;

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

    println!("Disk {} usage {:?}", &args.disk, disk_usage);
}
