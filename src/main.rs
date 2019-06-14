use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    px: f64,
    #[structopt(short = "e", long = "em", default_value = "16")]
    em: f64,
    #[structopt(short = "p", long = "places", default_value = "8")]
    decimal_places: u32,
}

fn main() {
    let args = Args::from_args();
    let res = ((&args.px / &args.em) * (u32::pow(10, args.decimal_places) as f64)).round() / (u32::pow(10, args.decimal_places) as f64);
    println!("{}", res);
}
