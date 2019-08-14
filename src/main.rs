use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Args {
    px: f64,
    #[structopt(short = "b", long, default_value = "16")]
    base_size: f64,
    #[structopt(short = "p", long, default_value = "8")]
    decimal_places: u32,
    #[structopt(name = "rem", long, short = "r")]
    rem_to_px: bool
}

fn main() {
    let args = Args::from_args();
    let res = if args.rem_to_px {
        rempx(args.px, args.base_size, args.decimal_places)
    } else {
        pxrem(args.px, args.base_size, args.decimal_places)
    };
    println!("{}", res);
}

fn pxrem(px: f64, base_size: f64, decimal_places: u32) -> f64 {
    ((px / base_size) * (u64::pow(10, decimal_places) as f64)).round() / (u64::pow(10, decimal_places) as f64)
}
fn rempx(rem: f64, base_size: f64, decimal_places: u32) -> f64 {
    ((rem * base_size) * (u64::pow(10, decimal_places) as f64)).round() / (u64::pow(10, decimal_places) as f64)
}
