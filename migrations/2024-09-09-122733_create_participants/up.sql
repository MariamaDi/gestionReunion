-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE Participants (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    nom VARCHAR(255) NOT NULL, 
    email VARCHAR(255) NOT NULL
);
