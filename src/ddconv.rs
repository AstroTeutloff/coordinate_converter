use clap::Parser;
use coordinate_converter::angles;


#[derive(Parser)]
#[command(name = "Coordinate Converter")]
#[command(version = "0.1")]
#[command(
    about = "Converts decimal coordinates to HMS DMS (or DMS DMS) coordinates.",
    long_about = None
)]
#[derive(Debug)]
struct Cli {
    #[arg(num_args = 1, required = true)]
    ra: f64,

    #[arg(num_args = 1, allow_hyphen_values = true, required = true)]
    dec: f64,

    #[arg(long, short, default_value_t = false, help="Set the convertion to DMS DMS instead of HMS DMS.")]
    dmsdms: bool
}

fn main() {
    let cli = Cli::parse();
    let ra = angles::Angle::new(cli.ra);
    let dec = angles::Angle::new(cli.dec);

    if cli.dmsdms {
        print_dmsdms(ra, dec);
    } else {
        print_hmsdms(ra, dec);
    }

}

fn print_dmsdms(ra: angles::Angle, dec: angles::Angle) -> () {
        println!("{:+} {:+}", ra.as_dms(), dec.as_dms());
}

fn print_hmsdms(ra: angles::Angle, dec: angles::Angle) -> () {
        println!("{} {:+}", ra.as_hms(), dec.as_dms());
}
