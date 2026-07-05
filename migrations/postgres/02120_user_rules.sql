-- migrate:up
ALTER TABLE "user"
    ADD COLUMN "rules" TEXT NULL;

-- migrate:down
ALTER TABLE "user"
DROP COLUMN IF EXISTS "rules";

