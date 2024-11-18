/* SPDX-License-Identifier: GPL-2.0-only OR GPL-3.0-only  */
/* Copyright (c) 2021 - 2024 Gavin Henry <ghenry@sentrypeer.org> */
/*
   _____            _              _____
  / ____|          | |            |  __ \
 | (___   ___ _ __ | |_ _ __ _   _| |__) |__  ___ _ __
  \___ \ / _ \ '_ \| __| '__| | | |  ___/ _ \/ _ \ '__|
  ____) |  __/ | | | |_| |  | |_| | |  |  __/  __/ |
 |_____/ \___|_| |_|\__|_|   \__, |_|   \___|\___|_|
                              __/ |
                             |___/
*/
use crate::tls::Config;

/// `Config` implements `Default`
impl Default for Config {
    fn default() -> Self {
        Self {
            cert: "cert.pem".into(),
            key: "key.pem".into(),
            tls_listen_address: "0.0.0.0:5061".into(),
        }
    }
}

pub fn load_file() -> Result<Config, confy::ConfyError> {
    let cfg = confy::load("sentrypeer", None)?;
    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn setup_config_file() {
        let cfg = Config {
            cert: "cert.pem".into(),
            key: "key.pem".into(),
            tls_listen_address: "0.0.0.0:5061".into(),
        };
        confy::store("sentrypeer", None, cfg).unwrap();
    }

    #[test]
    fn test_config_default() {
        let cfg = Config::default();
        assert_eq!(cfg.cert, PathBuf::from("cert.pem"));
        assert_eq!(cfg.key, PathBuf::from("key.pem"));
        assert_eq!(cfg.tls_listen_address, "0.0.0.0:5061");
    }

    #[test]
    fn test_load_file() {
        setup_config_file();

        let cfg: Config = load_file().unwrap();
        assert_eq!(cfg.cert, PathBuf::from("cert.pem"));
        assert_eq!(cfg.key, PathBuf::from("key.pem"));
        assert_eq!(cfg.tls_listen_address, "0.0.0.0:5061");
    }

    #[test]
    fn test_load_file_error() {
        let cfg: Result<Config, confy::ConfyError> = load_file();
        assert!(cfg.is_ok());
    }

    #[test]
    fn test_load_file_and_save() {
        let cfg: Config = load_file().unwrap();
        assert_eq!(cfg.cert, PathBuf::from("cert.pem"));
        assert_eq!(cfg.key, PathBuf::from("key.pem"));
        assert_eq!(cfg.tls_listen_address, "0.0.0.0:5061");
        let cfg = Config {
            cert: "cert2.pem".into(),
            key: "key2.pem".into(),
            tls_listen_address: "0.0.0.0:5062".into(),
        };
        confy::store("sentrypeer", None, cfg).unwrap();

        // Reset to original
        setup_config_file();
    }
}
