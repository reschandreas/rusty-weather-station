CREATE TABLE measurements (
  id SERIAL PRIMARY KEY,
  time DATETIME,
  temperature FLOAT,
  humidity FLOAT,
  pressure FLOAT,
  is_raining BOOLEAN
);
