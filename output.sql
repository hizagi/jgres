-- Table Definitions 

CREATE TABLE products (id int PRIMARY KEY NOT NULL,name varchar NOT NULL,quantity int NOT NULL,value int NOT NULL);

CREATE TABLE customers (id int PRIMARY KEY NOT NULL,name varchar NOT NULL,bith_date varchar NOT NULL);



-- Table Data 

INSERT INTO products (id,name,quantity,value) VALUES (12,'product1',11,1200);

