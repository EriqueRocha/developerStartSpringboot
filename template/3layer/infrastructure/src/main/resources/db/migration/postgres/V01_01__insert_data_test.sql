--Schema padrão: public
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

INSERT INTO admins (email, password, name, created_at, updated_at)
VALUES
    ('admin1@{{yourDomain}}', '$2a$10$gJoXoMZ0nsrX8TzW4.5pveCnyo.v8I/0HyBySBYX7PaQU3yj.tv3C', 'Admin One', NOW(), NOW()),
    ('admin2@{{yourDomain}}', '$2a$10$N9qo8uLOickgx2ZMRZo5e.7a6Kcn8WgXJZKie0t92xSu3kSjtLhCy', 'Admin Two', NOW(), NOW());

INSERT INTO {{tableName}} (email, password, name, created_at, updated_at)
VALUES
    ('{{userEntity}}1@{{yourDomain}}', '$2a$10$cCzcz2EiyQxC5tejMn7e7.1piC3dFvZ68a8FmXzFeiQmjIAHqRFQu', 'Author One', NOW(), NOW()),
    ('{{userEntity}}2@{{yourDomain}}', '$2a$10$9nbF6R7552YblV3Fkk6m5O0qmJUm1bS0g7cUB495UvA3r0xY1YqNa', 'Author Two', NOW(), NOW());