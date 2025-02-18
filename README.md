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

However this command is quite verbose so it is probably better to automate it.
Also it is probably woth mentioning that you need docker-cli installed on your your computer
Just look up how to install docker cli and remember that you dont look for docker desktop
That is the gui bloat that docker whant you to get used to so you will pay for it later

Creating a database continer can be done easyer using a docker compose file. 
However before we begin with the docker compose file it migh be good to mention
the difference between a docker compose file and a docker file since you unsually mix the two up
So A Dockerfile is used to help docker create an image f.ex from a file on your computer or something on docker hub
on the other hand a docker compose file describes how to run an image and its containers

For a demostration: https://www.youtube.com/watch?v=JmyAMcKUNYA

I created a docker compose file in: backend/db/docker-compose.yml
Check it out if you wonder how to set it up and what to do'
JSYK the version: parameter is deprecated if you see it in places

### 2. Setting up tracing 

So apparently interpreting log messages in traditional asyncronus rust code can be difficult.
The "tracing" crate helps rust programs to "collect scoped, structured, and async-aware diagnostics" and the "tracing_subscriber" crate "crate contains tools for composing subscribers out of smaller units of behaviour".
Quotes are taken from the official docks.rs of the crates.
This might not be nececary but it seams harmless and useful and is used by Jermy Chone in the rust axum production video accessed 2025.
To use it I added the tracing and tracing-subscriber crates as well as the feature env-filter to the tracing subscriber.
The tracing subscriber pipeline in main is what makes the info! debug! and other tracing mactos work.
The with env filter filters the loggs to f.ex only show info or higher using the RUST_LOG environmental variable which may be set in the [env] section in backend/.cargo/config.toml
you can set it to f.ex "info" or "debug" to only show logs with sush levels or higher.
You can also limit the source of the logs by specifying the crate name in the variable name like f.ex in this case "backend=info".
You could set the environmental variables in a .env file but aparently .env is not cool anymore and according to J. Chone setting the variable outside the application before it is run in this manner is what you want in a production application since it is more similar to how it is done in Kubernetes. I put the tracing logic in a function that is run in main and moved it to backend/src/log/tracer_config.rs
I tried to make the time string shorter but it proved more difficult than it was worh ro not just remove it

### 3. Configuring environment variables

As the next step I made the functions that retrive the environment variables in backend/src/config_env.rs although I maybe did not need it just yet.

### 4. Seeding DB

I think Ill only create a user table to keep it limited to the basics in this repo
We also need to add the features "postgres", "uuid", "time" and "runtime-tokio-rustls" to sqlx
Most of this is self explanitory, time adds time features, uuid allows for uuids, postgres allows sqlx to interact with postgres specifically and "runtime-tokio-rustls allows sqlx to adapt to the async timeing of tokio
I made the sql files to create a basic database and a user with an index to the username and a unique constraint for the email and password together in backend/db/sql/reset_db.
I also created a file for housing the functions to seed the db in a utils folder under the _dev. I think the underscore signals that it is not a part of the main application.
I also added the postgres connection to the env and redeclared the reset db function under the dev utils.
I relized that there is a whole thing about migrations in sqlx and I think that is how the DB is suppoesed to be initialized.
For migrations to take into account changes in the sql files you apparently need a build script. You can generate this using sqlx cli by using sqlx cli. 
```bash
cargo install sqlx-cli
sqlx migrate build-script
```
I belive you could also just add the build.rs that prints the required line manually
After some development I realized that you cannot run the sql that recreates the DB using the migration macro becouse it eliminates the session and the use you use in your connection string
The solustion to this is to create a default user and an app user and have the default user execute the sql in the files through query while the actual settingup and seeding can be done through macro and the functions to do this are in backend/src/db_setup/mod.rs.
To do this I split the files for the differnet functions in separate files in the db/sql folder named descriptively and created different functions for this in the db_setup model
I also think I should create some error handeling for the db setup functions as there are some errors like the file reading which are not covered by sql.
To have sql errors in my error module I need serde_with and serde 
I created the errors in backend/src/db_setup/error.rs
I think you can make the froms easyer with the thiserror crate but I kindof like doing the forms manually so wont use thiserror
Froms are the stuff that makes the ? operator work and you have to have them for all the errors in the peckingorder

### 5. Create the server and router
First I just added a router to the main, somehting of this sort: 
```rs
let main_router = Router::new().route("/hello_word", get(|| async {"Hello, World!"})); 
```
I also added the listener and axum server as well as the std::io::Error and from required for the question mark operators to work

