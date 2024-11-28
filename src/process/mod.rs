use chrono::{Datelike, NaiveDate, Utc};
use std::fs::File;
use std::io::Write;
use std::{error::Error, io::BufWriter};
use crate::log;
use log::Emp;
use log::Dept;
use log::Salent;
use log::Leav;


#[allow(unused_variables)]
#[allow(deprecated)]
pub fn max_days_in_month(year: i32, month: u32) -> u32 {
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
pub fn generate_output(
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

    let file = File::create(output_path).expect("Parsing Error in creating output text file");
    let mut write_handler = BufWriter::new(file);

    writeln!(
        write_handler,
        "Emp ID~#~Emp Name~#~Dept Title~#~Mobile No~#~Email~#~Salary Status~#~On Leave"
    ).expect("Error the header of output file");

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
        ).expect("Error in write the data in output file");
    }

    println!("Output generated successfully: Go Check the output File");
    Ok(())
}

