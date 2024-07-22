# Migration (`alembic`)

While SQLAlchemy is a Python SQL tookit and ORM, Alembic is a tool for creating migrations. 

## Pre-requisites

1. Make sure that the PostgreSQL database is running. 

2. Install the `alembic` package. 

Success criteria: You can run the `alembic` command:

```bash
$ alembic --version
```

The optimal way is to avoid installing it globally by having a virtual environment.  

Here is how you can do it using [Poetry](https://python-poetry.org/):

```bash
$ poetry init -n
$ poetry add alembic psycopg2 sqlalchemy
$ poetry run alembic init alembic
$ poetry shell
```

You might simply be able to install it with pip:

```bash
$ pip install alembic
```

But this will not work on Mac with `pip` installed via Homebrew. I recommend using `poetry` despite the initial installation hurdle. 

## Configuration

Create the `alembic` setup in the `src/backend` folder:

```bash
$ alembic init alembic
```

This will create a folder called alembic where you run it.

Update the `alembic.ini` with the database connection information. Assuming that you have exposed the PostgreSQL container to localhost on the default port 5432:

```ini
# sqlalchemy.url = postgresql://<username>:<password>@localhost:5432/<db_name>
sqlalchemy.url = postgresql://postgres:password@localhost:5432/whoknows
```

## Creating a migration

This is inspired from the official documentation. First, generate a migration file:

```bash
$ alembic revision -m "create account table"
```

The file will be created in the `src/backend/alembic/versions` folder.

Update the file with the migration code from the documentation. But let's pluralize the table name to `accounts`:

https://alembic.sqlalchemy.org/en/latest/tutorial.html#create-a-migration-script

```python
def upgrade():
    op.create_table(
        'accounts',
        sa.Column('id', sa.Integer, primary_key=True),
        sa.Column('name', sa.String(50), nullable=False),
        sa.Column('description', sa.Unicode(200)),
    )

def downgrade():
    op.drop_table('accounts')
```

## Running the migration

```bash
$ alembic upgrade head
```

Check on the database. You should see the `accounts` table. 

It also created a table called `alembic_version` to keep track of the migrations. 

Rollback the migration:

```bash
$ alembic downgrade -1
```

Check on the database again. The `accounts` table should be gone.

## Seeding

Inspired by this comment https://stackoverflow.com/a/19338319 but with some adjustments. Define the upgrade function as the following:

```python
def upgrade():
    accounts_table = op.create_table(
        'accounts',
        sa.Column('id', sa.Integer, primary_key=True),
        sa.Column('name', sa.String(50), nullable=False),
        sa.Column('description', sa.Unicode(200)),
    )
    op.bulk_insert(accounts_table,
    [
        {'id': 1, 'name': 'John Smith', 'description': 'CEO'},
        {'id': 2, 'name': 'Ed Williams', 'description': 'CTO'},
        {'id': 3, 'name': 'Wendy Jones', 'description': 'CFO'},
    ]
)
```

## Final words

1. **With the idea of migration** you should be inspired to create version control for your database. Any future changes can be tracked and rolled back if necessary. It will ease the process of deploying changes to the database but also to ensure that all developers can easily apply the latest changes to the database when they pull the code.

2. **With the concept of seeding** you should be inspired to take the data from your SQLite database and populate it into the PostgreSQL database.

There are multiple ORMs and migration tools out there. Research the ones in the programming langauge that you are working with. One recommendation is to create a separate benchmark repository to test out different tools and see which one fits your project the best.