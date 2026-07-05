ALTER TABLE "servers"
  ALTER COLUMN "last_reported_at" TYPE timestamptz
  USING "last_reported_at" AT TIME ZONE 'UTC';
