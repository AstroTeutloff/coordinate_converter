use clap::Parser;
use coordinate_converter::angles;
use std::process::exit;


#[derive(Parser)]
#[command(name = "Coordinate Converter")]
#[command(version = "0.1")]
#[command(
    about = "Converts HMS DMS coordinates to decimal degrees (or DMS DMS) coordinates.",
    long_about = None
)]
#[derive(Debug)]
struct Cli {
    #[arg(num_args = 1, required = true)]
    ra: String,

    #[arg(num_args = 1, allow_hyphen_values = true, required = true)]
    dec: String,

    #[arg(long, short, default_value_t = false, help="Set the convertion to DMS DMS instead of decimal degrees.")]
    dmsdms: bool
}

fn main() {
    let cli = Cli::parse();
    let ra = match angles::Angle::from_hms(&cli.ra) {
        Ok(ra) => ra,
        Err(e) => {eprintln!("{}", e); exit(1);}
    };
    let dec = match angles::Angle::from_dms(&cli.dec) {
        Ok(dec) => dec,
        Err(e) => {eprintln!("{}", e); exit(1);}
    };


    if cli.dmsdms {
        print_dmsdms(ra, dec);
    } else {
        print_decimaldegree(ra, dec);
    }

}

fn print_dmsdms(ra: angles::Angle, dec: angles::Angle) -> () {
        println!("{:+} {:+}", ra.as_dms(), dec.as_dms());
}

fn print_decimaldegree(ra: angles::Angle, dec: angles::Angle) -> () {
        println!("{} {:+}", ra, dec);
}
