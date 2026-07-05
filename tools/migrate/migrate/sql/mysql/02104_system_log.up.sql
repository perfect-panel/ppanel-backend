DROP TABLE IF EXISTS `user_balance_log`;
DROP TABLE IF EXISTS `user_commission_log`;
DROP TABLE IF EXISTS `user_gift_amount_log`;
DROP TABLE IF EXISTS `user_login_log`;
DROP TABLE IF EXISTS `user_reset_subscribe_log`;
DROP TABLE IF EXISTS `user_subscribe_log`;
DROP TABLE IF EXISTS `message_log`;
DROP TABLE IF EXISTS `system_logs`;
CREATE TABLE `system_logs` (
   `id` bigint NOT NULL AUTO_INCREMENT,
   `type` tinyint(1) NOT NULL DEFAULT '0' COMMENT 'Log Type: 1: Email Message 2: Mobile Message 3: Subscribe 4: Subscribe Traffic 5: Server Traffic 6: Login 7: Register 8: Balance 9: Commission 10: Reset Subscribe 11: Gift',
   `date` varchar(20) COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT 'Log Date',
   `object_id` bigint NOT NULL DEFAULT '0' COMMENT 'Object ID',
   `content` text COLLATE utf8mb4_general_ci NOT NULL COMMENT 'Log Content',
   `created_at` datetime(3) DEFAULT NULL COMMENT 'Create Time',
   PRIMARY KEY (`id`),
   KEY `idx_type` (`type`),
   KEY `idx_object_id` (`object_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;