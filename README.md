# DefaultRustAxumBackend
A default axum API that I can later fork from when I want to do something more special. Features shall be limited to getting usernames and create user when not logged in and CRUD user when logged in.


## Dev Diary

// maybe it would have been best to make the authentication middleware before the crud

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

I also moved all the structs that shape the requests to the views folder so that they can be reused by the frontend. Important to know is to derive from row on the structs that sqlx turns its results into as in backend/src/views/user.rs becouse sqlx by default returns a useless row type. (formerly I called views data_shapes maybe that was more descriptive, maybe data_views is a compromise?)

Afterwords we move on to the gate layer. Here I created modules for routes and middlewares and since we dont need any middlewares at this stage I just create a user routes module at backend/src/gate/routes/user_routes.rs. Here the main public function is the user routes function which takes the data access manager and returns an axum router. The router has routes wich take search urls and handler functions in wrappers which indicate the request type. The router also includes the data access manager using the with_state functions. This enables the handler functions to access the data access manager wrapped in a State type which is simpley unwrapped. The function can also access the application json through parameters wrapped in Json as well as variables in the url through parameters wrapped in Path. axum knows which parameters the functions shoud require through the inclusion of a with state and a path parameters marked within paranthesis {path_parameter} in the url. The order of the parameters is State, Path, and lastly Json if all three are included. The handler functions take theese areguments and pass them to the intended Controller and then takes the result of the controller and returns a result that can be used by users f.ex in the form of Html or Json. The characteristic of these types that axum deems can be used by users is that they implement the into response trait. To use the error handling the result error also have to implement into response which is done in /backend/src/gate/routes/error.rs

Lastly in main I create a data access manager through its ::new fuction and pass it to the user routes which I save as a variable.  Then this is nesteed in the main rputer which in turned served by axum. I also moved the server serving

I also made unit tests on the controllers and integration tests on the routers, it might be worth mentioning that serial test and reqwest were dependencies used 

To summerize:

main <- gate[router <- handler] <- data_access[controller <- base_functions{utils}] <-DB
|_data_access_manager--------------------------------------------------_|

### 7. Create request context

I created a request context struct in backend/src/request_context/mod.rs which may hold the logged in users id. I also reserved user id 0 for testing and an error if someone would try to use that.

Totally out of context but I think it would be awesume if you could have a macro that took the dominant view struct and changed the sql files if they have fields that the struct did not have and reverse. It would also be cool if you could have a linter that made sure the related struct like UserForCreate for User did not have fields the dominant struct did not have. Dont think Ill develop that in a while though.

### 8? Create minimal middewares for request context

Added tower to test middlewares in backend/src/gate/middlewares/middlewares_test.rs
It provides the service builder and service ext which enables the oncshot function that makes a request to a router without having it on a port. At first I ran into a lot of problems when making the routest so I made a lot of experiments in the middlewares test file. Maybe it is pedagogic if I am to make a tutorial video to make explain concepts in tests before doing big things.

But, basically a middleware is a function that is put in a layer or a route layer below the router. They are executed from bottom up and they have access to the request and the next parameter. The request parameter contains the body, extensions and headers and you can insert or remove things from them, changing the request as it passes through your router. This can be used for authentication verification and so as the middlewares can be placed anywhare along the router including nested stuff enabeling having some middlewares running only before some handlers. 

What I have done is a a mw that inserts a root request_context struct into the request extension in backend/src/gate/middlewares/mw_implant_request_context.rs. Later I will make so that this only happens if you have a token cookie. Then I made another middleware that checks if there is such a request_context in the extensions in backend/src/gate/middlewares/mw_require_request_context.rs. It does this by the custom extractor implemented for the request context struct in backend/src/gate/custom_extractors/ex_request_context.rs. This is where I ran into problems becouse I have different errors and results for middlewares and the custom extractors and the result implemented for the from request parts function determines what can be extracted by middlewares and hamdlers in the parameters. Thus I made some experiments in the test file and it seams to work fine now so I added the implant request context mw to the main routes and the require request context to the user routes. Experimenting with middlewares could be done as an earlyer step probably.

