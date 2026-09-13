use std::path::Path;

pub fn create_dotenv_frontend(api_url: &str, dotenv_path: &str) {
    let path = Path::new(dotenv_path);
    let mut lines: Vec<String> = Vec::new();

    if path.exists() {
        if let Ok(file_content) = std::fs::read_to_string(path) {
            lines = file_content.lines().map(|s| s.to_string()).collect();
        }
    }

    // Helper to update or append a key
    let mut update_or_append = |key: &str, value: &str| {
        let entry = format!("{}={}", key, value);
        let mut found = false;
        for line in lines.iter_mut() {
            if line.starts_with(&format!("{}=", key)) {
                *line = entry.clone();
                found = true;
                break;
            }
        }
        if !found {
            lines.push(entry);
        }
    };

    update_or_append("PUBLIC_API_URL", api_url);

    // Join lines with newlines and write back
    let new_content = lines.join("\n");
    let final_content = if new_content.is_empty() {
        new_content
    } else {
        new_content + "\n"
    };

    if let Err(e) = std::fs::write(path, final_content) {
        eprintln!("Failed to write to {}: {}", dotenv_path, e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Read, Write};

    #[test]
    fn test_create_dotenv_frontend_new_file() {
        let api_url = "https://api.example.com";
        let dotenv_path = "./src/frontend/.test-new-env";

        // Create a temporary file for testing
        create_dotenv_frontend(api_url, &dotenv_path);

        // Read the contents of the temporary file
        let mut file = File::open(dotenv_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Assert that the file has been created and contains the correct value
        assert_eq!(contents, format!("PUBLIC_API_URL={}\n", api_url));

        // remove the temporary file
        std::fs::remove_file(dotenv_path).unwrap();
    }

    #[test]
    fn test_create_dotenv_frontend_existing_file() {
        let api_url = "https://api.example.com";
        let dotenv_path = "./src/frontend/.test-exist-env";

        // Create a temporary file for testing
        let mut file = File::create(&dotenv_path).unwrap();
        file.write_all("PUBLIC_API_URL=old_value".as_bytes())
            .unwrap();

        // Update the file with the new value
        create_dotenv_frontend(api_url, &dotenv_path);

        // Read the contents of the temporary file
        let mut file = File::open(dotenv_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Assert that the file has been updated with the new value
        assert_eq!(contents, format!("PUBLIC_API_URL={}\n", api_url));

        // remove the temporary file
        std::fs::remove_file(dotenv_path).unwrap();
    }

    #[test]
    fn test_create_dotenv_frontend_append_new_key() {
        let api_url = "https://api.example.com";
        let dotenv_path = "./src/frontend/.test-append-env";

        // Create initial file
        {
            let mut file = File::create(dotenv_path).unwrap();
            file.write_all(b"PUBLIC_API_URL=old_url\nEXISTING_VAR=keep_me\n")
                .unwrap();
        }

        // Update
        create_dotenv_frontend(api_url, dotenv_path);

        // Verify
        let contents = std::fs::read_to_string(dotenv_path).unwrap();

        assert!(contents.contains(&format!("PUBLIC_API_URL={}", api_url)));
        assert!(contents.contains("EXISTING_VAR=keep_me"));

        // Cleanup
        std::fs::remove_file(dotenv_path).unwrap();
    }

    #[test]
    fn test_create_dotenv_frontend_survives_an_unwritable_path() {
        // A folder that does not exist cannot be written to, the cli reports it
        // and carries on instead of panicking
        let dotenv_path = "./no-such-folder/.env";

        create_dotenv_frontend("https://api.example.com", dotenv_path);

        assert!(!std::path::Path::new(dotenv_path).exists());
    }
}
