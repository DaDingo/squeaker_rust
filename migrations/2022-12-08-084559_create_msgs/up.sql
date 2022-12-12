-- Your SQL goes here
CREATE TABLE msgs (
  id SERIAL PRIMARY KEY,
  content TEXT NOT NULL,
  date_time timestamp NOT NULL,
  likes BIGINT NOT NULL
)