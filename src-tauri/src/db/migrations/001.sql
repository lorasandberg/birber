CREATE TABLE
    IF NOT EXISTS raws (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        cam_id TEXT UNIQUE,
        raw_path TEXT,
        jpg_path TEXT,
        preview TEXT,
        thumbnail TEXT,
        date_taken DATETIME,
        author TEXT
    );

CREATE TABLE
    IF NOT EXISTS photos (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        raw INTEGER NOT NULL,
        preview TEXT,
        thumbnail TEXT,
        photo_ops TEXT,
        rating INTEGER
    );

CREATE TABLE
    IF NOT EXISTS species (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        scientific_name TEXT,
        english_name TEXT,
        french_name TEXT
    );

CREATE TABLE
    IF NOT EXISTS photos_to_species (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        photos_id INTEGER,
        species_id INTEGER
    );

CREATE TABLE
    IF NOT EXISTS tags (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        tag TEXT UNIQUE
    );

CREATE TABLE
    IF NOT EXISTS photos_to_tags (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        photos_id INTEGER,
        tags_id INTEGER
    );

CREATE TABLE
    IF NOT EXISTS meta (id TEXT PRIMARY KEY, value TEXT);