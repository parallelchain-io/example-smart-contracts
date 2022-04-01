/*
 Copyright (c) 2022 ParallelChain Lab

 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.

 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.

 You should have received a copy of the GNU General Public License
 along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

// Student examination records smart contract.
// Teachers can use this contract to record a students examination
// results while students can view their examination results.

use smart_contract::{
    Transaction,
    contract_init,
};

use anyhow::{Result, anyhow};
use borsh::BorshSerialize;

pub mod exam_records;
use exam_records::{
    ExamResult,
    Grade,
    StudentExaminationRecord
};

// The `contract_init` macro is required to convert the smart contract code
// from idiomatic rust to a contract that is readable and executable in
// ParallelChain Mainnet Fullnode.
#[contract_init]
pub fn contract(tx: Transaction<StudentExaminationRecord>) -> Result<ExamResult> {
    
    tx.emit_event(
        format!("exam_record: start").as_bytes(),
        format!(
            "The smart contract has received an argument of {:?}", 
            tx.arguments).as_bytes()
    );

    let student = &tx.arguments;
    let mathematics = &student.get_mathematics();
    let science = &student.get_science();
    let languages = &student.get_languages();
    let general_studies = &student.get_general_studies();
    
    // the homeroom teacher adds the student's record
    StudentExaminationRecord::add_new_entry(
        &tx, 
        &student.first_name,
        &student.last_name,
        mathematics,
        science,
        languages,
        general_studies,
    );

    // show blank exam record
    StudentExaminationRecord::show_results(&tx);

    // The mathematics, science, languages and general_studies teacher gather
    // together in a cafe and update the student's examination record.
    let mut teachers_in_cafe_holding_student_results = std::collections::HashMap::new();

    teachers_in_cafe_holding_student_results.insert("mathematics", mathematics);
    teachers_in_cafe_holding_student_results.insert("science", science);
    teachers_in_cafe_holding_student_results.insert("languages", languages);
    teachers_in_cafe_holding_student_results.insert("general_studies", general_studies);

    let mut total_score = 0;
    for (subject, subject_result) in teachers_in_cafe_holding_student_results {
        StudentExaminationRecord::update_result(
            &tx, 
            subject.to_string(), 
            subject_result
        );
        total_score += subject_result.score;
    }

    let average_score = total_score / 4;

    if average_score == 0 {
        
        tx.emit_event(
            format!("exam_records: average_score").as_bytes(),
            format!("The average score cannot be a 0!").as_bytes()
        );

        Err(anyhow!("The average score cannot be a 0!"))
    }
    else {
        let average_exam_result = ExamResult { grade: Grade::B, score: average_score };
        Ok(average_exam_result)
    }
}


