use base16ct::mixed;
use p72::decrypt;
use std::{env, io};

fn main() -> std::io::Result<()> {
    let mut args = env::args();

    args.next().unwrap();
    let input_path = &args.next().expect("Provide input path")[..];
    let output_path = &args.next().expect("Provide output path")[..];
    let key = &args.next().expect("Provide hex-encoded key")[..];

    println!("Decrypting {} into {}...", input_path, output_path);

    let mut buf = [0u8; 16];
    let key_bytes = mixed::decode(key, &mut buf).unwrap();
    decrypt(
        input_path,
        output_path,
        key_bytes.try_into().map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "Wrong key length: 16 bytes expected")
        })?,
    )?;
    Ok(())
}
