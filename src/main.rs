use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Args {
    px: f64,
    #[structopt(short = "b", long, default_value = "16")]
    base_size: f64,
    #[structopt(short = "p", long, default_value = "8")]
    decimal_places: u32,
}

fn main() {
    let args = Args::from_args();
    let res = ((&args.px / &args.base_size) * (u64::pow(10, args.decimal_places) as f64)).round() / (u64::pow(10, args.decimal_places) as f64);
    println!("{}", res);
}
