#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_file_success() {
        let data = b"test_data";
        let server_url = "http://localhost:8000"; // Ensure this server is expected to return success

        // Call the function and expect an Ok result
        let result = send_file(data, server_url);
        assert!(result.is_ok(), "Expected Ok(()) but got Err({})", result.unwrap_err());
    }

    #[test]
    fn test_send_file_failure() {
        let data = b"test_data";
        let server_url = "http://localhost:8001"; // Ensure this server is expected to return failure

        // Call the function and expect an Err result
        let result = send_file(data, server_url);
        assert!(result.is_err(), "Expected Err but got Ok(())");
    }
}
