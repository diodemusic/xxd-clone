use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let mut file = File::open("big.bin")?;
    let mut buf = [0u8; 64];
    let n = file.read(&mut buf)?;

    loop {
        for (i, row) in buf[..n].chunks(16).enumerate() {
            print!("{:08x}", i * 16);

            for (j, byte) in row.iter().enumerate() {
                if j % 2 == 0 {
                    print!(" ");
                }

                print!("{:02x}", byte);
            }

            println!("");
        }

        if buf.len() == 64 {
            break;
        }
    }

    Ok(())
}
