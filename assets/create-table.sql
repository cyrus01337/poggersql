CREATE TABLE pogsql (
    id SERIAL PRIMARY KEY,
    scripts INT,
    successful_scripts INT,
    failed_scripts INT,
    version TEXT
)
