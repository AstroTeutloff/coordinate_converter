use clap::Parser;
use coordinate_converter::angles;
use std::process::exit;


#[derive(Parser)]
#[command(name = "Coordinate Converter")]
#[command(version = "0.1")]
#[command(
    about = "Converts DMS DMS coordinates to decimal degrees (or HMS DMS) coordinates.",
    long_about = None
)]
#[derive(Debug)]
struct Cli {
    #[arg(num_args = 1, required = true)]
    ra: String,

    #[arg(num_args = 1, allow_hyphen_values = true, required = true)]
    dec: String,

    #[arg(long, short, default_value_t = false, help="Set the convertion to HMS DMS instead of decimal degrees.")]
    hmsdms: bool
}

fn main() {
    let cli = Cli::parse();
    let ra = match angles::Angle::from_dms(&cli.ra) {
        Ok(ra) => ra,
        Err(e) => {eprintln!("{}", e); exit(1);}
    };
    let dec = match angles::Angle::from_dms(&cli.dec) {
        Ok(dec) => dec,
        Err(e) => {eprintln!("{}", e); exit(1);}
    };


    if cli.hmsdms {
        print_hmsdms(ra, dec);
    } else {
        print_decimaldegree(ra, dec);
    }

}

fn print_hmsdms(ra: angles::Angle, dec: angles::Angle) -> () {
        println!("{} {:+}", ra.as_hms(), dec.as_dms());
}

fn print_decimaldegree(ra: angles::Angle, dec: angles::Angle) -> () {
        println!("{} {:+}", ra, dec);
}
