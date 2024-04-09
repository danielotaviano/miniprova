-- Add migration script here


CREATE TABLE student_answer (
    id SERIAL PRIMARY KEY,
    student_id VARCHAR NOT NULL,
    question_id VARCHAR NOT NULL,
    answer_id VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (student_id, question_id)
);