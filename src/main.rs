use clap::{Arg, Command};

/// Enum to represent encryption or decryption mode
#[derive(Debug, Copy, Clone)]
enum CipherMode {
    Encrypt,
    Decrypt,
}

/// Encrypts or decrypts text using the Rail Fence cipher.
///
/// # Arguments
///
/// * `text` - The text to encrypt or decrypt.
/// * `depth` - The number of rails (depth of the fence).
/// * `mode` - The encryption or decryption mode.
///
/// # Returns
///
/// * The encrypted or decrypted text.
fn rail_fence_cipher(text: &str, depth: usize, mode: CipherMode) -> String {
    let mut rails: Vec<Vec<char>> = vec![Vec::new(); depth];
    let text_chars: Vec<char> = text.chars().collect();
    let mut result: Vec<char> = Vec::new();

    match mode {
        CipherMode::Encrypt => {
            let mut current_rail = 0;
            let mut direction: isize = 1; 

            // Place characters in zigzag pattern
            for &c in &text_chars {
                rails[current_rail as usize].push(c);
                current_rail = if current_rail == 0 {
                    direction = 1;
                    current_rail + direction
                } else if current_rail == (depth as isize - 1) {
                    direction = -1;
                    current_rail + direction
                } else {
                    current_rail + direction
                };
            }

            // Concatenate rails to produce the encrypted text
            for rail in rails {
                result.extend(rail);
            }
        }
        CipherMode::Decrypt => {
            // Calculate the pattern of rails
            let mut rail_lengths: Vec<usize> = vec![0; depth];
            let mut current_rail: isize = 0;
            let mut direction: isize = 1; 

            for _ in &text_chars {
                rail_lengths[current_rail as usize] += 1;
                current_rail = if current_rail == 0 {
                    direction = 1;
                    current_rail + direction
                } else if current_rail == (depth as isize - 1) {
                    direction = -1;
                    current_rail + direction
                } else {
                    current_rail + direction
                };
            }

            // Populate rails with characters
            let mut pos: usize = 0;
            for (i, &len) in rail_lengths.iter().enumerate() {
                rails[i] = text_chars[pos..pos + len].to_vec();
                pos += len;
            }

            // Read characters in zigzag pattern
            let mut rail_iters: Vec<_> = rails.iter_mut().map(|r| r.iter()).collect();
            current_rail = 0;
            direction = 1;

            for _ in 0..text_chars.len() {
                if let Some(&c) = rail_iters[current_rail as usize].next() {
                    result.push(c);
                }
                current_rail = if current_rail == 0 {
                    direction = 1;
                    current_rail + direction
                } else if current_rail == (depth as isize - 1) {
                    direction = -1;
                    current_rail + direction
                } else {
                    current_rail + direction
                };
            }
        }
    }

    result.iter().collect()
}

fn main() {
    // Parse command-line arguments
    let matches: clap::ArgMatches = Command::new("Rail Fence")
        .bin_name("rail_fence")
        .version("1.0")
        .author("martian58")
        .about("Encrypts or decrypts text using the Rail Fence cipher")
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .value_name("DEPTH")
                .help("Sets the depth of the Rail Fence cipher")
                .required(true),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("TEXT")
                .help("The text to encrypt or decrypt")
                .required(true),
        )
        .arg(
            Arg::new("decrypt")
                .short('x')
                .long("decrypt")
                .help("Decrypt the input text instead of encrypting")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let depth: usize = matches.get_one::<String>("depth").unwrap().parse().expect("Invalid depth");
    let input_text: &String = matches.get_one::<String>("input").unwrap();
    let decrypt: bool = matches.get_flag("decrypt");

    // Determine the mode
    let mode: CipherMode = if decrypt {
        CipherMode::Decrypt
    } else {
        CipherMode::Encrypt
    };

    // Encrypt or decrypt the text
    let result: String = rail_fence_cipher(input_text, depth, mode);

    match mode {
        CipherMode::Encrypt => println!("Encrypted Text: {}", result),
        CipherMode::Decrypt => println!("Decrypted Text: {}", result),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let plaintext: &'static str = "HELLO";
        let encrypted: String = rail_fence_cipher(plaintext, 3, CipherMode::Encrypt);
        assert_eq!(encrypted, "HOELL");
    }

    #[test]
    fn test_decrypt() {
        let encrypted: &'static str = "HOELL";
        let decrypted: String = rail_fence_cipher(encrypted, 3, CipherMode::Decrypt);
        assert_eq!(decrypted, "HELLO");
    }

    #[test]
    fn test_encrypt_with_depth_4() {
        let plaintext: &'static str = "RAILFENCEISTHEBEST";
        let encrypted: String = rail_fence_cipher(plaintext, 4, CipherMode::Encrypt);
        assert_eq!(encrypted, "RNHAECTETIFESBSLIE");
    }

    #[test]
    fn test_decrypt_with_depth_4() {
        let encrypted: &'static str = "RNHAECTETIFESBSLIE";
        let decrypted: String = rail_fence_cipher(encrypted, 4, CipherMode::Decrypt);
        assert_eq!(decrypted, "RAILFENCEISTHEBEST");
    }
}