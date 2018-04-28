extern crate simple_crypt;
extern crate clap;
mod unit_test_crypter;
mod cli;
mod crypter;

fn main() {
    match simple_crypt::run() {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e.error_information);
            std::process::exit(1);
        }
    }
}
