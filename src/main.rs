// mod macros;
mod init;
mod configuration_parameters;
mod log;
mod process;

//use std::process::Output;
use log::read_dept_file;
use log::read_leave_file;
use log::read_sal_file;
use log::read_text_file;

use std::time::SystemTime;
use init::init_loggers;
use process::generate_output;

// #[macro_use]
// extern crate slog;



fn main() {
    let _start_aggregation_timer = SystemTime::now();
    let app_name = "Genrating text file";
    let config_para = init_loggers(app_name);

    let emp_path=config_para.emp_path;
    let dept_path=config_para.dept_path;
    let sal_path=config_para.salary_path;
    let leave_path=config_para.leave_path;
    let output_path=config_para.output_path;

   

    let emp_data = read_text_file(&emp_path).expect("Failed to read employee data");
    let dept_data = read_dept_file(&dept_path, "Sheet1").expect("Failed to read department data");
    let sal_data = read_sal_file(&sal_path, "Sheet1").expect("Failed to read salary data");
    let leave_data = read_leave_file(&leave_path, "Sheet1").expect("Failed to read leave data");

    generate_output(emp_data, dept_data, sal_data, leave_data, &output_path)
        .expect("Failed to generate output");
}

// To run execute the below command on terminal.....
// cargo run --release -- -e emp_file_path -d dept_file_path -s sal_file_path -l leave_file_path -o output_file_path