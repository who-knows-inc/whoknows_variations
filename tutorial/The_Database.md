# The database

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
$ docker-compose up --build
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