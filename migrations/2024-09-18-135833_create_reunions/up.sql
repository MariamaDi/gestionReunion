CREATE EXTENSION IF NOT EXISTS "uuid-ossp";



CREATE TABLE reunions (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    titre VARCHAR(255) NOT NULL,
    date_reunion DATE NOT NULL,
    heure_debut TIME NOT NULL,
    heure_fin TIME NOT NULL,
    participants UUID[] NOT NULL,
    notes TEXT
);