### 9 Use cookies in middlewares

This needs the crate tower-cookies for the cookies middleware that allows cookies extraction. To use the cookies extractor you need to add a Cookie manager layer at the end of the routes as done in main and the middleware tests. Then specific cokkies can be accessed from the middleware using the .get, .name and .value can be used to access the different stuff. I also updated the middlewares error to handdle the cookie value extraction in backend/src/gate/middlewares/mw_implant_request_context.rs. A useful function to use an error in a map is the transponse function which turns an Option(Result) into a Result(Option). Lastly it is worth to mention that the cookie accessed by the cookie manager layer is a header with the key of "Cookie" and a value of "x=x". 

### Testing

To make testing easyer I installed cargo-make with $ cargo install cargo-make $ which allows you to make custom scripts in backend/Makefile.toml

### 10 JWT authetication

Apperently it is proper to have a unique hash fore each user stored in the database to pervent attacks using hash dicitionaries, ensure users can have the same password, protect against leaks and other security reasons so encryption salt fore password and token is added to the user table in backend/db/sql/migrations/01_recreate_tables.sql.

In backend/src/views/user.rs tables to handle login and validation of passwords was created.
I also made a list by name base and user controller but that is mostly irrelevant for authentication.
To handle the encryption and hashing of the password I created a new crypt crate. 
Here I created the hash and validate password functions in backend/src/crypt/password.rs. 
For the encryption I used argon2 for bacouse it uses high memory consumption and is slow which makes it harder for attackers to try billions of guesses including with GPUs and ASICs. 
Other alternative encryptiion methods are sha512, Poly1305 which is verry fast and used for message encryption and Blake2b which is both fast and secure, Speed is bad in passwordhashing though so poly can be usde for message encryption and blake can be used for jwt encryption. 
I also created an encrypt into base64 function and added the base64 create. 

Then in backend/src/data_access/user_controller/mod.rs I created a login and register user function.
Register uses the uuid crate to generate a salt and transforms it to b64 to later use it as the encryption content for hashing the password.
In the login function users are queried by email which are then filtered by password.

// I created string to b64 and vice versa tests in backend/src/utils/base64.rs


////////////////////

// continue with the generateon of a jwt token and using it for authentication in the request context generation.

// what shall be created is a create_token function that returns user_id_b64u. expiration_b64u.singanture_b64u

// he uses the time library 

// he creates time utils like 
* now_utc which returns an offset date time struct 
* format_time which takes the offset date time and returns a string
* now_utc_plus_sec_str which takes a f64 and returns a string of such many seconds after now
* parse_utc which takes a str& and returns an offsetdatetime

// he creates b64 utils
* b64 encode & decode

// he creates a token struct

// he creates generate token and token sign into b64u private functions in crypt/token

// he aso creates a validate token private function

// he creates public functions that use each fuction

// he implements display for token struct, this is used to mkae the toekn a string of the right format

// he also implements the from str for token which takes a x.x.x.string and returns a token struct where ident and expiration is decoded from b64

// then after making tests he creates the priveate generate token function content and returns the token struct

// he creates the content for the validate token sign and expiration

// he creates the content for the token sign into b64u which uses the sha512 encrypt function to generate a signature, this signature is then evaluated en the validate token function

// the in web/mod he creates set token cookie and remove token cookie functions in adition to an AUTH_TOKEN cookie name const.

// set token cookie generates a token from the users id and token salt and pits it into a cookie which is added to the tower cookies object

// he then changes the web/routs/login function to set a cookie

// then he creates a ctx_resolve function wihich gets the auth cookie, parses the cookie string to a token struct, gets a user theough the UserBmc, validates the token using the users token salt, updates the token by setting a new and creates new request context. 

// the ctx resplve middleware runs the above function and if it retuns an error it removes the cookie from the tower cookies.

// the ctx resolve middleware is added to the main routes.

// then he creates a logof paload struct that just has logof as a bool varoable

// he then creates a remove cookie function in web mod

// then he just creates a logoff handler which just removes the cookie


