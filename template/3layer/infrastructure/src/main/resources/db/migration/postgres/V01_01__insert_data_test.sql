--default schema: public
CREATE TABLE IF NOT EXISTS admins (
      id BIGSERIAL PRIMARY KEY,
      email VARCHAR(255) NOT NULL UNIQUE,
      password VARCHAR(255) NOT NULL,
      name VARCHAR(255) NOT NULL,
      created_at TIMESTAMP NOT NULL DEFAULT NOW(),
      updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS {{tableName}} (
       id BIGSERIAL PRIMARY KEY,
       email VARCHAR(255) NOT NULL UNIQUE,
       password VARCHAR(255) NOT NULL,
       name VARCHAR(255) NOT NULL,
       created_at TIMESTAMP NOT NULL DEFAULT NOW(),
       updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_admins_email ON admins (email);
CREATE INDEX IF NOT EXISTS idx_authors_email ON {{tableName}} (email);

-- default password: test123456
INSERT INTO admins (email, password, name, created_at, updated_at)
VALUES
    ('admin1@{{yourDomain}}', '$2a$10$XTcZJSBbcIdA7PD2Ta8fmu4EmZ2tasvrPoHM2BdUtd.mYi2I5EBFK', 'Admin One', NOW(), NOW()),
    ('admin2@{{yourDomain}}', '$2a$10$HAtgYRWeNHDeyA4kntuq6OrEMN8Qgz86XN0ftyg.wsBWXzTunAmKe', 'Admin Two', NOW(), NOW());

INSERT INTO {{tableName}} (email, password, name, created_at, updated_at)
VALUES
    ('{{userEntity}}1@{{yourDomain}}', '$2a$10$xi3eengxM5..Sa16AqgRU.cZ7lltDkacVlXLbYRqrzzttDVprHS06', '{{userEntity}} One', NOW(), NOW()),
    ('{{userEntity}}2@{{yourDomain}}', '$2a$10$VgpXD/oN91RpM/OH9s/3OO5B/BGrfpOAcV/0FPRPKu0ZJV1ITuZey', '{{userEntity}} Two', NOW(), NOW());