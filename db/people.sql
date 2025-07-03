CREATE SCHEMA IF NOT EXISTS people;

CREATE TYPE people.document_type AS ENUM ('DNI', 'CE', 'RUC');
CREATE TYPE people.people_type AS ENUM ('N', 'J');

CREATE TABLE services(
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE people.roles (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE people.permissions (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL DEFAULT ''::TEXT UNIQUE
);

CREATE TABLE IF NOT EXISTS people.people (
  id SERIAL PRIMARY KEY,
  password_hash TEXT NOT NULL,
  username TEXT NOT NULL UNIQUE,
  created_at BIGINT NOT NULL DEFAULT (EXTRACT(EPOCH FROM NOW())::BIGINT),
  removed_at BIGINT DEFAULT NULL
);

CREATE TABLE people.role_permissions (
  id SERIAL PRIMARY KEY,
  role_id INTEGER NOT NULL REFERENCES people.roles(id) ON DELETE CASCADE,
  permission_id INTEGER NOT NULL REFERENCES people.permissions(id) ON DELETE CASCADE,
  UNIQUE (role_id, permission_id)
);
CREATE INDEX ON people.role_permissions(role_id);
CREATE INDEX ON people.role_permissions(permission_id);

CREATE TABLE people.service_roles (
  id SERIAL PRIMARY KEY,
  service_id INTEGER NOT NULL REFERENCES services(id) ON DELETE CASCADE,
  role_id INTEGER NOT NULL REFERENCES people.roles(id) ON DELETE CASCADE,
  UNIQUE (service_id, role_id)
);

CREATE TABLE people.person_service_roles (
  id SERIAL PRIMARY KEY,
  person_id INTEGER NOT NULL REFERENCES people.people(id) ON DELETE CASCADE,
  service_role_id INTEGER NOT NULL REFERENCES people.service_roles(id) ON DELETE CASCADE,
  UNIQUE (person_id, service_role_id)
);

CREATE INDEX ON people.service_roles(service_id);
CREATE INDEX ON people.service_roles(role_id);
CREATE INDEX ON people.person_service_roles(person_id);
CREATE INDEX ON people.person_service_roles(service_role_id);
