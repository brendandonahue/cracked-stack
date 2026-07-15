-- Auth access method
DEFINE ACCESS OVERWRITE account ON DATABASE TYPE RECORD SIGNUP { CREATE user CONTENT { email: $email, name: $name, pass: crypto::argon2::generate($pass), role: $role, created_at: time::now() } } SIGNIN { SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) LIMIT 1 } DURATION FOR TOKEN 1d, FOR SESSION 4w2d;

-- User table
DEFINE TABLE OVERWRITE user SCHEMAFULL PERMISSIONS FOR select, update, delete WHERE id = $auth.id OR $auth.role = "admin", FOR create WHERE $access = "account" OR $auth.role = "admin";
DEFINE FIELD OVERWRITE email      ON user TYPE string;
DEFINE FIELD OVERWRITE name       ON user TYPE string;
DEFINE FIELD OVERWRITE pass       ON user TYPE string;
DEFINE FIELD OVERWRITE role       ON user TYPE string;
DEFINE FIELD OVERWRITE created_at ON user TYPE datetime;
DEFINE INDEX OVERWRITE unique_email ON TABLE user FIELDS email UNIQUE;

-- Item table (generic demo resource — replace with your domain model)
DEFINE TABLE OVERWRITE item SCHEMAFULL PERMISSIONS FOR select WHERE created_by = $auth.id OR $auth.role = "admin", FOR create WHERE $auth.id != NONE, FOR update, delete WHERE created_by = $auth.id OR $auth.role = "admin";
DEFINE FIELD OVERWRITE title       ON item TYPE string;
DEFINE FIELD OVERWRITE description ON item TYPE option<string>;
DEFINE FIELD OVERWRITE created_by  ON item TYPE record<user>;
DEFINE FIELD OVERWRITE created_at  ON item TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE updated_at  ON item TYPE datetime DEFAULT time::now();

-- Password reset tokens
DEFINE TABLE OVERWRITE password_reset_tokens SCHEMAFULL PERMISSIONS FOR create, select, update, delete WHERE email = $auth.email OR $auth.role = "admin";
DEFINE FIELD OVERWRITE token      ON password_reset_tokens TYPE string;
DEFINE FIELD OVERWRITE email      ON password_reset_tokens TYPE string;
DEFINE FIELD OVERWRITE expires_at ON password_reset_tokens TYPE datetime;
DEFINE FIELD OVERWRITE used       ON password_reset_tokens TYPE bool DEFAULT false;
DEFINE INDEX OVERWRITE idx_reset_token ON password_reset_tokens FIELDS token UNIQUE;
DEFINE INDEX OVERWRITE idx_reset_email ON password_reset_tokens FIELDS email;

-- File uploads (metadata; binary content is stored on disk under UPLOAD_DIR)
DEFINE TABLE OVERWRITE file SCHEMAFULL PERMISSIONS FOR select WHERE public = true OR owner = $auth.id OR $auth.role = "admin", FOR create WHERE $auth.id != NONE, FOR update, delete WHERE owner = $auth.id OR $auth.role = "admin";
DEFINE FIELD OVERWRITE filename      ON file TYPE string;
DEFINE FIELD OVERWRITE original_name ON file TYPE string;
DEFINE FIELD OVERWRITE content_type  ON file TYPE string;
DEFINE FIELD OVERWRITE size          ON file TYPE int;
DEFINE FIELD OVERWRITE owner         ON file TYPE record<user>;
DEFINE FIELD OVERWRITE public        ON file TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE created_at    ON file TYPE datetime DEFAULT time::now();

-- Enable GraphQL
DEFINE CONFIG OVERWRITE GRAPHQL AUTO;


