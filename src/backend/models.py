import sqlite3
import os
import bcrypt

DATABASE_PATH = '/tmp/whoknows.db'

def connect_db(init_mode=False):
    """Returns a new connection to the database."""
    if not os.path.exists(DATABASE_PATH):
        if not init_mode:
            raise FileNotFoundError("Database not found. Run init_db to create the database.")
    return sqlite3.connect(DATABASE_PATH)

def init_db():
    """Creates the database tables."""
    conn = connect_db(init_mode=True)  # Passer init_mode=True, så den tillader at initialisere databasen
    try:
        with open('schema.sql', mode='r') as f:
            conn.cursor().executescript(f.read())
        conn.commit()
        print(f"Database initialized at {DATABASE_PATH}")
    except Exception as e:
        print(f"Error initializing the database: {e}")
    finally:
        conn.close()


def query_db(query, args=(), one=False):
    """Queries the database and returns a list of dictionaries."""
    conn = connect_db()
    cur = conn.execute(query, args)
    rv = [dict((cur.description[idx][0], value) for idx, value in enumerate(row)) for row in cur.fetchall()]
    conn.close()
    return (rv[0] if rv else None) if one else rv

def hash_password(password):
    """Hashes the password using bcrypt."""
    return bcrypt.hashpw(password.encode('utf-8'), bcrypt.gensalt()).decode('utf-8')

def verify_password(stored_hash, password):
    """Verifies a hashed password using bcrypt."""
    return bcrypt.checkpw(password.encode('utf-8'), stored_hash.encode('utf-8'))
