use std::fs;

fn main() -> std::io::Result<()> {
    let bytes = fs::read("hello.txt")?;

    for (row, chunk) in bytes.chunks(16).enumerate() {
        print!("{:08x}: ", row * 16);

        for (i, byte) in chunk.iter().enumerate() {
            print!("{:02x}", byte);

            if i % 2 == 1 {
                print!(" ");
            }
        }

        print!(" ");

        for byte in chunk {
            match byte {
                0x20..=0x7e => print!("{}", *byte as char),
                _ => print!("."),
            }
        }

        println!();
    }

    Ok(())
}
