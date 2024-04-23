use std::env;
use std::fs;
use std::io::{Read};
use std::path::Path;
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE};

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

    let entries = match fs::read_dir(start_dir) {
        Ok(entries) => entries,
        Err(e) => {
            println!("Failed to read directory: {}", e);
            return (None, None);
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                println!("Failed to process an entry: {}", e);
                continue;
            }
        };
        let path = entry.path();

        if path.is_dir() {
            if let (Some(target_path_nested), Some(key_path_nested)) = find_files(&path, target_file, key_file) {
                target_path = Some(target_path_nested);
                key_path = Some(key_path_nested);
            }
        } else {
            if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                if file_name == target_file {
                    target_path = Some(path.to_str().unwrap().to_owned());
                } else if file_name == key_file {
                    key_path = Some(path.to_str().unwrap().to_owned());
                }
            }
        }
    }

    (target_path, key_path)
}


/// Decrypts a file using a specified key file, applying a simple XOR decryption algorithm.
///
/// This function opens the specified `target_path` and `key_path`, reads their contents, and then
/// applies a byte-wise XOR operation between the ciphertext from the target file and the key.
/// The key is repeated if it is shorter than the ciphertext. The function requires both files
/// to exist and be readable; additionally, the key file must not be empty as it is essential for
/// decryption.
///
/// # Arguments
/// * `target_path` - The path to the file to decrypt. This should be a string slice pointing to a valid
///   filesystem path of a readable file.
/// * `key_path` - The path to the key file used for decryption. Similarly, this should point to a readable
///   file. The function checks that this file is not empty since an empty key cannot perform decryption.
///
/// # Returns
/// A `Result<Vec<u8>, String>` encapsulating the outcome of the operation:
/// - `Ok(Vec<u8>)`: Contains the decrypted data assuming no errors occurred.
/// - `Err(String)`: An error message describing why the decryption could not be completed. Possible
///   reasons include file access issues, read errors, or an empty key file.
///
/// # Errors
/// The function might return an error in the following scenarios:
/// - Failure to open either the target file or the key file, possibly due to incorrect paths or insufficient
///   permissions.
/// - Failure to read from the opened files, which might be due to I/O errors.
/// - The key file is found to be empty after reading its contents, which makes decryption impossible.
///
/// # Example
/// ```
/// let target_path = "path/to/encrypted.txt";
/// let key_path = "path/to/key.txt";
/// match decrypt_file(target_path, key_path) {
///     Ok(decrypted_data) => {
///         println!("Decryption successful!");
///         // Additional code to handle the decrypted data (e.g., saving to a file or processing further)
///     },
///     Err(e) => println!("Decryption failed: {}", e),
/// }
/// ```

fn decrypt_file(target_path: &str, key_path: &str) -> Result<Vec<u8>, String> {
    let mut key_file = fs::File::open(key_path)
        .map_err(|e| format!("Failed to open key file: {}", e))?;

    let mut key = Vec::new();
    key_file.read_to_end(&mut key)
        .map_err(|e| format!("Failed to read key file: {}", e))?;

    let mut target_file = fs::File::open(target_path)
        .map_err(|e| format!("Failed to open target file: {}", e))?;

    let mut ciphertext = Vec::new();
    target_file.read_to_end(&mut ciphertext)
        .map_err(|e| format!("Failed to read target file: {}", e))?;

    if key.is_empty() {
        return Err("Key file is empty, cannot decrypt".to_string());
    }

    let plaintext: Vec<u8> = ciphertext.iter().zip(key.iter().cycle())
        .map(|(c, k)| c ^ k)
        .collect();

    Ok(plaintext)
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

fn send_file(data: &[u8], server_url: &str) -> Result<(), String> {
    let client = Client::new();
    println!("Sending POST request to: {}", server_url); // Debug statement
    let response = client.post(server_url)
                         .header(CONTENT_TYPE, "text/plain")  // Make sure this matches your server expectation
                         .body(data.to_vec())
                         .send();

    match response {
        Ok(response) => {
            if response.status().is_success() {
                println!("File sent successfully to the remote server.");
                Ok(())
            } else {
                Err(format!("Server responded with status code: {}", response.status()))
            }
        },
        Err(err) => {
            Err(format!("Failed to send file: {}", err))
        }
    }
}

/// Orchestrates the process of finding, decrypting, and sending a file.
///
/// This program requires three command-line arguments:
/// 1. `start_dir` - The directory from which the search for files begins.
/// 2. `target_file` - The name of the file to find and decrypt.
/// 3. `key_file` - The name of the key file used for decryption.
///
/// The program searches for the `target_file` and `key_file` starting from `start_dir`.
/// If found, it attempts to decrypt the `target_file` using the contents of the `key_file` and then
/// sends the decrypted data to a predefined server endpoint.
///
/// # Usage
/// Run the program from the command line as follows:
/// ```bash
/// cargo run <start_dir> <target_file> <key_file>
/// ```
/// Example:
/// ```bash
/// cargo run /path/to/search /path/to/target/file.txt /path/to/key/file.txt
/// ```
///
/// # Behavior
/// - **File Finding**: Searches recursively starting from `start_dir` for both `target_file` and `key_file`.
/// - **Decryption**: Uses the XOR decryption method (defined in `decrypt_file`).
/// - **Transmission**: Sends the decrypted data to `http://localhost:8080/post_endpoint`.
///
/// # Errors
/// - Exits with an error message if the incorrect number of arguments is provided.
/// - Reports failures during the file finding, decryption, or file sending processes.
///

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <start_dir> <target_file> <key_file>", args[0]);
        std::process::exit(1);
    }

    let start_dir = Path::new(&args[1]);
    let target_file = &args[2];
    let key_file = &args[3];
    let server_url = "http://localhost:8080/post_endpoint";

    let (target_path, key_path) = find_files(start_dir, target_file, key_file);

    match (target_path, key_path) {
        (Some(target_path), Some(key_path)) => {
            println!("Found target file: {}", target_path);
            println!("Found key file: {}", key_path);
            println!();

            let target_file_contents = fs::read_to_string(&target_path)
            .expect("Something went wrong reading the target file");
            let key_file_contents = fs::read_to_string(&key_path)
            .expect("Something went wrong reading the key file");

            println!("Target file contents:\n{}", target_file_contents);
            println!("Key file contents:\n{}", key_file_contents);
            println!();

            match decrypt_file(&target_path, &key_path) {
                Ok(decrypted_data) => {
                    match send_file(&decrypted_data, server_url) {
                        Ok(_) => println!("File was sent successfully."),
                        Err(e) => eprintln!("Failed to send file: {}", e),
                    }
                },
                Err(e) => eprintln!("Error decrypting file: {}", e),
            }
        },
        _ => eprintln!("Failed to find the required files."),
    }
}
