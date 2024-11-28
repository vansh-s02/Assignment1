#!/usr/bin/env bash

EMP_FILE=$"/home/vanshs/Documents/Assignment/assignment1/inputfile/employee_data.txt"
DEPT_FILE=$"/home/vanshs/Documents/Assignment/assignment1/inputfile/Deptfile.xlsx"
SAL_FILE=$"/home/vanshs/Documents/Assignment/assignment1/inputfile/Sheet2.xlsx"
LEV_FILE=$"/home/vanshs/Documents/Assignment/assignment1/inputfile/Sheet3.xlsx"
OUT_FILE=$"/home/vanshs/Documents/Assignment/assignment1/inputfile/output_data.txt"

cargo run -- \
-e ${EMP_FILE} \
-d ${DEPT_FILE} \
-s ${SAL_FILE} \
-l ${LEV_FILE} \
-o ${OUT_FILE}