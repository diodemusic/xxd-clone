use std::fs;

fn read_file(filename: &str) -> Vec<u8> {
    let bytes_result = fs::read(&filename);

    let bytes = match bytes_result {
        Ok(bytes) => bytes,
        Err(error) => panic!("Error reading file: {error:?}"),
    };

    bytes
}

fn parse_row(row: usize, chunk: &[u8]) {
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

fn main() -> std::io::Result<()> {
    let bytes = read_file("hello.txt");

    for (row, chunk) in bytes.chunks(16).enumerate() {
        parse_row(row, chunk);
    }

    Ok(())
}
