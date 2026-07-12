use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let mut file = File::open("big.bin")?;
    let mut buf = [0u8; 64];
    let mut total_bytes = 0;

    loop {
        let n = file.read(&mut buf)?;

        if n == 0 {
            break;
        }

        for (i, row) in buf[..n].chunks(16).enumerate() {
            print!("{:08x}:", total_bytes + i * 16);

            for (j, byte) in row.iter().enumerate() {
                if j % 2 == 0 {
                    print!(" ");
                }

                print!("{:02x}", byte);
            }

            let l = 2 * row.len() + (row.len().div_ceil(2));

            for _ in 0..40 - l {
                print!(" ");
            }

            print!("  ");

            for byte in row {
                match byte {
                    0x20..=0x7e => print!("{}", *byte as char),
                    _ => print!("."),
                }
            }
            println!();
        }

        total_bytes += n;
    }

    Ok(())
}
