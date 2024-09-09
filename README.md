# Team Avalibility Coordinator Backend

The REST API for storing data about avalibilty of the players and running proccesesing

Built using Axum and PostgresDB

# Building

To build this project using cargo, run:

```
cargo build
```

# Configuring the DB

First set the environment variables:

```
POSTGRES_PASSWORD=yourpassword
POSTGRES_USER=youruser
POSTGRES_DB=yourdatabase
```

And then configure the db using `setup.sql`


# Running

To run this project use

```
cargo run
```

# Docker

Configure the db in the docker container from compose

Then the project can be run using docker compose:

```
docker compose up
```