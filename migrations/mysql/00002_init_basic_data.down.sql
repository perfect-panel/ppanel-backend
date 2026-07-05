-- 000002_init_data.down.sql
SET
FOREIGN_KEY_CHECKS = 0;

DELETE
FROM `auth_method`
WHERE `id` IN (1, 2, 3, 4, 5, 6, 7, 8);
DELETE
FROM `payment`
WHERE `id` = -1;
DELETE
FROM `subscribe_type`
WHERE `id` IN (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
DELETE
FROM `system`
WHERE `id` IN
      (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
       31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41);

SET
FOREIGN_KEY_CHECKS = 1;