### 6. Create the Create Get & Delete routes for user.
maybe a rest api pattern...
Jermy chone seams to have a clever architecture primarially split in a "web" layer and a "model" layer
The "model" layer handles normalization, database communitation and security aka data access
The "web" layer handles authentication and authorization.
Theese names seam somewhat standard however I find them to be too abstract, therefore I will call the model folder data_access
Likewise I will call the web layer gate as it contains the routes and middlewares
I moved the db_setup folder to the data access layer and created a dataaccess manager with an implemented new function that makes the migrations, resets the db and returns a connection pool that can only be used in the data access module. This is done in in backend/src/data_access/mod.rs.
I also created a function that creates data access managers to use when testing the future controllers
I wanted to create a strong reusable foundation for the crud functions so I created a base controller trait in backend/src/data_access/base_crud.rs
The base controller contains a const for the table name of the data base table that is to be interacted with. If the db name coresponded to struct names it could have been accesssed with a proc macro but this aproach gives more naming freedome.
Than I started to create generic functions for the get list create and delete functions. Theese take generic parameters arguments, the base controller and the response struct. The response struct should implement From row so that sqlx can map the intended response as well as Unpin and Send for the get requests that return a struct.

To get the struct fields of the generic structs I created a proc-macro lib in the utils folder. This is done through cargo new name_of_create --lib. Than I configured the Cargo.toml file in backend/src/utils/proc-macros/Cargo.toml. Under lib I set proc macros to true and I added quote and syn as dev dependencies. syn turns binary streams into token trees and quote does the oposite. 
Than in backend/src/utils/proc-macros/src/lib.rs I created a derive type proc macro for the GetStructFields trait that does percisely that. Proc macros are basically macros that turn stuff into rust at compile time. The derive type lets you create custom Derives that implements stuff into structs with one word. They do however have to be linked to a trait in the main crate so I added the get struct field to a traits for proc macros folder under utils backend/src/utils/traits_for_proc_macros.rs 
You also have to specify the proc macro lib as a dependency by providing the create name and path in backend/Cargo.toml. This allows us to query the DB for struct specific fields simply by deriving the Get struct fields trait as shown in the get and list functions in base_crud.

The get struct field is used in the get methods to create a string of what to select from the table. This then uses an sqlx query to interact with the database as detailed in backend/src/data_access/base_crud.rs. To get the struct fields and values for the detault create function I created a utils function that turns structs with serde serialize into hashmaps. Hashmaps have implemented functions to get the keys and values as iters. The function also accounts for None values and retunrs a NULL string in those ocurrences. This function is used in the default functions that have the struct as an imput to turn that struct into a string that can be used with sqlx. 

The utils module has an error module which is implemented by the data access module.

The base functions are used in the user controller at backend/src/data_access/user_controller/mod.rs which now just have to pass the data access manager and the request information implementing the nececary traits. As a controller it aslo has a table name which it passes as a generic parameter to the functions. The other generic parameter is fufilled by the controller function parameter which specializes the generic function to make the required standard querry. If we would want to create a less standard querry we would do so as sqlx in a specific controller such as this user controller.

Afterwords we move on to the gate layer. Here I created modules for routes and middlewares and since we dont need any middlewares at this stage I just create a user routes module at backend/src/gate/routes/user_routes.rs. Here the main public function is the user routes function which takes the data access manager and returns an axum router. The router has routes wich take search urls and handler functions in wrappers which indicate the request type. The router also includes the data access manager using the with_state functions. This enables the handler functions to access the data access manager wrapped in a State type which is simpley unwrapped. The function can also access the application json through parameters wrapped in Json as well as variables in the url through parameters wrapped in Path. axum knows which parameters the functions shoud require through the inclusion of a with state and a path parameters marked within paranthesis {path_parameter} in the url. The order of the parameters is State, Path, and lastly Json if all three are included. The handler functions take theese areguments and pass them to the intended Controller and then takes the result of the controller and returns a result that can be used by users f.ex in the form of Html or Json. The characteristic of these types that axum deems can be used by users is that they implement the into response trait. To use the error handling the result error also have to implement into response which is done in /backend/src/gate/routes/error.rs

Lastly in main I create a data access manager through its ::new fuction and pass it to the user routes which I save as a variable.  Then this is nesteed in the main rputer which in turned served by axum. I also moved the server serving 

To summerize:

main <- gate[router <- handler] <- data_access[controller <- base_functions{utils}] <-DB
|_data_access_manager--------------------------------------------------_|

### 7. Create request context

I created a request context struct in backend/src/request_context/mod.rs which may hold the logged in users id. I also reserved user id 0 for testing and an error if someone would try to use that.
