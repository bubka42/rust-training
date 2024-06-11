use base16ct::mixed;
use p72::encrypt;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1][..];
    let output_path = &args[2][..];
    let key = &args[3];

    let mut buf = [0u8; 16];
    let key_bytes = mixed::decode(key, &mut buf).unwrap();
    assert_eq!(key_bytes.len(), 16);

    println!("Encrypting {} into {}...", input_path, output_path);
    encrypt(input_path, output_path, key_bytes.try_into().unwrap())?;
    Ok(())
}
