# ORM (`sqlalchemy`)

The ORM setup lies in `src/bakend/db/`.

1. `db_orm.py` sets up the connection to the database.

2. `models.py` defines the tables, constraints and relationships.

## `.env`

To get started you must copy .env.example in `src` to `.env` and fill in the correct values:

```plaintext
POSTGRES_SERVER=whoknows_database
POSTGRES_USER=postgres
POSTGRES_PASSWORD=password
POSTGRES_PORT=5432
POSTGRES_DB=whoknows
```

The interesting part is that due to the setup in the docker compose file, the application is on the same network as the database. Instead of an IP adress we can refer to it through the key defined in the `docker-compose.yml` file (`whoknows_database`). 

## The dependencies

Take a note of the new dependencies in `src/backend/requirements.txt`. 

`psycopg` is the driver that allows us to connect to `PostgreSQL` but it requires some non-Python OS-level dependencies to be installed. 

This is taken care of in the `src/backend/Dockerfile`:

```Dockerfile
# Install psycopg2 dependencies
RUN apt install libpq-dev -y
```

## `app.py`

In the `app.py` file the SQLite database check is disabled and a new section has been added:

```python
from db.models import User, Page
from db.db_orm import Database

database = Database()

with database.connect_db() as db:
    users = db.query(User).all()
    for user in users:
        print(user.username)
```

It should print out `admin` if you still have the following in your `schema.sql` file:

```sql
INSERT INTO users (username, email, password) 
    VALUES ('admin', 'keamonk1@stud.kea.dk', '5f4dcc3b5aa765d61d8327deb882cf99');
```

## Final words

You are now well on your way to use the ORM in your project. Let copilot guide you on how to do the rest.

Next step is [migrations](./Migration.md).