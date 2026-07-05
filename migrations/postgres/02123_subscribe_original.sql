-- migrate:up
ALTER TABLE "subscribe"
    ADD COLUMN "show_original_price" BOOLEAN NOT NULL DEFAULT false;

-- migrate:down
ALTER TABLE "subscribe"
DROP COLUMN "show_original_price";

