ALTER TABLE "servers"
  ALTER COLUMN "last_reported_at" TYPE timestamp(3)
  USING "last_reported_at" AT TIME ZONE 'UTC';
