-- drop all sessions for a username or a database name
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
usename = 'postgres_user' OR datname = 'test_db';

DROP DATABASE IF EXISTS test_db;
DROP USER IF EXISTS postgres_user; 

CREATE USER postgres_user WITH PASSWORD 'super_secret_password';
CREATE DATABASE test_db WITH OWNER postgres_user ENCODING = 'UTF8';

-- Maybe this cannot be done in a migration.