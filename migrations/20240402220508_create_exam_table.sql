-- Add migration script here

CREATE TABLE exam (
    id VARCHAR PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "start_date" TIMESTAMP NOT NULL,
    end_date TIMESTAMP NOT NULL,
    class_id VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (class_id) REFERENCES class(id)
);

CREATE TABLE question (
    id VARCHAR PRIMARY KEY,
    "question" VARCHAR NOT NULL,
    exam_id VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (exam_id) REFERENCES exam(id)
);

CREATE TABLE answer (
    id VARCHAR PRIMARY KEY,
    "answer" VARCHAR NOT NULL,
    is_correct BOOLEAN NOT NULL,
    question_id VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (question_id) REFERENCES question(id)
);
