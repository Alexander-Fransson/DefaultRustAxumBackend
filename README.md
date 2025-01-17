# DefaultRustAxumBackend
A default axum API that I can later fork from when I want to do something more special. Features shall be limited to getting usernames and create user when not logged in and CRUD user when logged in.


## Dev Diary

### 1. Create dev database

// after step is done read through and make instructions simpler

To start with the best first step is probably to create a dev database that can contain users for the get requests.
Some modules I am sure I will need are axum, sqlx and tokio with features full so Ill add those.
It is probably also good to add a .gitignore that ignores the target folder.
I start with creating a folder for the sql files
I also create a folder for example scripts becouse I think I will need it although I dont know if it will be nececary just jet.
First of all I need a database though, Ill get one through docker
Aparently you can create a postgres database in docker with just a semi simple command

```bash
docker run --name postgres-container-name -e POSTGRES_PASSWORD=super_secret_password -p 5430:5432 postgres
```
--name sets the name of the container, -e sets enviromental variables -p sets the port as PORT_ON_YOUR_COMPUTER:PORT_IN_CONTAINER (5432 is postgres default container) and postgres is the default postgres image on docker hub.
If you want it to run in the background you can add -d before postgres but I dont recomend it as you will probably forget about it.

However this command is quite verbose so it is probably better to automate it