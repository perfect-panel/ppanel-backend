ALTER TABLE "subscribe"
DROP COLUMN "group_id",
ADD COLUMN "language" VARCHAR(255) NOT NULL DEFAULT '';
DROP TABLE IF EXISTS "subscribe_group";
