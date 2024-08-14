/// Creates or updates a `.env` file with the specified API URL.
///
/// # Arguments
///
/// * `api_url` - The API URL to be set in the `.env` file.
/// * `dotenv_path` - The path to the `.env` file.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::Read;
/// use std::io::Write;
/// use std::path::Path;
///
/// fn replace_value(contents: &String, key: &str, new_value: &str) -> String {
///     // implementation details omitted
/// }
///
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///
///     #[test]
///     fn test_create_dotenv_frontend_new_file() {
///         // test implementation omitted
///     }
///
///     #[test]
///     fn test_create_dotenv_frontend_existing_file() {
///         // test implementation omitted
///     }
///
///     #[test]
///     fn test_replace_value_existing_key() {
///         // test implementation omitted
///     }
///
///     #[test]
///     fn test_replace_value_non_existing_key() {
///         // test implementation omitted
///     }
/// }
/// ```
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn create_dotenv_frontend(api_url: &str, dotenv_path: &str) {
    let dotenv_exists: bool = Path::new(dotenv_path).exists();
    if dotenv_exists {
        match File::open(dotenv_path) {
            Ok(mut file) => {
                let mut contents = String::new();
                if let Err(err) = file.read_to_string(&mut contents) {
                    eprintln!("Error reading file: {}", err)
                }
                let new_contents = replace_value(&contents, "PUBLIC_API_URL=", api_url);
                if let Err(err) = File::create(dotenv_path)
                    .and_then(|mut file| file.write_all(new_contents.as_bytes()))
                {
                    eprintln!("Error writing to file: {}", err);
                }
            }
            Err(err) => {
                eprintln!("Error opening file: {}", err)
            }
        }
    } else {
        match File::create(dotenv_path) {
            Ok(mut file) => {
                if let Err(err) = file.write_all(format!("PUBLIC_API_URL={}", api_url).as_bytes()) {
                    eprintln!("Error writing to file: {}", err)
                }
            }
            Err(err) => {
                eprintln!("Error creating file: {}", err)
            }
        }
    }
}

fn replace_value(contents: &String, key: &str, new_value: &str) -> String {
    if let Some(index) = contents.find(key) {
        let (_, old_value) = contents.split_at(index + key.len());
        contents.replace(old_value, new_value)
    } else {
        contents.to_string()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(contents, format!("PUBLIC_API_URL={}", api_url));

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
        assert_eq!(contents, format!("PUBLIC_API_URL={}", api_url));

        // remove the temporary file
        std::fs::remove_file(dotenv_path).unwrap();
    }

    #[test]
    fn test_replace_value_existing_key() {
        let contents = String::from("PUBLIC_API_URL=old_value");
        let key = "PUBLIC_API_URL=";
        let new_value = "https://api.example.com";

        let result = replace_value(&contents, key, new_value);

        assert_eq!(result, format!("PUBLIC_API_URL={}", new_value));
    }

    #[test]
    fn test_replace_value_non_existing_key() {
        let contents = String::from("OTHER_KEY=value");
        let key = "PUBLIC_API_URL=";
        let new_value = "https://api.example.com";

        let result = replace_value(&contents, key, new_value);

        assert_eq!(result, contents);
    }
}
