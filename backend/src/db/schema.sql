begin;

create table events (
  id bigserial primary key,
  event_type text not null,
  external_id text not null,
  created_at timestamptz not null,
  meta jsonb
);

create unique index events_unique_external_id on events (event_type, external_id);
create index events_by_created_at on events (created_at);

end;