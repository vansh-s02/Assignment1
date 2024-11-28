
use clap::{self, ArgMatches};
use clap::{Command, Arg};

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

pub struct ConfigurationParameters {
    pub emp_path: String,
    pub dept_path: String,
    pub salary_path: String,
    pub leave_path: String,
    pub output_path: String,
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let emp_path = matches.get_one::<String>("EMP_FILE").expect("Error in Employee File Path").to_string();
        let dept_path = matches.get_one::<String>("DEPT_FILE").expect("Error in Employee File Path").to_string();
        let salary_path = matches.get_one::<String>("SAL_FILE").expect("Error in Employee File Path").to_string();
        let leave_path = matches.get_one::<String>("LEAVE_FILE").expect("Error in Employee File Path").to_string();
        let output_path =matches.get_one::<String>("OUTPUT_FILE").expect("Error in Employee File Path").to_string();
        

        ConfigurationParameters {
            emp_path,
            dept_path,
            salary_path,
            leave_path,
            output_path,
        }
    }
}

impl ConfigurationParameters {
    pub fn emp_file_path(&self) -> &str {
        &self.emp_path
    }
    pub fn dept_file_path(&self) -> &str {
        &self.dept_path
    }
    pub fn sal_file_path(&self) -> &str {
        &self.salary_path
    }
    pub fn leave_file_path(&self) -> &str {
        &self.leave_path
    }
    pub fn output_file_path(&self) -> &str {
        &self.output_path
    }
}

#[allow(unused_variables)]
fn get_eligible_arguments_for_app(app_name: &str)->ArgMatches {
    let matches = Command::new("Report Generating System ")
        .version("1.0")
        .author("Vansh Singhal")
        .about("Reads input files and processes them")
        .arg(
            Arg::new("EMP_FILE")
                .short('e')
                .long("emp-data-file-pat")
                .help("Path to the employee data text file")
                .required(true),
        )
        .arg(
            Arg::new("DEPT_FILE")
                .short('d')
                .long("dept-data-file-pat")
                .help("Path to the department data Excel file")
                .required(true),
        )
        .arg(
            Arg::new("SAL_FILE")
                .short('s')
                .long("salary-data-file-pat")
                .help("Path to the salary data Excel file")
                .required(true),
        )
        .arg(
            Arg::new("LEAVE_FILE")
                .short('l')
                .long("leave-data-file-pat")
                .help("Path to the leave data Excel file")
                .required(true),
        )
        .arg(
            Arg::new("OUTPUT_FILE")
                .short('o')
                .long("output-file-pat")
                .help("Path to the leave data Excel file")
                .required(true),
        )
        .get_matches();

    matches
}
