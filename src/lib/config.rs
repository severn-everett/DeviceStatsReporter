use std::error::Error;

use crate::lib::common::{IllegalArgumentError, RuntimeError, RuntimeMode};

pub struct RunnerConfig {
    pub runtime_mode: RuntimeMode,
    pub check_interval: u64,
}

const RUNTIME_MODE_KEY: &str = "runtime_mode";
const SINGLE_RUNTIME_MODE: &str = "Single";
const CONTINUOUS_RUNTIME_MODE: &str = "Continuous";
const CHECK_INTERVAL_KEY: &str = "check_interval";
const DEFAULT_CHECK_INTERVAL: u64 = 1;
const MINIMUM_CHECK_INTERVAL: u64 = DEFAULT_CHECK_INTERVAL;
const MAXIMUM_CHECK_INTERVAL: u64 = 240;

pub fn load_config(config_path: Option<&String>) -> Result<RunnerConfig, Box<dyn Error>> {
    let mut runner_config = RunnerConfig {
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
    match settings.get_str(RUNTIME_MODE_KEY) {
        Ok(mode) => {
            match mode.as_str() {
                CONTINUOUS_RUNTIME_MODE => {
                    runner_config.runtime_mode = RuntimeMode::Continuous;
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
                            let error = Box::new(
                                IllegalArgumentError::new(e.to_string().as_str())
                            );
                            return Err(error);
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

    use crate::lib::common::{IllegalArgumentError, RuntimeMode};
    use crate::lib::config::{DEFAULT_CHECK_INTERVAL, load_config};

    #[test]
    fn load_default_config() {
        let result = load_config(None).unwrap();
        assert_eq!(RuntimeMode::Single, result.runtime_mode);
        assert_eq!(DEFAULT_CHECK_INTERVAL, result.check_interval);
    }

    #[test]
    fn load_standalone_config() {
        let result = load_config(
            Some(&String::from("resources/test/good/ideal_standalone.yaml"))
        ).unwrap();
        assert_eq!(RuntimeMode::Single, result.runtime_mode);
        assert_eq!(DEFAULT_CHECK_INTERVAL, result.check_interval);
    }

    #[test]
    fn load_continuous_config() {
        let result = load_config(
            Some(&String::from("resources/test/good/ideal_continuous.yaml"))
        ).unwrap();
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