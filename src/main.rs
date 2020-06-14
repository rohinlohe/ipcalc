use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Pattern to look for
    pattern: String,

    /// Path to the file
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
