# DefaultRustAxumBackend
A default axum API that I can later fork from when I want to do something more special. Features shall be limited to getting usernames and create user when not logged in and CRUD user when logged in.


## Dev Diary

### 1. Create dev database
To start with the best first step is probably to create a dev database that can contain users for the get requests.
Some modules I am sure I will need are axum, sqlx and tokio with features full so Ill add those.
It is probably also good to add a .gitignore that ignores the target folder