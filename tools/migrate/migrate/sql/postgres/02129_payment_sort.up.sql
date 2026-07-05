ALTER TABLE "payment"
    ADD COLUMN IF NOT EXISTS "sort" bigint NOT NULL DEFAULT 0;

UPDATE "payment"
SET "sort" = "id"
WHERE "sort" = 0;
