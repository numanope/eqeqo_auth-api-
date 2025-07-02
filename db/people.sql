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
  hash TEXT NOT NULL,
  name TEXT NOT NULL,
  person_type people.people_type DEFAULT 'N',
  document_type people.document_type DEFAULT 'DNI',
  document_number TEXT,
  created_at NUMERIC NOT NULL,
  removed_at NUMERIC DEFAULT NULL,
  UNIQUE (document_type, document_number)
);
ALTER TABLE people.people ALTER COLUMN created_at SET DEFAULT EXTRACT(EPOCH FROM NOW());

CREATE TABLE people.role_permissions (
  id SERIAL PRIMARY KEY,
  role_id INTEGER NOT NULL REFERENCES people.roles(id) ON DELETE CASCADE,
  permission TEXT NOT NULL,
  UNIQUE (role_id, permission)
);

CREATE TABLE people.people_role (
  id SERIAL PRIMARY KEY,
  person_id INTEGER REFERENCES people.people(id) NOT NULL ON DELETE CASCADE,
  role_id INTEGER REFERENCES people.roles(id) NOT NULL ON DELETE CASCADE,
  UNIQUE (person_id, role_id)
);

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
