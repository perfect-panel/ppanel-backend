INSERT INTO `system` (`category`, `key`, `value`, `type`, `desc`, `created_at`, `updated_at`)
SELECT 'site', 'CustomData', '{
  "kr_website_id": ""
}', 'string', 'Custom Data', '2025-04-22 14:25:16.637', '2025-10-14 15:47:19.187'
    WHERE NOT EXISTS (
    SELECT 1 FROM `system` WHERE `category` = 'site' AND `key` = 'CustomData'
);
