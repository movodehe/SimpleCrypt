#![feature(try_from, extern_prelude)]
#![allow(dead_code)]
extern crate clap;
mod cli;
use cli::cli;
mod crypter;
use crypter::{crypt_and_save, CryptError};

pub fn run() -> Result<(), CryptError> {
    let (file, filename, key, encrypted) = cli()?;
    match crypt_and_save(file, filename, key, encrypted) {
        Ok(_) => {
            if encrypted {
                println!("Success! You can find the decrypted file in the same directory with the original or chosen name.");
            } else {
                println!("Success! You can find the encrypted file in the same directory with a .encrypt suffix.");
                println!("Advise: You can now test to decrypt the file again and then should delete the both unencrypted files.");
                println!("Otherwise you whould have just wasted your time and no one wants to waist their time.");
            }
        }
        Err(e) => return Err(e),
    }
    Ok(())
}
