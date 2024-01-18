use std::fs;
use std::io::Write;

fn update_credentials_examples() -> Result<(), Box<dyn std::error::Error>> {
    let mut credentials_md = fs::read_to_string("ockam-documentation/reference/libraries/rust/credentials.md")?;
    let get_started_hello = fs::read_to_string("ockam/examples/rust/get_started/hello.rs")?;

    let start_line = credentials_md.lines().position(|line| line.contains("node.credentials(),"))?;
    let end_line = credentials_md.lines().position(|line| line.contains(");"))?;

    credentials_md = credentials_md.lines().enumerate().filter_map(|(i, line)| {
        if i >= start_line && i <= end_line {
            None
        } else {
            Some(line)
        }
    }).collect::<Vec<_>>().join("\n");

    let new_code_block = get_started_hello.lines().collect::<Vec<_>>().join("\n");
    let updated_credentials_md = format!(
        "{}\n{}\n{}",
        &credentials_md[..start_line],
        new_code_block,
        &credentials_md[end_line..]
    );

    let mut file = fs::File::create("ockam-documentation/reference/libraries/rust/credentials.md")?;
    file.write_all(updated_credentials_md.as_bytes())?;

    Ok(())
}

fn main() {
    if let Err(err) = update_credentials_examples() {
        eprintln!("Error updating credentials examples: {}", err);
        std::process::exit(1);
    }
}
