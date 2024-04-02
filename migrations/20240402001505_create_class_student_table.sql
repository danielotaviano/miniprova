-- Add migration script here
CREATE TABLE class_student (
    id VARCHAR PRIMARY KEY,
    class_id VARCHAR,
    student_id VARCHAR,
    FOREIGN KEY (class_id) REFERENCES class(id),
    FOREIGN KEY (student_id) REFERENCES "user"(id)
);