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

use borsh::{BorshDeserialize, BorshSerialize};
use smart_contract::{
    Transaction,
    sdk_method_bindgen,
};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub enum Grade {
    A,
    B,
    C,
    D,
    E,
    F,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct ExamResult {
    pub grade: Grade,
    pub score: u32,
}

// An example of a data struct using the `sdk_method_bindgen` macro
// provided by ParallelChain Mainnet Smart contract SDK.
// The macro provides custom methods to interact with the blockchain.
//
// Note that both the serializer and deserializer macros such as Borsh
// need to be applied to this struct for it to work.
// See the ParallelChain Mainnet SDK documentation for more information.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
#[sdk_method_bindgen]
pub struct StudentExaminationRecord {
    pub first_name: String,
    pub last_name: String,
    mathematics: ExamResult,
    science: ExamResult,
    languages: ExamResult,
    general_studies: ExamResult,
}

impl StudentExaminationRecord {

    pub fn add_new_entry(
        tx: &Transaction<Self>,
        first_name: &String,
        last_name: &String,
        mathematics: &ExamResult,
        science: &ExamResult,
        languages: &ExamResult,
        general_studies: &ExamResult,
    ) {

        let student_exam_record = StudentExaminationRecord {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            mathematics: ExamResult { 
                grade: mathematics.grade.to_owned(), 
                score: mathematics.score,
            },
            science: ExamResult { 
                grade: science.grade.to_owned(), 
                score: science.score,
            },
            languages: ExamResult { 
                grade: languages.grade.to_owned(), 
                score: languages.score,
            },
            general_studies: ExamResult { 
                grade: general_studies.grade.to_owned(), 
                score: general_studies.score,
            },
        };

        tx.set_student_examination_record(
            format!(
                "{}{}", 
                &student_exam_record.first_name,
                &student_exam_record.last_name
            ).as_bytes(),
            &student_exam_record
        );

        tx.emit_event(
            format!("exam_record: add_new_entry").as_bytes(),
            format!(
                "Successfully added examination
                results for {}, {}", 
                &student_exam_record.last_name,
                &student_exam_record.first_name).as_bytes()
        );

    }

    //show exam results
    pub fn show_results(tx: &Transaction<Self>) {
       
        match tx.get_student_examination_record(format!(
            "{}{}", 
            &tx.arguments.first_name,
            &tx.arguments.last_name
        ).as_bytes()) {
            Some(se_record) => {
                // store the results as a hashmap to easily iterate without 
                // going through the first and last name
                let mut subjects = std::collections::HashMap::new();

                subjects.insert("mathematics", se_record.mathematics);
                subjects.insert("science", se_record.science);
                subjects.insert("languages", se_record.languages);
                subjects.insert("general_studies", se_record.general_studies);
        
                tx.emit_event(
                    format!("exam_record: show_results/name").as_bytes(),
                    format!("The results for {}, {} are: ",
                    &se_record.first_name,
                    &se_record.last_name).as_bytes()
                );

                for (subject, exam_result) in subjects {
                    tx.emit_event(
                        format!("exam_record: show_results/results").as_bytes(),
                        format!("Subject: {}, Grade: {:?}, Score: {}",
                            &subject,
                            &exam_result.grade,
                            &exam_result.score,
                        ).as_bytes()
                    );
                }
            },
            None => tx.emit_event(
                format!("exam_record: show_results/no_profile").as_bytes(),
                format!("No such profile found").as_bytes()
            ),
        };
         
    }

    // update the examination results of a student
    pub fn update_result(
        tx: &Transaction<Self>,
        subject_name: String,
        subject_result: &ExamResult
    ) {
        // the key is the first_name and last_name of the student concatenated
        let key = format!(
            "{}{}", 
            &tx.arguments.first_name,
            &tx.arguments.last_name
        ); 

        match tx.get_student_examination_record(key.as_bytes()) {
            Some(mut se_subject_record) => {
                match subject_name.as_str() {
                    "mathematics" => {
                        se_subject_record.write_mathematics(subject_result.to_owned());
                        tx.set_student_examination_record(key.as_bytes(), &se_subject_record);
                        tx.emit_event(
                            format!("exam_record: update_result").as_bytes(),
                            format!("Subject: Mathematics").as_bytes()
                        );
                    },
                    "science" => {
                        se_subject_record.write_science(subject_result.to_owned());
                        tx.set_student_examination_record(key.as_bytes(), &se_subject_record);
                        tx.emit_event(
                            format!("exam_record: update_result").as_bytes(),
                            format!("Subject: Science").as_bytes()
                        );
                    },
                    "languages" => {
                        se_subject_record.write_languages(subject_result.to_owned());
                        tx.set_student_examination_record(key.as_bytes(), &se_subject_record);
                        tx.emit_event(
                            format!("exam_record: update_result").as_bytes(),
                            format!("Subject: Languages").as_bytes()
                        );
                    },
                    "general_studies" => {
                        se_subject_record.write_general_studies(subject_result.to_owned());
                        tx.set_student_examination_record(key.as_bytes(), &se_subject_record);
                        tx.emit_event(
                            format!("exam_record: update_result").as_bytes(),
                            format!("Subject: General Studies").as_bytes()
                        );
                    },
                    _ => tx.emit_event(
                        format!("exam_record: update_result/no subject").as_bytes(),
                        format!("No subject found.").as_bytes()
                    ),
                }; 
                
            },
            None => tx.emit_event(
                format!("exam_record: update_result/no profile").as_bytes(),
                format!("No such profile found found.").as_bytes()
            ),
        };
    } 

    // setter method to update subjects
    pub fn write_mathematics(
        &mut self,
        subject_result: ExamResult
    ) {
        self.mathematics = subject_result;
    } 

    pub fn write_science(
        &mut self,
        subject_result: ExamResult
    ) {
        self.science = subject_result;
    } 

    pub fn write_languages(
        &mut self,
        subject_result: ExamResult
    ) {
        self.languages = subject_result;
    } 

    pub fn write_general_studies(
        &mut self,
        subject_result: ExamResult
    ) {
        self.general_studies = subject_result;
    } 

    // getter method to access subject results
     pub fn get_mathematics(&self) -> ExamResult {
        self.mathematics.to_owned()
    } 

    pub fn get_science(&self) -> ExamResult {
        self.science.to_owned()
    } 

    pub fn get_languages(&self) -> ExamResult {
        self.languages.to_owned()
    } 

    pub fn get_general_studies(&self) -> ExamResult {
        self.general_studies.to_owned()
    } 

}
