// use std::process::Output;

use configuration_parameters::{get_configuration_parameters, ConfigurationParameters};


use crate::configuration_parameters;
// use crate::macros;

pub fn init_loggers(app_name: &str) -> ConfigurationParameters {
    let config_params = get_configuration_parameters(app_name);
    let emp_path = config_params.emp_file_path().to_string();
    let dept_path = config_params.dept_file_path().to_string();
    let salary_path = config_params.sal_file_path().to_string();
    let leave_path = config_params.leave_file_path().to_string();
    let output_path = config_params.output_file_path().to_string();
    
    ConfigurationParameters {
        emp_path,
        dept_path,
        salary_path,
        leave_path,
        output_path,
    }
}
