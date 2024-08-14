# Whoknows Variations

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

[The Database](./tutorial/The_Database.md)


## ORM (`sqlalchemy`)

[ORM](./tutorial/ORM.md)



## Migration

[Migration](./tutorial/Migration.md)

## [Optional] pgAdmin

You could consider adding [pgAdmin](https://www.pgadmin.org/) to your project. 

pgAdmin provides a visual dashboard to monitor and administer your database. Docker offers an example of how to do it:

https://github.com/docker/awesome-compose/tree/master/postgresql-pgadmin


## Future work

1. You should have improved the schema significantly from the legacy SQLite schema. Adapt the models to the new schema.

2. Create migrations for the data in your SQLite database so that it will be read into the new PostgreSQL database.
