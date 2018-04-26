mod unit_test_crypter;
mod crypter;

fn main() {
	let version = env!("CARGO_PKG_VERSION");
	println!("**************************************************************");
	println!("*                                                            *");
	println!("*                   Welcome to SimpleCrypt                   *");
	println!("*                  by movodehe and wholl0p.                  *");
	println!("*                                                            *");
	println!("*                       version: {}                       *", version);
	println!("*                                                            *");	
	println!("*                  Usage: [file] [bitshift]                  *");
	println!("*                                                            *");
	println!("**************************************************************");
	crypter::run();
}
