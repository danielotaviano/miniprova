use crate::utils::generate_id;

pub struct Exam {
    id: String,
    name: String,
    start_date: i64,
    end_date: i64,
    class_id: String,
    questions: Vec<Question>,
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

pub struct Question {
    id: String,
    question: String,
    exam_id: String,
    answers: Vec<Answer>,
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

pub struct Answer {
    id: String,
    answer: String,
    is_correct: bool,
    question_id: String,
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
