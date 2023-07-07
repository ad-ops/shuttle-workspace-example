CREATE TABLE IF NOT EXISTS persons
(
    id          BIGSERIAL PRIMARY KEY,
    name        TEXT    NOT NULL,
    age         SMALLINT NOT NULL
);