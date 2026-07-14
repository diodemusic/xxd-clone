use std::fs;

fn read_file(filename: &str) -> Vec<u8> {
    let bytes_result = fs::read(filename);

    match bytes_result {
        Ok(bytes) => bytes,
        Err(error) => panic!("Error reading file: {error:?}"),
    }
}

fn parse_row(row: usize, chunk: &[u8], output: &mut String) {
    output.push_str(&format!("{:08x}: ", row * 16));

    for (i, byte) in chunk.iter().enumerate() {
        output.push_str(&format!("{:02x}", byte));

        if i % 2 == 1 {
            output.push(' ');
        }
    }

    let chunk_budget = chunk.len() * 2 + chunk.len() / 2;

    for _ in chunk_budget..40 {
        output.push(' ');
    }

    for byte in chunk {
        match byte {
            0x20..=0x7e => output.push_str(&format!("{}", *byte as char)),
            _ => output.push('.'),
        }
    }

    output.push('\n');
}

fn main() -> std::io::Result<()> {
    let mut output = String::new();
    let bytes = read_file("hello.txt");

    for (row, chunk) in bytes.chunks(16).enumerate() {
        parse_row(row, chunk, &mut output);
    }

    println!("{output}");

    Ok(())
}
