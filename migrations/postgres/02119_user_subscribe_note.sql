-- migrate:up
ALTER TABLE "user_subscribe"
ADD COLUMN "note" VARCHAR(500) NOT NULL DEFAULT '';

-- migrate:down
ALTER TABLE "user_subscribe"
DROP COLUMN "note";

