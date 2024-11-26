use calamine::{open_workbook_auto, DataType, Reader};
use chrono::{Datelike, NaiveDate, Utc};
use clap::{Arg, Command};
use csv::ReaderBuilder;
#[warn(dead_code)]
use std::fs::File;
use std::io::Write;
use std::{error::Error, io::BufWriter};

#[warn(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct EMPLOYEE {
    emp_id: i32,
    emp_name: String,
    dept_id: i32,
    mobile_no: String,
    email: String,
}
#[warn(dead_code)]
#[derive(Debug)]
struct DEPARTMENT {
    dept_id: i32,
    dept_title: String,
    dept_strength: i32,
}

#[derive(Debug)]
struct SALARYENTRY {
    emp_id: i32,
    sal_id: i32,
    sal_date: String,
    sal: f32,
    status: String,
}
#[warn(dead_code)]
#[derive(Debug)]
struct LEAVE {
    emp_id: i32,
    leave_id: i32,
    leave_from: NaiveDate,
    leave_to: NaiveDate,
    leave_type: String,
}

fn read_text_file(file_path: &str) -> Result<Vec<EMPLOYEE>, Box<dyn Error>> {
    let file = File::open(file_path).expect("Parsing Error");
    let mut reader = ReaderBuilder::new()
        .delimiter(b'|') 
        .has_headers(true) 
        .from_reader(file);

    let mut employees = Vec::new();
    for result in reader.deserialize() {
        // println!("{:?}",result);
        let employee: EMPLOYEE = result.expect("Parsing Error");
        employees.push(employee);
    }
    Ok(employees)
}

fn read_Dept_file(file_path: &str, sheet_name: &str) -> Result<Vec<DEPARTMENT>, Box<dyn Error>> {
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
        departments.push(DEPARTMENT {
            dept_id,
            dept_title,
            dept_strength,
        });
    }
    Ok(departments)
}

fn read_Sal_file(file_path: &str, sheet_name: &str) -> Result<Vec<SALARYENTRY>, Box<dyn Error>> {
    
    let mut workbook = open_workbook_auto(file_path).expect("Parsing Error");
    let range = workbook.worksheet_range(sheet_name).expect("Parsing Error");

    let mut salaries = Vec::new();
    for row in range.rows().skip(1) {
        //println!("{:?}",row);
        let salary = SALARYENTRY {
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

fn read_Leave_file(file_path: &str, sheet_name: &str) -> Result<Vec<LEAVE>, Box<dyn Error>> {

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

        let leave_entry = LEAVE {
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

fn max_days_in_month(year: i32, month: u32) -> u32 {
    let first_day_of_next_month = if month == 12 {
        NaiveDate::from_ymd(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd(year, month + 1, 1)
    };

    let last_day_of_month = first_day_of_next_month.pred(); 
    last_day_of_month.day() 
}

fn generate_output(
    emp_data: Vec<EMPLOYEE>,
    dept_data: Vec<DEPARTMENT>,
    sal_data: Vec<SALARYENTRY>,
    leave_data: Vec<LEAVE>,
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
                //println!("{} {} {} {}",current_month,start_month,end_month,max_days_in_month(Utc::now().year(), current_month));
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
                    //println!("{} {}",start_date,end_date);
                    // if start_date == end_date {
                    //     println!("Yes i am in here");
                    //     (end_date - start_date).num_days() + 1
                    // } else {
                    //     (end_date - start_date).num_days() + 1
                    // }
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

        // println!("{:?}",total_leave_days);

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

    let emp_path = matches.get_one::<String>("EMP_FILE").expect("Parsing Error");
    let dept_path = matches.get_one::<String>("DEPT_FILE").expect("Parsing Error");
    let salary_path = matches.get_one::<String>("SAL_FILE").expect("Parsing Error");
    let leave_path = matches.get_one::<String>("LEAVE_FILE").expect("Parsing Error");
    let output_path = matches.get_one::<String>("OUTPUT_FILE").expect("Parsing Error");

    // println!("{:?}",read_text_file(emp_path));
    // println!("{:?}",read_Dept_file(dept_path, "Sheet1"));
    // match read_text_file(emp_path) {
    //     Ok(emp_data) => println!("{:.expect("Parsing Error")}", emp_data),
    //     Err(e) => eprintln!("Error reading employee data: {}", e),
    // }

    // match read_Dept_file(dept_path, "Sheet1") {
    //     Ok(dept_data) => println!("{:.expect("Parsing Error")}", dept_data),
    //     Err(e) => eprintln!("Error reading department data: {}", e),
    // }

    // match read_Sal_file(salary_path, "Sheet1") {
    //     Ok(sal_data) => println!("{:.expect("Parsing Error")}", sal_data),
    //     Err(e) => eprintln!("Error reading salary data: {}", e),
    // }

    // match read_Leave_file(leave_path, "Sheet1") {
    //     Ok(leave_data) => println!("{:.expect("Parsing Error")}", leave_data),
    //     Err(e) => eprintln!("Error reading leave data: {}", e),
    // }
    let emp_data = read_text_file(emp_path).expect("Failed to read employee data");
    let dept_data = read_Dept_file(dept_path, "Sheet1").expect("Failed to read department data");
    let sal_data = read_Sal_file(salary_path, "Sheet1").expect("Failed to read salary data");
    let leave_data = read_Leave_file(leave_path, "Sheet1").expect("Failed to read leave data");

    generate_output(emp_data, dept_data, sal_data, leave_data, output_path)
        .expect("Failed to generate output");
}

// cargo run --release -- -e /home/vanshs/Documents/Assignment/assignmentF/inputfile/employee_data.txt -d /home/vanshs/Documents/Assignment/assignmentF/inputfile/Deptfile.xlsx -s /home/vanshs/Documents/Assignment/assignmentF/inputfile/Sheet2.xlsx -l /home/vanshs/Documents/Assignment/assignmentF/inputfile/Sheet3.xlsx -o /home/vanshs/Documents/Assignment/assignmentF/inputfile/output_data.txt