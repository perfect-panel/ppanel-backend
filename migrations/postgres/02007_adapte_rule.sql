-- migrate:up
ALTER TABLE "server_rule_group"
ADD COLUMN "default" BOOLEAN NOT NULL DEFAULT false,
ADD COLUMN "type" VARCHAR(100) NOT NULL DEFAULT '';

-- migrate:down
ALTER TABLE "server_rule_group"
DROP COLUMN "default",
DROP COLUMN "type";

