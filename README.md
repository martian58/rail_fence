# Rail Fence Cipher Implementation

This project implements the Rail Fence Cipher encryption and decryption in Rust. It demonstrates the correctness of the implementation through unit tests and provides a command-line interface for encrypting and decrypting text.

## Features

- **Encryption**: Convert plaintext to ciphertext using the zigzag Rail Fence pattern.
- **Decryption**: Reconstruct plaintext from ciphertext using the Rail Fence pattern.
- **Command-line Interface**: Specify options like depth of the fence, input text, and whether to encrypt or decrypt.
- **Unit Tests**: Validate the correctness of the implementation with `cargo test`.

---

## Prerequisites

To build and run this project, make sure you have the following installed:

- [Rust and Cargo](https://www.rust-lang.org/tools/install)

---

## How to Compile the Project

To compile the project, navigate to the project directory and run:

```bash
cargo build
```

This will create the compiled binary in the `target/debug` directory.

---

## How to Run the Program

### Installation

```bash
git clone https://github.com/martian58/rail_fence.git
cd rail_fence
cargo run -- --help
```

You can use the `cargo run` command to execute the program. The program requires several arguments:

### Basic Command Syntax

```bash
cargo run -- [OPTIONS]
```
Or
```bash
./target/debug/rail_fence [OPTIONS]
```

### Get help 
```bash
cargo run -- --help
```
Or
```bash
./target/debug/rail_fence --help
```

### Example Commands

1. **Encrypt a Message**:

   ```bash
   cargo run -- -d 3 -i "HELLO"
   ```

   Output:
   ```
   Encrypted Text: HOELL
   ```

2. **Decrypt a Message**:

   ```bash
   cargo run -- -d 3 -i "HOELL" --decrypt
   ```

   Output:
   ```
   Decrypted Text: HELLO
   ```

### Command-Line Options

| Option        | Description                                                                 |
|---------------|-----------------------------------------------------------------------------|
| `-d, --depth` | Depth of the Rail Fence cipher (number of rails). Required.                |
| `-i, --input` | The text to encrypt or decrypt. Required.                                  |
| `-x, --decrypt` | Decrypt the input text instead of encrypting it. Optional (default is encryption). |

---

## How to Run Unit Tests

To validate the correctness of the implementation, run the unit tests using:

```bash
cargo test
```

### Test Output

```
running 4 tests
test tests::test_encrypt ... ok
test tests::test_decrypt ... ok
test tests::test_encrypt_with_depth_4 ... ok
test tests::test_decrypt_with_depth_4 ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## How to Clean the Project

To remove build artifacts and clean the project directory, run:

```bash
cargo clean
```

---

## Project Structure

- **`src/main.rs`**: Contains the implementation of the Rail Fence Cipher and the command-line interface.
- **`Cargo.toml`**: Manages dependencies and project metadata.
- **`README.md`**: Documentation for the project (this file).

---

## Example Usage

1. **Encrypt a Longer Message**:

   ```bash
   cargo run -- -d 4 -i "RAILFENCEISTHEBEST"
   ```

   Output:
   ```
   Encrypted Text: RNHAECTETIFESBSLIE
   ```

2. **Decrypt the Same Message**:

   ```bash
   cargo run -- -d 4 -i "RNHAECTETIFESBSLIE" --decrypt
   ```

   Output:
   ```
   Decrypted Text: RAILFENCEISTHEBEST
   ```

---

## Additional Notes

- Ensure the depth of the Rail Fence cipher (`--depth`) is at least 2.
- The input text can contain uppercase or lowercase characters, but the program will treat all characters as uppercase.
- Non-alphabetic characters are ignored during encryption/decryption.
