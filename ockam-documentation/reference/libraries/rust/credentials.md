def read_file(file_path: str) -> str:
    with open(file_path, 'r') as file:
        return file.read()

def find_code_block(file_path: str, start_line: int, end_line: int) -> str:
    file_content = read_file(file_path)
    lines = file_content.split('\n')
    code_block = '\n'.join(lines[start_line-1:end_line])
    return code_block

def replace_code_block(file_path: str, start_line: int, end_line: int, new_code: str) -> None:
    file_content = read_file(file_path)
    lines = file_content.split('\n')
    lines[start_line-1:end_line] = new_code.split('\n')
    updated_content = '\n'.join(lines)
    with open(file_path, 'w') as file:
        file.write(updated_content)

def update_credentials_file() -> None:
    credentials_file_path = 'ockam-documentation/reference/libraries/rust/credentials.md'
    get_started_file_path = 'ockam/examples/rust/get_started/credentials.md'

    existing_code_block = find_code_block(credentials_file_path, 97, 103)
    new_code_block = find_code_block(get_started_file_path, 97, 103)

    # Replace the existing code block with the new code block
    replace_code_block(credentials_file_path, 97, 103, new_code_block)

    # Set the trust_context parameter to None in the updated code block
    updated_code_block = new_code_block.replace('"trust_context".into()', 'None')

    # Replace the code block with the updated code block
    replace_code_block(credentials_file_path, 97, 103, updated_code_block)

# Run the function to update the credentials file
update_credentials_file()
```

Unit tests:

```python
def test_read_file():
    file_content = read_file('test_file.txt')
    assert file_content == 'This is a test file.'

def test_find_code_block():
    code_block = find_code_block('test_file.txt', 3, 5)
    assert code_block == 'line 3\nline 4\nline 5'

def test_replace_code_block():
    replace_code_block('test_file.txt', 3, 5, 'new line 3\nnew line 4\nnew line 5')
    file_content = read_file('test_file.txt')
    assert file_content == 'line 1\nline 2\nnew line 3\nnew line 4\nnew line 5\nline 6\nline 7'

def test_update_credentials_file():
    update_credentials_file()
    credentials_file_content = read_file('ockam-documentation/reference/libraries/rust/credentials.md')
    expected_code_block = find_code_block('ockam/examples/rust/get_started/credentials.md', 97, 103)
    assert expected_code_block in credentials_file_content
    assert '"trust_context".into()' not in credentials_file_content
    assert 'None' in credentials_file_content

test_read_file()
test_find_code_block()
test_replace_code_block()
test_update_credentials_file()
