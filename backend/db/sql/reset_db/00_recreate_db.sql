-- drop all sessions for a username or a database name
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
usename = scharlaton OR datname = test_db;

DROP DATABASE IF EXISTS test_db;
DROP USER IF EXISTS scharlaton; 

CREATE USER scharlaton WITH PASSWORD "super_secret_password";
CREATE DATABASE test_db WITH OWNER scharlaton ENCODING = "UTF8";

