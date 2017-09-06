extern crate workup;

extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "workup", about = "Runs workup")]
struct Opt {
    #[structopt(short = "o", long = "offline", help = "Work offline")]
    offline: bool,

    #[structopt(short = "p", long = "policyfile", help = "Policyfile to use for converge")]
    policyfile: String,

    #[structopt(long = "verify_ssl", help = "Fail if there are SSL exceptions", default_value = "true")]
    verify_ssl: bool,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    unimplemented!();
}
