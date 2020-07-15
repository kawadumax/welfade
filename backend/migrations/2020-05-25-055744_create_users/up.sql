-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY, 
  name VARCHAR NOT NULL, 
  email VARCHAR NOT NULL, 
  password VARCHAR NOT NULL, 
  session_token VARCHAR
);

COMMENT on COLUMN users.id IS '自動付与のid。';
COMMENT on COLUMN users.name IS 'ユーザーID。ログイン時に使う。';
COMMENT on COLUMN users.email IS 'メールアドレス';
COMMENT on COLUMN users.password IS 'パスワード';
COMMENT on COLUMN users.session_token IS 'ログイン時に発行するトークン';