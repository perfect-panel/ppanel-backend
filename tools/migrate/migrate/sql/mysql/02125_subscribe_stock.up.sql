-- Update the `subscribe` table to set `inventory` to -1 where it is currently 0
UPDATE `subscribe`
SET `inventory` = -1
WHERE `inventory` = 0;