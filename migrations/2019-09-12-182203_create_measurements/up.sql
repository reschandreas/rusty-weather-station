CREATE TABLE measurements (
  id SERIAL PRIMARY KEY,
  time timestamp default now() NOT NULL,
  temperature NUMERIC NOT NULL,
  humidity NUMERIC NOT NULL,
  pressure NUMERIC NOT NULL,
  is_raining BOOLEAN NOT NULL
);
