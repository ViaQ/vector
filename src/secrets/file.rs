use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::Path;
use std::io::{Read, ErrorKind};

use vector_lib::configurable::{component::GenerateConfig, configurable_component};

use crate::{config::SecretBackend, signal};

/// Configuration for the `file` secrets backend.
#[configurable_component(secrets("file"))]
#[derive(Clone, Debug)]
pub struct FileBackend {
    /// Base path to use for fetching secrets from files.
    pub base_path: String,
}

impl GenerateConfig for FileBackend {
    fn generate_config() -> toml::Value {
        toml::Value::try_from(FileBackend {
            base_path: String::from("/path/to/secrets"),
        })
        .unwrap()
    }
}

impl SecretBackend for FileBackend {
    fn retrieve(
        &mut self,
        secret_keys: HashSet<String>,
        _signal_rx: &mut signal::SignalRx,
    ) -> crate::Result<HashMap<String, String>> {
        return retrieve_secrets(&self.base_path, secret_keys);
    }
}

fn retrieve_secrets(
    base: &String,
    secret_keys: HashSet<String>,
) -> crate::Result<HashMap<String, String>> {
    let base_path = Path::new(&base);
    let mut secrets = HashMap::new();
    for k in secret_keys.into_iter() {
        let secret_path = Path::new(&k);
        if secret_path.is_absolute() {
            return Err(format!("secret key can not be absolute: {}", k).into());
        }

        let file_path = base_path.join(secret_path);
        let file_result = File::open(file_path);
        match file_result {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                if contents.is_empty() {
                    return Err(format!("secret for key '{}' was empty", k).into());
                }

                secrets.insert(k.to_string(), contents);
            }
            Err(error) => {
                if error.kind() == ErrorKind::NotFound {
                    return Err(format!("secret file for '{}' not found", k).into());
                }
                return Err(format!("error reading file for '{}': {}", k, error).into());
            }
        }
    }
    Ok(secrets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retrieves_secret() {
        let base_path = String::from("tests/secrets/");
        let mut secret_keys = HashSet::new();
        secret_keys.insert(String::from("secret"));

        let result = retrieve_secrets(&base_path, secret_keys);
        let values = result.expect("error reading secret");
        assert!(values.contains_key("secret"));
        assert_eq!(values.get("secret").unwrap(), "value\n");
    }

    #[test]
    fn reject_absolute_path() {
        let base_path = String::from("tests/secrets/");
        let mut secret_keys = HashSet::new();
        secret_keys.insert(String::from("/absolute/path"));

        let result = retrieve_secrets(&base_path, secret_keys);
        let error = result.expect_err("absolute key should produce error");
        assert_eq!(error.to_string(), "secret key can not be absolute: /absolute/path");
    }

    #[test]
    fn reject_empty_secret() {
        let base_path = String::from("tests/secrets/");
        let mut secret_keys = HashSet::new();
        secret_keys.insert(String::from("empty"));

        let result = retrieve_secrets(&base_path, secret_keys);
        let error = result.expect_err("empty secret should produce error");
        assert_eq!(error.to_string(), "secret for key 'empty' was empty");
    }

    #[test]
    fn secret_not_found() {
        let base_path = String::from("tests/secrets/");
        let mut secret_keys = HashSet::new();
        secret_keys.insert(String::from("does_not_exist"));

        let result = retrieve_secrets(&base_path, secret_keys);
        let error = result.expect_err("secret should not be found");
        assert_eq!(error.to_string(), "secret file for 'does_not_exist' not found");
    }
}
