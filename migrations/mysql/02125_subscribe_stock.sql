-- migrate:up
-- Update the `subscribe` table to set `inventory` to -1 where it is currently 0
UPDATE `subscribe`
SET `inventory` = -1
WHERE `inventory` = 0;
-- migrate:down

-- This migration script reverts the inventory values in the 'subscribe' table
UPDATE `subscribe`
SET `inventory` = 0
WHERE `inventory` = -1;
