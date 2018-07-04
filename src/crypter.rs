#[allow(dead_code)]
use std::process::exit;
use std::fs::File;
use std::io::{BufReader, Read, BufWriter, Write};
use std::convert::TryFrom;

pub fn crypt_and_save(f: File, name: String, key: u64, encrypted: bool) -> Result<(), CryptError> {
    let buf_reader = BufReader::new(f);
    let mut buffer = BufWriter::new(File::create(name)?);
    let mut bytes: [u8; 8] = [0; 8];
    let mut counter: usize = 0;
    let mut to_crypt;
    let mut crypted;
    for byte in buf_reader.bytes() {
        if counter % 8 == 0 && counter > 0 {
            to_crypt = bytes_to_u64(bytes);
            if encrypted {
                crypted = decrypt(to_crypt, key);
            } else {
                crypted = encrypt(to_crypt, key);
            }
            bytes = u64_to_bytes(crypted);
            for b in bytes.iter() {
                buffer.write(&[*b])?;
            }
        }
        bytes[counter % 8] = byte?;
        counter += 1;
    }
    if counter % 8 != 0 {
        crypt_and_save_remaining(bytes, counter % 8, &mut buffer, key, encrypted)?;
    }
    buffer.flush()?;
    Ok(())
}

fn crypt_and_save_remaining(bytes: [u8; 8], number: usize, buffer: &mut BufWriter<File>, key: u64, encrypted: bool) -> Result<(), CryptError> {
    let small_key = u64_to_u8(key);
    for counter in 0..number {
        if encrypted {
            buffer.write(&[decrypt_small(bytes[counter], small_key)])?;
        } else {
            buffer.write(&[encrypt_small(bytes[counter], small_key)])?;
        }
    }
    Ok(())
}

fn u64_to_u8(mut number: u64) -> u8 {
    number <<= 56;
    number >>= 56;
    match u8::try_from(number) {
        Ok(byte) => byte,
        Err(e) => {
            println!("Magical Error ¯\\(°_o)/¯ {}", e);
            exit(1);
        }
    }
}

pub fn bytes_to_u64(bytes: [u8; 8]) -> u64 {
    let mut sol: u64 = 0;
    let bits = 8;
    for byte in bytes.iter() {
        sol <<= bits;
        sol += u64::from(*byte);
    }
    sol
}

pub fn u64_to_bytes(number: u64) -> [u8; 8] {
    let mut bytes: [u8; 8] = [0; 8];
    let bits = 8;
    let mut b;
    for counter in 0..8 {
        b = number << (bits * (7 - counter));
        b >>= bits * 7;
        bytes[7 - counter] = match u8::try_from(b) {
            Ok(byte) => byte,
            Err(e) => {
                 println!("Unable to do convert: {}", e);
                 exit(1);
            }
        };
    }
    bytes
}

fn encrypt(byte: u64, key: u64) -> u64 {
    byte.wrapping_add(key)
}

fn decrypt(byte: u64, key: u64) -> u64 {
    byte.wrapping_sub(key)
}

fn encrypt_small(byte: u8, key: u8) -> u8 {
    byte.wrapping_add(key)
}

fn decrypt_small(byte: u8, key: u8) -> u8 {
    byte.wrapping_sub(key)
}

pub fn standard_crypt_error_handling(e: CryptError) {
    println!("ERROR: {}", e.error_information);
    exit(1);
}

#[derive(Debug)]
pub struct CryptError {
    pub error_information: String,
}
impl CryptError {
    pub fn new(string: String) -> CryptError {
        CryptError { error_information: string }
    }
}
impl From<std::io::Error> for CryptError {
    fn from(e: std::io::Error) -> CryptError {
        CryptError::new(e.to_string())
    }
}

impl From<std::num::ParseIntError> for CryptError {
    fn from(e: std::num::ParseIntError) -> CryptError {
        CryptError::new(e.to_string())
    }
}
