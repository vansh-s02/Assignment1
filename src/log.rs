
use calamine::{open_workbook_auto, DataType, Reader};
use chrono::NaiveDate;
use csv::ReaderBuilder;
use std::fs::File;
use std::error::Error;

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub struct Emp {   //Struct for the employee data
    pub emp_id: i32,
    pub emp_name: String,
    pub dept_id: i32,
    pub mobile_no: String,
    pub email: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Dept {     //Struct for the Department data
    pub dept_id: i32,
    pub dept_title: String,
    pub dept_strength: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Salent {    //Struct for the Salary data
    pub emp_id: i32,
    pub sal_id: i32,
    pub sal_date: String,
    pub sal: f32,
    pub status: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Leav {          //Struct for the Leave data
    pub emp_id: i32,
    pub leave_id: i32,
    pub leave_from: NaiveDate,
    pub leave_to: NaiveDate,
    pub leave_type: String,
}

// fuction to read the emp text file and pass it data in vector<struct> form
pub fn read_text_file(file_path: &str) -> Result<Vec<Emp>, Box<dyn Error>> {
    let file = File::open(file_path).expect("Error in opening emp file");//open the input file
    let mut reader = ReaderBuilder::new()//reading the input file
        .delimiter(b'|') 
        .has_headers(true) 
        .from_reader(file);

    let mut employees = Vec::new();

    //reading the lines of file and store them in vector one by one
    for result in reader.deserialize() {
        let employee: Emp = result.expect("Erppr om reading emp line");
        employees.push(employee);
    }
    Ok(employees)
}

//funtion to read the dept file and give the data of it in vector<struct> form
pub fn read_dept_file(file_path: &str, sheet_name: &str) -> Result<Vec<Dept>, Box<dyn Error>> {
    let mut workbook = open_workbook_auto(file_path).expect("Error in opening Dept file");
    let range = workbook.worksheet_range(sheet_name).expect("Error in reading lines");

    let mut departments = Vec::new();
    for row in range.rows().skip(1) {
        let dept_id = row[0].get_float().ok_or_else(|| "Invalid Dept ID").expect("Parsing Error in getting dept id") as i32;
        let dept_title = row[1]
            .get_string()
            .ok_or_else(|| "Invalid Dept Title").expect("Parsing Error in data title")
            .to_string();
        let dept_strength = row[2].get_float().ok_or_else(|| "Invalid Dept Strength").expect("Parsing Error in getting strenght") as i32;
        departments.push(Dept {
            dept_id,
            dept_title,
            dept_strength,
        });
    }
    Ok(departments)
}

//funtion to read the sal file and give the data of it in vector<struct> form
pub fn read_sal_file(file_path: &str, sheet_name: &str) -> Result<Vec<Salent>, Box<dyn Error>> {
    
    let mut workbook = open_workbook_auto(file_path).expect("Parsing Error in opening sal file");
    let range = workbook.worksheet_range(sheet_name).expect("Parsing Error in reading line of sal");

    let mut salaries = Vec::new();
    for row in range.rows().skip(1) {
        //println!("{:?}",row);
        let salary = Salent {
            emp_id: row[0].get_float().ok_or("Invalid Emp ID").expect("Parsing Error in getting emp id  data") as i32,
            sal_id: row[1].get_float().ok_or("Invalid Salary Id").expect("Parsing Error in getting sal id data") as i32,
            sal_date: (row[2].get_string().ok_or("Invalid Salaray Date").expect("Parsing Error in getting sal date data") as &str).to_string(),
            sal: row[3].get_float().ok_or("Invalid Salary").expect("Parsing Error in getting sal data") as f32,
            status: (row[4].get_string().ok_or("Invalid Dept Strength").expect("Parsing Error in getting salary status data") as &str).to_owned(),
        };
        salaries.push(salary);
    }
    Ok(salaries)
}

//funtion to read the leave file and give the data of it in vector<struct> form
#[allow(deprecated)]
pub fn read_leave_file(file_path: &str, sheet_name: &str) -> Result<Vec<Leav>, Box<dyn Error>> {

    let mut workbook = open_workbook_auto(file_path).expect("Parsing Error in opening leave file");
    let range = workbook.worksheet_range(sheet_name).expect("Parsing Error in reading leave file data");

    let mut leaves = Vec::new();

    for row in range.rows().skip(1) {
        //println!("{:?}",row);
        let emp_id = row[0].get_float().ok_or("Invalid EMP ID").expect("Parsing Error in emp id ") as i32;
        //println!("{:?}",emp_id);
        let leave_id = row[1].get_float().ok_or("Invalid Leave ID").expect("Parsing Error in leave id") as i32;

        let leave_from_date_str = row[2].get_string().unwrap_or_default();
        let leave_to_date_str = row[3].get_string().unwrap_or_default();

        let leave_from = NaiveDate::parse_from_str(leave_from_date_str, "%e-%m-%Y")
            .unwrap_or_else(|_| NaiveDate::from_ymd(1970, 1, 1)); // Use default date if parsing fails
        let leave_to = NaiveDate::parse_from_str(leave_to_date_str, "%e-%m-%Y")
            .unwrap_or_else(|_| NaiveDate::from_ymd(1970, 1, 1));

        let leave_type = row[4].get_string().ok_or("Invalid Leave Type").expect("Errorin leave type ").to_string();

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

