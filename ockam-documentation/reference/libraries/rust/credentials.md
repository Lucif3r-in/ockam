// Function to update the examples in credentials.md
fn update_credentials_examples() {
    // Read the contents of credentials.md
    let credentials_md = read_file("ockam-documentation/reference/libraries/rust/credentials.md");

    // Find the code block to be replaced
    let start_line = 97;
    let end_line = 103;
    let code_block_to_replace = credentials_md.get_code_block(start_line, end_line);

    // Read the contents of get_started.rs
    let get_started_rs = read_file("ockam/examples/rust/get_started");

    // Find the corresponding code block in get_started.rs
    let corresponding_code_block = get_started_rs.get_code_block(start_line, end_line);

    // Replace the code block in credentials.md with the corresponding code block
    let updated_credentials_md = credentials_md.replace_code_block(start_line, end_line, corresponding_code_block);

    // Write the updated contents back to credentials.md
    write_file("ockam-documentation/reference/libraries/rust/credentials.md", updated_credentials_md);
}

// Unit tests for update_credentials_examples function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_credentials_examples() {
        // Mock the contents of credentials.md and get_started.rs
        let credentials_md = "Contents of credentials.md";
        let get_started_rs = "Contents of get_started.rs";

        // Mock the expected updated credentials.md
        let expected_updated_credentials_md = "Expected updated credentials.md";

        // Mock the file reading and writing functions
        mock_read_file("ockam-documentation/reference/libraries/rust/credentials.md", credentials_md);
        mock_read_file("ockam/examples/rust/get_started", get_started_rs);
        mock_write_file("ockam-documentation/reference/libraries/rust/credentials.md", expected_updated_credentials_md);

        // Call the function to update the examples
        update_credentials_examples();

        // Verify that the file reading and writing functions were called with the correct arguments
        assert_file_read("ockam-documentation/reference/libraries/rust/credentials.md");
        assert_file_read("ockam/examples/rust/get_started");
        assert_file_written("ockam-documentation/reference/libraries/rust/credentials.md", expected_updated_credentials_md);
    }
}
