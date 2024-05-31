DROP DATABASE IF EXISTS mydb;
CREATE DATABASE mydb CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_bin;
GRANT ALL ON mydb.* TO 'shogo'@'%';
USE mydb;

CREATE TABLE users (
    username VARCHAR(100) NOT NULL,
    password VARCHAR(100) NOT NULL
);
