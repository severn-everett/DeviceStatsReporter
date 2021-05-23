use crate::lib::common::{RuntimeMode, RuntimeError, IllegalArgumentError, MINUTES_MULTIPLIER};
use std::error::Error;
use std::env;

pub struct RunnerConfig {
    pub runtime_mode: RuntimeMode,
    pub check_interval: u64,
}

const RUNTIME_MODE_KEY: &str = "runtime_mode";
const SINGLE_RUNTIME_MODE: &str = "Single";
const CONTINUOUS_RUNTIME_MODE: &str = "Continuous";
const CHECK_INTERVAL_KEY: &str = "check_interval";
const DEFAULT_WAIT_INTERVAL: u64 = 1;

pub fn load_config() -> Result<RunnerConfig, Box<dyn Error>> {
    let mut runner_config = RunnerConfig {
        runtime_mode: RuntimeMode::Single,
        check_interval: DEFAULT_WAIT_INTERVAL,
    };
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Ok(runner_config);
    }
    let config_path = args.get(1).unwrap();
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
                            if (u64::MAX / MINUTES_MULTIPLIER) > check_interval {
                                runner_config.check_interval = check_interval;
                            } else {
                                let error = Box::new(
                                    IllegalArgumentError::new("Check interval set too high")
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