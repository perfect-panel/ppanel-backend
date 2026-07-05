
-- This migration script reverts the inventory values in the 'subscribe' table
UPDATE `subscribe`
SET `inventory` = 0
WHERE `inventory` = -1;