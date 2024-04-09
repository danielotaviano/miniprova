use std::error::Error;

use crate::exam::{self, model::Exam};

pub async fn get_exam_result(
    exam_id: &str,
    student_id: &str,
) -> Result<Option<(Exam, Vec<String>)>, Box<dyn Error>> {
    let exam = match exam::service::get_student_exam(exam_id, student_id).await {
        Err(_) => {
            return Err("Error getting the exam".into());
        }
        Ok(exam) => exam,
    };

    if exam.is_some() && exam.as_ref().unwrap().0.end_date > chrono::Utc::now().timestamp_millis() {
        return Err("Exam is not available yet".into());
    }

    Ok(exam)
}
