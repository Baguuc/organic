CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  ordr INT NOT NULL,
  done BOOL NOT NULL
);

