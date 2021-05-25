use std::error::Error;
use uuid::Uuid;

use crate::lib::common::{IllegalArgumentError, RuntimeError, RuntimeMode};
use config::ConfigError;

pub struct RunnerConfig {
    pub device_name: String,
    pub server_address: String,
    pub topic: String,
    pub runtime_mode: RuntimeMode,
    pub check_interval: u64,
}

// Configuration key names
const DEVICE_NAME_KEY: &str = "device_name";
const RUNTIME_MODE_KEY: &str = "runtime_mode";
const CHECK_INTERVAL_KEY: &str = "check_interval";
const SERVER_ADDRESS_KEY: &str = "server_address";
const TOPIC_KEY: &str = "topic";
// Configuration values
const DEFAULT_SERVER_ADDRESS: &str = "tcp://localhost:1883";
const DEFAULT_TOPIC: &str = "Device_Status";
const SINGLE_RUNTIME_MODE: &str = "Single";
const CONTINUOUS_RUNTIME_MODE: &str = "Continuous";
const DEFAULT_CHECK_INTERVAL: u64 = 1;
const MINIMUM_CHECK_INTERVAL: u64 = DEFAULT_CHECK_INTERVAL;
const MAXIMUM_CHECK_INTERVAL: u64 = 240;

pub fn load_config(config_path: Option<&String>) -> Result<RunnerConfig, Box<dyn Error>> {
    let mut runner_config = RunnerConfig {
        device_name: Uuid::new_v4().to_string(),
        server_address: String::from(DEFAULT_SERVER_ADDRESS),
        topic: String::from(DEFAULT_TOPIC),
        runtime_mode: RuntimeMode::Single,
        check_interval: DEFAULT_CHECK_INTERVAL,
    };
    let config_path = match config_path {
        Some(cp) => cp,
        None => return Ok(runner_config)
    };
    let mut settings = config::Config::default();
    match settings.merge(config::File::with_name(config_path)) {
        Ok(_) => {}
        Err(e) => {
            let error = Box::new(RuntimeError::new(e.to_string().as_str()));
            return Err(error);
        }
    };
    // Device name
    match settings.get_str(DEVICE_NAME_KEY) {
        Ok(device_name) => runner_config.device_name = device_name,
        Err(_) => {}
    };
    // Server address
    match settings.get_str(SERVER_ADDRESS_KEY) {
        Ok(server_address) => runner_config.server_address = server_address,
        Err(_) => {}
    };
    // Topic
    match settings.get_str(TOPIC_KEY) {
        Ok(topic) => runner_config.topic = topic,
        Err(_) => {}
    };
    // Runtime mode
    match settings.get_str(RUNTIME_MODE_KEY) {
        Ok(mode) => {
            match mode.as_str() {
                CONTINUOUS_RUNTIME_MODE => {
                    runner_config.runtime_mode = RuntimeMode::Continuous;
                    // Check interval
                    match settings.get(CHECK_INTERVAL_KEY) {
                        Ok(check_interval) => {
                            if check_interval >= MINIMUM_CHECK_INTERVAL && check_interval <= MAXIMUM_CHECK_INTERVAL {
                                println!("TEST INTERVAL: {}", check_interval);
                                runner_config.check_interval = check_interval;
                            } else {
                                let error = Box::new(
                                    IllegalArgumentError::new(
                                        format!("Check interval must be between {} and {}", MINIMUM_CHECK_INTERVAL, MAXIMUM_CHECK_INTERVAL).as_str()
                                    )
                                );
                                return Err(error);
                            }
                        }
                        Err(e) => {
                            match e {
                                ConfigError::NotFound(_) => {},
                                _ => {
                                    let error = Box::new(
                                        IllegalArgumentError::new(e.to_string().as_str())
                                    );
                                    return Err(error);
                                }
                            }
                        }
                    }
                }
                SINGLE_RUNTIME_MODE => runner_config.runtime_mode = RuntimeMode::Single,
                _ => {
                    let error = Box::new(
                        IllegalArgumentError::new(format!("Unexpected runtime mode '{}'", mode).as_str())
                    );
                    return Err(error);
                }
            };
        }
        Err(_) => {}
    };

    Ok(runner_config)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use pretty_assertions::assert_ne;

    use crate::lib::common::{IllegalArgumentError, RuntimeMode};
    use crate::lib::config::{DEFAULT_CHECK_INTERVAL, load_config, DEFAULT_SERVER_ADDRESS, DEFAULT_TOPIC};

    #[test]
    fn load_default_config() {
        let result = load_config(None).unwrap();
        assert_ne!("", result.device_name);
        assert_eq!(DEFAULT_SERVER_ADDRESS, result.server_address);
        assert_eq!(DEFAULT_TOPIC, result.topic);
        assert_eq!(RuntimeMode::Single, result.runtime_mode);
        assert_eq!(DEFAULT_CHECK_INTERVAL, result.check_interval);
    }

    #[test]
    fn load_bare_single_config() {
        let result = load_config(
            Some(&String::from("resources/test/good/bare_single.yaml"))
        ).unwrap();
        assert_ne!("", result.device_name);
        assert_eq!(DEFAULT_SERVER_ADDRESS, result.server_address);
        assert_eq!(DEFAULT_TOPIC, result.topic);
        assert_eq!(RuntimeMode::Single, result.runtime_mode);
    }

    #[test]
    fn load_bare_continuous_config() {
        let result = load_config(
            Some(&String::from("resources/test/good/bare_continuous.yaml"))
        ).unwrap();
        assert_ne!("", result.device_name);
        assert_eq!(DEFAULT_SERVER_ADDRESS, result.server_address);
        assert_eq!(DEFAULT_TOPIC, result.topic);
        assert_eq!(RuntimeMode::Continuous, result.runtime_mode);
        assert_eq!(DEFAULT_CHECK_INTERVAL, result.check_interval);
    }

    #[test]
    fn load_full_single_config() {
        let result = load_config(
            Some(&String::from("resources/test/good/full_single.yaml"))
        ).unwrap();
        assert_eq!("Test Device Name", result.device_name);
        assert_eq!("tcp://test.server.address:1883", result.server_address);
        assert_eq!("Test Topic", result.topic);
        assert_eq!(RuntimeMode::Single, result.runtime_mode);
        assert_eq!(DEFAULT_CHECK_INTERVAL, result.check_interval);
    }

    #[test]
    fn load_full_continuous_config() {
        let result = load_config(
            Some(&String::from("resources/test/good/full_continuous.yaml"))
        ).unwrap();
        assert_eq!("Test Device Name", result.device_name);
        assert_eq!("tcp://test.server.address:1883", result.server_address);
        assert_eq!("Test Topic", result.topic);
        assert_eq!(RuntimeMode::Continuous, result.runtime_mode);
        assert_eq!(5, result.check_interval);
    }

    #[test]
    fn load_unrecognized_runtime_mode() {
        let result = load_config(
            Some(&String::from("resources/test/bad/unrecognized_runtime_mode.yaml"))
        ).err().unwrap().downcast::<IllegalArgumentError>().unwrap();
        assert_eq!("An illegal argument was encountered. Reason: Unexpected runtime mode 'UNRECOGNIZED_MODE'", result.to_string());
    }

    #[test]
    fn load_negative_check_interval() {
        let result = load_config(
            Some(&String::from("resources/test/bad/negative_check_interval.yaml"))
        ).err().unwrap().downcast::<IllegalArgumentError>().unwrap();
        assert_eq!("An illegal argument was encountered. Reason: Check interval must be between 1 and 240", result.to_string());
    }

    #[test]
    fn load_too_high_check_interval() {
        let result = load_config(
            Some(&String::from("resources/test/bad/too_high_check_interval.yaml"))
        ).err().unwrap().downcast::<IllegalArgumentError>().unwrap();
        assert_eq!("An illegal argument was encountered. Reason: Check interval must be between 1 and 240", result.to_string());
    }

    #[test]
    fn load_bad_check_interval() {
        let result = load_config(
            Some(&String::from("resources/test/bad/bad_check_interval.yaml"))
        ).err()
            .unwrap()
            .downcast::<IllegalArgumentError>()
            .unwrap();
        assert!(result.to_string().contains("An illegal argument was encountered. Reason: invalid type: string \"FIVE\""));
    }
}