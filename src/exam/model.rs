use serde::{Deserialize, Serialize};

use crate::utils::generate_id;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Exam {
    pub id: String,
    pub name: String,
    pub start_date: i64,
    pub end_date: i64,
    pub class_id: String,
    pub questions: Vec<Question>,
}

impl Exam {
    pub fn new(
        name: &str,
        start_date: &i64,
        end_date: &i64,
        class_id: &str,
        questions: Vec<Question>,
    ) -> Self {
        Exam {
            id: generate_id(),
            name: name.to_string(),
            start_date: start_date.clone(),
            end_date: end_date.clone(),
            class_id: class_id.to_string(),
            questions,
        }
    }

    pub fn new_with_id(
        id: &str,
        name: &str,
        start_date: &i64,
        end_date: &i64,
        class_id: &str,
        questions: Vec<Question>,
    ) -> Self {
        Exam {
            id: id.to_string(),
            name: name.to_string(),
            start_date: start_date.clone(),
            end_date: end_date.clone(),
            class_id: class_id.to_string(),
            questions,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_start_date(&self) -> &i64 {
        &self.start_date
    }

    pub fn get_end_date(&self) -> &i64 {
        &self.end_date
    }

    pub fn get_class_id(&self) -> &String {
        &self.class_id
    }

    pub fn get_questions(&self) -> &Vec<Question> {
        &self.questions
    }

    pub fn set_questions(&mut self, questions: Vec<Question>) -> () {
        self.questions = questions;
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct Question {
    pub id: String,
    pub question: String,
    pub exam_id: String,
    pub answers: Vec<Answer>,
}

impl Question {
    pub fn new(question: &str, exam_id: &str, answers: Vec<Answer>) -> Self {
        Question {
            id: generate_id(),
            question: question.to_string(),
            exam_id: exam_id.to_string(),
            answers,
        }
    }

    pub fn new_with_id(id: &str, question: &str, exam_id: &str, answers: Vec<Answer>) -> Self {
        Question {
            id: id.to_string(),
            question: question.to_string(),
            exam_id: exam_id.to_string(),
            answers,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_question(&self) -> &String {
        &self.question
    }

    pub fn get_exam_id(&self) -> &String {
        &self.exam_id
    }

    pub fn get_answers(&self) -> &Vec<Answer> {
        &self.answers
    }

    pub fn set_answers(&mut self, answers: Vec<Answer>) -> () {
        self.answers = answers;
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct Answer {
    pub id: String,
    pub answer: String,
    pub is_correct: bool,
    pub question_id: String,
}

impl Answer {
    pub fn new(answer: &str, is_correct: &bool, question_id: &str) -> Self {
        Answer {
            id: generate_id(),
            answer: answer.to_string(),
            is_correct: is_correct.clone(),
            question_id: question_id.to_string(),
        }
    }

    pub fn new_with_id(id: &str, answer: &str, is_correct: &bool, question_id: &str) -> Self {
        Answer {
            id: id.to_string(),
            answer: answer.to_string(),
            is_correct: is_correct.clone(),
            question_id: question_id.to_string(),
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_answer(&self) -> &String {
        &self.answer
    }

    pub fn is_correct(&self) -> bool {
        self.is_correct
    }

    pub fn get_question_id(&self) -> &String {
        &self.question_id
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StudentAnswer {
    pub student_id: String,
    pub question_id: String,
    pub answer_id: String,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

impl StudentAnswer {
    pub fn new(
        student_id: &str,
        question_id: &str,
        answer_id: &str,
        created_at: Option<i64>,
        updated_at: Option<i64>,
    ) -> Self {
        StudentAnswer {
            student_id: student_id.to_string(),
            question_id: question_id.to_string(),
            answer_id: answer_id.to_string(),
            created_at,
            updated_at,
        }
    }

    pub fn new_with_id(
        student_id: &str,
        question_id: &str,
        answer_id: &str,
        created_at: Option<i64>,
        updated_at: Option<i64>,
    ) -> Self {
        StudentAnswer {
            student_id: student_id.to_string(),
            question_id: question_id.to_string(),
            answer_id: answer_id.to_string(),
            created_at,
            updated_at,
        }
    }

    pub fn get_student_id(&self) -> &String {
        &self.student_id
    }

    pub fn get_question_id(&self) -> &String {
        &self.question_id
    }

    pub fn get_answer_id(&self) -> &String {
        &self.answer_id
    }

    pub fn get_created_at(&self) -> Option<i64> {
        self.created_at
    }

    pub fn set_created_at(&mut self, created_at: Option<i64>) {
        self.created_at = created_at;
    }

    pub fn get_updated_at(&self) -> Option<i64> {
        self.updated_at
    }

    pub fn set_updated_at(&mut self, updated_at: Option<i64>) {
        self.updated_at = updated_at;
    }
}
