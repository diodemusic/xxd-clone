use std::fs;

fn main() -> std::io::Result<()> {
    let bytes = fs::read("big.bin")?;

    for chunk in bytes.chunks(16) {
        for (i, byte) in chunk.iter().enumerate() {
            if i % 2 == 0 {
                print!(" ");
            }

            if i % 16 == 0 {
                println!();
            }

            print!("{:02x}", byte);
        }
    }

    Ok(())
}
