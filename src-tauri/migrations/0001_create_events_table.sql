CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    start TEXT NOT NULL,
    end TEXT NOT NULL,
    location TEXT NOT NULL,
    person_in_charge TEXT NOT NULL,
    contact_num TEXT NOT NULL,
    is_approved INTEGER NOT NULL DEFAULT 0,
    color TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
)