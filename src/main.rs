use std::process::exit;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, BufWriter, Write};
use std::str::FromStr;
use std::convert::TryFrom;

fn main() {
    let (file, filename, key, encrypted) = match get_args() {
        Ok(args) => args,
        Err(e) => return standard_io_error_handling(e),
    };
    match crypt_and_save(file, filename, key, encrypted) {
        Ok(_) => {
            if encrypted {
                println!("Success! You can find the decrypted file in the same directory with the original or chosen name.");
            } else {
                println!("Success! You can find the encrypted file in the same directory with a .encrypt suffix");
                println!("Advise: You can now test to decrypt the file again and then should delete the both unencrypted files.");
                println!("Otherwise you whould have just wasted your time and no one wants to waist their time.");
            }
        }
        Err(e) => standard_io_error_handling(e),
    }
}

fn get_args() -> Result<(File, String, u64, bool), io::Error> {
    let file = match env::args().nth(1) {
        Some(f) => f,
        None => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No argument given")),
    };
    let key = match env::args().nth(2) {
        Some(k) => match u64::from_str(&k) {
            Ok(k) => k,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("{}", e))),
        }
        None => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No key given")),
    };
    let mut encrypted = false;
    if file.ends_with(".encrypt") {
        encrypted = true;
    }
    Ok((File::open(&file)?, file, key, encrypted))
}

fn crypt_and_save(f: File, mut name: String, key: u64, encrypted: bool) -> Result<(), io::Error> {
    let buf_reader = BufReader::new(f);
    if encrypted {
        name = name.trim_right_matches(".encrypt").to_string();
    } else {
        name.push_str(".encrypt");
    }
    match check_file_existance(&name) {
        Some(s) => name = s,
        None => (),
    }
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

fn crypt_and_save_remaining(bytes: [u8; 8], number: usize, buffer: &mut BufWriter<std::fs::File>, key: u64, encrypted: bool) -> Result<(), io::Error> {
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

fn bytes_to_u64(bytes: [u8; 8]) -> u64 {
    let mut sol: u64 = 0;
    let mut counter = 7;
    for byte in bytes.iter() {
        sol += u64::from(*byte).pow(counter);
        counter -= 1;
    }
    sol
}

fn u64_to_bytes(number: u64) -> [u8; 8] {
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

fn check_file_existance(name: &String) -> Option<String> {
    match File::open(&name) {
        Ok(_) => {
            loop {
                let mut input = String::new();
                println!("The file {} already exists. Do you want to overwrite it? (y/n)", name);
                match io::stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("ERROR: {}", e);
                        exit(1);
                    }
                }
                if input.eq("y\n") {
                    break;
                }
                if input.eq("n\n") {
                    println!("Please choose a new name for the file.");
                    println!("Note that this time there won't be a check if the file already exists.");
                    input.clear();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            return Some(input.trim_right().to_string());
                        }
                        Err(e) => {
                            println!("ERROR: {}", e);
                            exit(1);
                        }
                    }
                }
            }
        }
        Err(_) => (),
    }
    None
}

fn standard_io_error_handling(e: io::Error) {
    println!("ERROR: {}", e);
    exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    # [test]
    fn u64_to_bytes_1() {
        assert_eq!([0; 8], u64_to_bytes(0));
    }
    # [test]
    fn u64_to_bytes_2() {
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 123], u64_to_bytes(123));
    }
    # [test]
    fn u64_to_bytes_3() {
        assert_eq!([0, 0, 0, 0, 0, 0, 1, 0], u64_to_bytes(256));
    }
    # [test]
    fn u64_to_bytes_4() {
        let base: u64 = 256;
        assert_eq!([100, 0, 0, 0, 0, 0, 0, 0], u64_to_bytes(base.pow(7) * 100));
    }
    # [test]
    fn u64_to_bytes_5() {
        let base: u64 = 256;
        assert_eq!([89, 233, 92, 97, 1, 33, 255, 123], u64_to_bytes(base.pow(0) * 123 + base.pow(1) * 255 + base.pow(2) * 33 + base.pow(3) * 1
        + base.pow(4) * 97 + base.pow(5) * 92 + base.pow(6) * 233 + base.pow(7) * 89));
    }
    # [test]
    fn u64_to_bytes_6() {
        let base: u64 = 256;
        assert_eq!([255, 255, 255, 255, 255, 255, 255, 255], u64_to_bytes(base.pow(0) * 255 + base.pow(1) * 255 + base.pow(2) * 255 + base.pow(3) * 255
        + base.pow(4) * 255 + base.pow(5) * 255 + base.pow(6) * 255 + base.pow(7) * 255));
    }
}