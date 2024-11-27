use calamine::{open_workbook_auto, DataType, Reader};
use chrono::{Datelike, NaiveDate, Utc};
use clap::{Arg, Command};
use csv::ReaderBuilder;
use std::fs::File;
use std::io::Write;
use std::{error::Error, io::BufWriter};



#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct Emp {   //Struct for the employee data
    emp_id: i32,
    emp_name: String,
    dept_id: i32,
    mobile_no: String,
    email: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Dept {     //Struct for the Department data
    dept_id: i32,
    dept_title: String,
    dept_strength: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Salent {    //Struct for the Salary data
    emp_id: i32,
    sal_id: i32,
    sal_date: String,
    sal: f32,
    status: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Leav {          //Struct for the Leave data
    emp_id: i32,
    leave_id: i32,
    leave_from: NaiveDate,
    leave_to: NaiveDate,
    leave_type: String,
}

// fuction to read the emp text file and pass it data in vector<struct> form
fn read_text_file(file_path: &str) -> Result<Vec<Emp>, Box<dyn Error>> {
    let file = File::open(file_path).expect("Parsing Error");//open the input file
    let mut reader = ReaderBuilder::new()//reading the input file
        .delimiter(b'|') 
        .has_headers(true) 
        .from_reader(file);

    let mut employees = Vec::new();

    //reading the lines of file and store them in vector one by one
    for result in reader.deserialize() {
        let employee: Emp = result.expect("Parsing Error");
        employees.push(employee);
    }
    Ok(employees)
}

//funtion to read the dept file and give the data of it in vector<struct> form
fn read_dept_file(file_path: &str, sheet_name: &str) -> Result<Vec<Dept>, Box<dyn Error>> {
    let mut workbook = open_workbook_auto(file_path).expect("Parsing Error");
    let range = workbook.worksheet_range(sheet_name).expect("Parsing Error");

    let mut departments = Vec::new();
    for row in range.rows().skip(1) {
        let dept_id = row[0].get_float().ok_or_else(|| "Invalid Dept ID").expect("Parsing Error") as i32;
        let dept_title = row[1]
            .get_string()
            .ok_or_else(|| "Invalid Dept Title").expect("Parsing Error")
            .to_string();
        let dept_strength = row[2].get_float().ok_or_else(|| "Invalid Dept Strength").expect("Parsing Error") as i32;
        departments.push(Dept {
            dept_id,
            dept_title,
            dept_strength,
        });
    }
    Ok(departments)
}

//funtion to read the sal file and give the data of it in vector<struct> form
fn read_sal_file(file_path: &str, sheet_name: &str) -> Result<Vec<Salent>, Box<dyn Error>> {
    
    let mut workbook = open_workbook_auto(file_path).expect("Parsing Error");
    let range = workbook.worksheet_range(sheet_name).expect("Parsing Error");

    let mut salaries = Vec::new();
    for row in range.rows().skip(1) {
        //println!("{:?}",row);
        let salary = Salent {
            emp_id: row[0].get_float().ok_or("Invalid Emp ID").expect("Parsing Error") as i32,
            sal_id: row[1].get_float().ok_or("Invalid Salary Id").expect("Parsing Error") as i32,
            sal_date: (row[2].get_string().ok_or("Invalid Salaray Date").expect("Parsing Error") as &str).to_string(),
            sal: row[3].get_float().ok_or("Invalid Salary").expect("Parsing Error") as f32,
            status: (row[4].get_string().ok_or("Invalid Dept Strength").expect("Parsing Error") as &str).to_owned(),
        };
        salaries.push(salary);
    }
    Ok(salaries)
}

//funtion to read the leave file and give the data of it in vector<struct> form
#[allow(deprecated)]
fn read_leave_file(file_path: &str, sheet_name: &str) -> Result<Vec<Leav>, Box<dyn Error>> {

    let mut workbook = open_workbook_auto(file_path).expect("Parsing Error");
    let range = workbook.worksheet_range(sheet_name).expect("Parsing Error");

    let mut leaves = Vec::new();

    for row in range.rows().skip(1) {
        //println!("{:?}",row);
        let emp_id = row[0].get_float().ok_or("Invalid EMP ID").expect("Parsing Error") as i32;
        //println!("{:?}",emp_id);
        let leave_id = row[1].get_float().ok_or("Invalid Leave ID").expect("Parsing Error") as i32;

        let leave_from_date_str = row[2].get_string().unwrap_or_default();
        let leave_to_date_str = row[3].get_string().unwrap_or_default();

        let leave_from = NaiveDate::parse_from_str(leave_from_date_str, "%e-%m-%Y")
            .unwrap_or_else(|_| NaiveDate::from_ymd(1970, 1, 1)); // Use default date if parsing fails
        let leave_to = NaiveDate::parse_from_str(leave_to_date_str, "%e-%m-%Y")
            .unwrap_or_else(|_| NaiveDate::from_ymd(1970, 1, 1));

        let leave_type = row[4].get_string().ok_or("Invalid Leave Type").expect("Parsing Error").to_string();

        let leave_entry = Leav {
            emp_id,
            leave_id,
            leave_from,
            leave_to,
            leave_type,
        };

        leaves.push(leave_entry);
    }

    Ok(leaves)
}

//funtion to cal max day in a month
#[allow(deprecated)]
fn max_days_in_month(year: i32, month: u32) -> u32 {
    let first_day_of_next_month = if month == 12 {
        NaiveDate::from_ymd(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd(year, month + 1, 1)
    };

    let last_day_of_month = first_day_of_next_month.pred(); 
    last_day_of_month.day() 
}


//Generating the required output form the all passing data....
#[allow(deprecated)]
fn generate_output(
    emp_data: Vec<Emp>,
    dept_data: Vec<Dept>,
    sal_data: Vec<Salent>,
    leave_data: Vec<Leav>,
    output_path: &str,
) -> Result<(), Box<dyn Error>> {
   
    let dept_map: std::collections::HashMap<i32, String> = dept_data
        .into_iter()
        .map(|dept| (dept.dept_id, dept.dept_title))
        .collect();
    
    let current_month = Utc::now().month();

    let file = File::create(output_path).expect("Parsing Error");
    let mut write_handler = BufWriter::new(file);

    writeln!(
        write_handler,
        "Emp ID~#~Emp Name~#~Dept Title~#~Mobile No~#~Email~#~Salary Status~#~On Leave"
    ).expect("Parsing Error");

    for emp in emp_data {
        
        let dept_title = dept_map
            .get(&emp.dept_id)
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string());

        let salary_status = sal_data
            .iter()
            .find(|&sal| {
                sal.emp_id == emp.emp_id
                    && sal.sal_date.contains(&format!("-{:02}-", current_month))
            })
            .map(|_| "Credited")
            .unwrap_or("Not Credited");

        let total_leave_days: i64 = leave_data
            .iter()
            .filter(|leave| leave.emp_id == emp.emp_id)
            .map(|leave| {
                let start_month = leave.leave_from.month();
                let end_month = leave.leave_to.month();
                if start_month == current_month && end_month == current_month {
                    let start_date = leave.leave_from;
                    let end_date = leave.leave_to;

                    (end_date - start_date).num_days() + 1
                } else if start_month == current_month {
                    let start_date = leave.leave_from;
                    let end_date = NaiveDate::from_ymd(
                        leave.leave_from.year(),
                        current_month,
                        max_days_in_month(Utc::now().year(), current_month),
                    );
                    (end_date - start_date).num_days() + 1
                } else if end_month == current_month {
                    let start_date = NaiveDate::from_ymd(Utc::now().year(), end_month, 1);
                    let end_date = leave.leave_to;

                    (end_date - start_date).num_days() + 1
                } else {
                    0
                }
            })
            .sum();

        // writing the required data to the output file    
        writeln!(
            write_handler,
            "{}~#~{}~#~{}~#~{}~#~{}~#~{}~#~{}",
            emp.emp_id,
            emp.emp_name,
            dept_title,
            emp.mobile_no,
            emp.email,
            salary_status,
            total_leave_days
        ).expect("Parsing Error");
    }

    println!("Output generated successfully: Go Check the output File");
    Ok(())
}

fn main() {
    let matches = Command::new("Report Generating System")
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

    //Reading the input of all file path through Clap!!!!!!    
    let emp_path = matches.get_one::<String>("EMP_FILE").expect("Parsing Error");
    let dept_path = matches.get_one::<String>("DEPT_FILE").expect("Parsing Error");
    let salary_path = matches.get_one::<String>("SAL_FILE").expect("Parsing Error");
    let leave_path = matches.get_one::<String>("LEAVE_FILE").expect("Parsing Error");
    let output_path = matches.get_one::<String>("OUTPUT_FILE").expect("Parsing Error");

    let emp_data = read_text_file(emp_path).expect("Failed to read employee data");
    let dept_data = read_dept_file(dept_path, "Sheet1").expect("Failed to read department data");
    let sal_data = read_sal_file(salary_path, "Sheet1").expect("Failed to read salary data");
    let leave_data = read_leave_file(leave_path, "Sheet1").expect("Failed to read leave data");

    generate_output(emp_data, dept_data, sal_data, leave_data, output_path)
        .expect("Failed to generate output");
}

// To run execute the below command on terminal.....
// cargo run --release -- -e emp_file_path -d dept_file_path -s sal_file_path -l leave_file_path -o output_file_path