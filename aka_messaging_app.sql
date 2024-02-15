\echo "Delete and recreate aka messaging app database?"
\prompt "Return for yes or control-C to cancel " answer

DROP DATABASE aka_messaging_app;
CREATE DATABASE aka_messaging_app;
\connect aka_messaging_app;


\i aka_messaging_app-schema.sql;