use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use reqwest::blocking::Client;

/// # Arguments
/// * `start_dir` - The starting directory to begin the search.
/// * `target_file` - The name of the target file to find.
/// * `key_file` - The name of the key file to find.
///
/// # Returns
/// A tuple containing `Option<String>` paths to the target and key files if found, respectively.
///
/// # Example
/// ```
/// use std::path::Path;
/// let start_dir = Path::new(".");
/// let target_file = "example_target.txt";
/// let key_file = "example_key.txt";
/// let (target_path, key_path) = find_files(start_dir, target_file, key_file);
/// println!("Target file path: {:?}", target_path);
/// println!("Key file path: {:?}", key_path);
/// ```

fn find_files(start_dir: &Path, target_file: &str, key_file: &str) -> (Option<String>, Option<String>) {
    let mut target_path = None;
    let mut key_path = None;

    let entries = fs::read_dir(start_dir).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            if let (Some(target_path_nested), Some(key_path_nested)) = find_files(&path, target_file, key_file) {
                target_path = Some(target_path_nested);
                key_path = Some(key_path_nested);
            }
        } else {
            if path.file_name().unwrap() == target_file {
                target_path = Some(path.to_str().unwrap().to_owned());
            } else if path.file_name().unwrap() == key_file {
                key_path = Some(path.to_str().unwrap().to_owned());
            }
        }
    }

    (target_path, key_path)
}

/// Decrypts a file using a specified key file.
///
/// # Arguments
/// * `target_path` - The path to the file to decrypt.
/// * `key_path` - The path to the key file used for decryption.
///
/// # Returns
/// A `Vec<u8>` containing the decrypted file data.
///
/// # Example
/// ```
/// let target_path = "path/to/target.txt";
/// let key_path = "path/to/key.txt";
/// let decrypted_data = decrypt_file(target_path, key_path);
/// ```

fn decrypt_file(target_path: &str, key_path: &str) -> Vec<u8> {
    let mut key_file = fs::File::open(key_path).unwrap();
    let mut key = Vec::new();
    key_file.read_to_end(&mut key).unwrap();

    let mut target_file = fs::File::open(target_path).unwrap();
    let mut ciphertext = Vec::new();
    target_file.read_to_end(&mut ciphertext).unwrap();

    let plaintext: Vec<u8> = ciphertext.iter().zip(key.iter().cycle()).map(|(c, k)| c ^ k).collect();

    plaintext
}

/// Sends file data to a specified server URL.
///
/// # Arguments
/// * `data` - The data of the file to send.
/// * `server_url` - The URL of the server to which the file will be sent.
///
/// # Example
/// ```
/// let decrypted_data: Vec<u8> = vec![...]; // Assume this is decrypted file data
/// send_file(&decrypted_data, server_url);
/// ```

fn send_file(data: &[u8], server_url: &str) {
    let client = Client::new();
    let response = client
        .post(server_url)
        .body(data.to_vec())
        .send();

    match response {
        Ok(response) => {
            if response.status().is_success() {
                println!("File sent successfully to the remote server.");
            } else {
                println!("Failed to send file. Error code: {}", response.status());
            }
        }
        Err(err) => {
            println!("Failed to send file: {}", err);
        }
    }
}

/// Main function to orchestrate file finding, decryption, and sending.
///
/// # Example
/// Run the program from the command line with the following arguments:
/// ```

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} <start_dir> <target_file> <key_file>", args[0]);
        return;
    }

    let start_dir = Path::new(&args[1]);
    let target_file = &args[2];
    let key_file = &args[3];
    let server_url = "http://localhost:8000";

    let (target_path, key_path) = find_files(start_dir, target_file, key_file);

    if let (Some(target_path), Some(key_path)) = (target_path, key_path) {
        println!("Found target file: {}", target_path);
        println!("Found key file: {}", key_path);

        let target_file_contents = fs::read_to_string(&target_path).unwrap_or_else(|_| String::from("Failed to read target file"));
        let key_file_contents = fs::read_to_string(&key_path).unwrap_or_else(|_| String::from("Failed to read key file"));

        println!("\nTarget file contents:\n{}", target_file_contents);
        println!("\nKey file contents:\n{}", key_file_contents);

        let decrypted_data = decrypt_file(&target_path, &key_path);
        send_file(&decrypted_data, server_url);
    } else {
        println!("Failed to find the required files.");
    }
}