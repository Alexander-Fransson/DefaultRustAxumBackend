-- drop all sessions for a username or a database name
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
usename = 'root' OR datname = 'test_db';

DROP DATABASE IF EXISTS test_db;
DROP USER IF EXISTS 'root'; -- root is a reserved keyword so it has to be quoted to be used as a username

CREATE USER root WITH PASSWORD 'super_secret_password';
CREATE DATABASE test_db WITH OWNER 'root' ENCODING = 'UTF8';

