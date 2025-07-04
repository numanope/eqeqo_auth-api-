\c postgres
DROP DATABASE IF EXISTS auth_api;
CREATE DATABASE auth_api;
\c auth_api

CREATE EXTENSION IF NOT EXISTS pgcrypto;

DROP SCHEMA IF EXISTS people CASCADE;
CREATE SCHEMA people;

DROP TYPE IF EXISTS people.document_type CASCADE;
CREATE TYPE people.document_type AS ENUM ('DNI','CE','RUC');

DROP TYPE IF EXISTS people.person_type CASCADE;
CREATE TYPE people.person_type AS ENUM ('N','J');

DROP TABLE IF EXISTS services;
CREATE TABLE services (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

DROP TABLE IF EXISTS people.permissions;
CREATE TABLE people.permissions (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL DEFAULT '' UNIQUE
);

DROP TABLE IF EXISTS people.roles;
CREATE TABLE people.roles (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

DROP TABLE IF EXISTS people.role_permissions;
CREATE TABLE people.role_permissions (
  role_id INTEGER NOT NULL REFERENCES people.roles(id) ON DELETE CASCADE,
  permission_id INTEGER NOT NULL REFERENCES people.permissions(id) ON DELETE CASCADE,
  PRIMARY KEY (role_id, permission_id)
);

DROP TABLE IF EXISTS people.people;
CREATE TABLE people.people (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  created_at BIGINT NOT NULL DEFAULT (EXTRACT(EPOCH FROM NOW())::BIGINT),
  removed_at BIGINT DEFAULT NULL
);

DROP TABLE IF EXISTS people.service_roles;
CREATE TABLE people.service_roles (
  service_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
  role_id INTEGER NOT NULL REFERENCES people.roles(id) ON DELETE CASCADE,
  PRIMARY KEY (service_id, role_id)
);

DROP TABLE IF EXISTS people.person_service_roles;
CREATE TABLE people.person_service_roles (
  person_id   INTEGER NOT NULL
    REFERENCES people.people(id) ON DELETE CASCADE,
  service_id  INTEGER NOT NULL,
  role_id     INTEGER NOT NULL,
  PRIMARY KEY (person_id, service_id, role_id),
  FOREIGN KEY (service_id, role_id)
    REFERENCES people.service_roles(service_id, role_id)
    ON DELETE CASCADE
);
