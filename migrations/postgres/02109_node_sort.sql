-- migrate:up
ALTER TABLE "nodes"
    ADD COLUMN "sort" INTEGER NOT NULL DEFAULT 0;

-- migrate:down
ALTER TABLE "nodes"
DROP COLUMN "sort";

