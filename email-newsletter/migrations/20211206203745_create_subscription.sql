create table subscription (
  id uuid not null,
  email text not null unique,
  name text not null,
  subscribed_at timestamptz not null
);
