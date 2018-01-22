extern crate os_type;
extern crate reqwest;
extern crate semver;
extern crate url;

mod chefdk;

fn main() {
    chefdk::install_chefdk();
}
