use colored::Colorize;
use std::fs;

fn read_file(filename: &str) -> Vec<u8> {
    let bytes_result = fs::read(filename);

    match bytes_result {
        Ok(bytes) => bytes,
        Err(error) => panic!("Error reading file: {error:?}"),
    }
}

fn parse_byte(byte: &u8, output: &mut String) {
    match byte {
        0x20..=0x7e => output.push_str(&format!("{:02x}", byte)),
        _ => output.push_str(&format!("{:02x}", byte).red().to_string()),
    }
}

fn parse_ascii(chunk: &[u8], output: &mut String) {
    let chunk_budget = chunk.len() * 2 + chunk.len() / 2;

    for _ in chunk_budget..40 {
        output.push(' ');
    }

    output.push(' ');

    for byte in chunk {
        match byte {
            0x20..=0x7e => output.push_str(&format!("{}", *byte as char)),
            _ => output.push_str(&".".red().to_string()),
        }
    }
    output.push('\n');
}

fn parse_row(row: usize, chunk: &[u8], output: &mut String) {
    output.push_str(&format!("{:08x}: ", row * 16));

    for (i, byte) in chunk.iter().enumerate() {
        parse_byte(byte, output);

        if i % 2 == 1 {
            output.push(' ');
        }
    }

    parse_ascii(chunk, output);
}

fn main() -> std::io::Result<()> {
    let mut output = String::new();
    let bytes = read_file("hello.txt");

    for (row, chunk) in bytes.chunks(16).enumerate() {
        parse_row(row, chunk, &mut output);
    }

    print!("{output}");

    Ok(())
}
