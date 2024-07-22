# Flask Variations

## Introduction

This gives an example of how to setup PostgreSQL with Flask. 

The example uses `sqlalchemy` as an ORM and `alembic` as a migration tool. `psycopg2` is the PostgreSQL adapter.

PostgreSQL is currently the most popular and used database. [Source](https://survey.stackoverflow.co/2023/#section-most-popular-technologies-databases). You are free to use any database you want, of course.


## Structure

One of the goals is to create a better structure. This is the tree of `src` folder:

```bash
src
├── Makefile
├── README.md
├── backend
│   ├── app.py
│   ├── app.pyc
│   ├── app_tests.py
│   ├── db
│   ├── migration
│   ├── requirements.txt
│   ├── static
│   │   └── style.css
│   └── templates
│       ├── about.html
│       ├── layout.html
│       ├── login.html
│       ├── register.html
│       └── search.html
├── database
├── run_forever.sh
└── schema.sql
```

Note that `schema.sql` was moved into the database folder. A seperation has been created between what relates to the database itself and the database code that the application uses.

1. The database itself. 

**a.** The `database` folder on the same level as the `backend` contains the Dockerfile that defines our PostgreSQL database. 


2. Database code relating to the application.

**a.** Inside of the `backend` folder, the `db` folder sets up the database connection (*ORM*) and the models. 

**b.** Inside of the `backend` folder, the `migration` folder contains the `alembic` configuration and the migration scripts.

## `schema.sql`

The `schema.sql` file has been adapted from SQLite to PostSQL syntax. The changes relate to how primary key and enums are defined. 

Original (SQLite)                            | Updated (PostgreSQL)
---------------------------------------------|-----------------------------------------
  id INTEGER PRIMARY KEY AUTOINCREMENT,      |   id **SERIAL PRIMARY KEY**,
    ...                                      | **DROP TABLE IF EXISTS pages**;
  title TEXT PRIMARY KEY UNIQUE,             |   title **TEXT PRIMARY KEY**,
  language TEXT NOT NULL                     |   language TEXT NOT NULL DEFAULT 'en' CHECK
  CHECK(language IN ('en', 'da'))            |   (language IN ('en', 'da')),
  DEFAULT 'en',                              |


## The database itself

The Dockerfile is simple. It gets the latest version of the database and copies the `schema.sql` file into the `/docker-entrypoint-initdb.d/` folder. This folder is used by the PostgreSQL image to initialize the database.

```Dockerfile
FROM postgres:latest
COPY schema.sql /docker-entrypoint-initdb.d/
```

In the `src` folder a `docker-compose.yml` file is created to show how to setup the database with user credentials. You should absolutely consider how you can define these credentials in a way that ensure it won't be pushed into version control. 

### Volumes

Furthermore, the `docker-compose.yml` file shows that a volume has been created called `database`. This ensures that even if the container is stopped and restarted, the data will persist. 

You can deploy your image to a registry and then deploy it to a server.

You can run the database with the following command in the `src` folder (run without `-d` (detached mode) to see the logs the first time):

```bash
$ docker-compose up
```

**Note**: The `restart: always` attribute means that the service tries to restart the container if it crashes.

## Accessing the database

While the database is running, you can access the database with the following command:

```bash
$ docker exec -it whoknows_db psql -U postgres -d whoknows
```

You should see a prompt that looks like this (version numbers may vary):

```plaintext
psql (16.3 (Debian 16.3-1.pgdg120+1))
Type "help" for help.

whoknows=# 
```

To verify the schema creation and see the tables:

```sql
whoknows=# \dt
```

You can always find PostgreSQL cheat sheets online.


## Debugging help

If you issues with the database and want to start over, you should remember to remove the volume too:

```bash
$ docker-compose down -v
```

The above command removes both the container and the volume in one go. 

For convenience, here is a single command to connect and see the created tables:

```bash
$ docker exec -it whoknows_database psql -U postgres -d whoknows -c '\dt'
```

## ORM (`sqlalchemy`)

The ORM setup lies in `/src/bakend/db/`. To get started you must copy .env.example in `src` to `.env` and fill in the correct values:

```plaintext
POSTGRES_SERVER=whoknows_database
POSTGRES_USER=postgres
POSTGRES_PASSWORD=password
POSTGRES_PORT=5432
POSTGRES_DB=whoknows
```

The interesting part is that due to the setup in the docker compose file, the application is on the same network as the database. Instead of an IP adress we can refer to it through the key defined in the `docker-compose.yml` file (`whoknows_database`). 



## Migration