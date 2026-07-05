DROP TABLE IF EXISTS `email_task`;
CREATE TABLE `email_task` (
                               `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'ID',
                               `subject` varchar(255) COLLATE utf8mb4_general_ci NOT NULL COMMENT 'Email Subject',
                               `content` text COLLATE utf8mb4_general_ci NOT NULL COMMENT 'Email Content',
                               `recipient` text COLLATE utf8mb4_general_ci NOT NULL COMMENT 'Email Recipient',
                               `scope` varchar(50) COLLATE utf8mb4_general_ci NOT NULL COMMENT 'Email Scope',
                               `register_start_time` datetime(3) DEFAULT NULL COMMENT 'Register Start Time',
                               `register_end_time` datetime(3) DEFAULT NULL COMMENT 'Register End Time',
                               `additional` text COLLATE utf8mb4_general_ci COMMENT 'Additional Information',
                               `scheduled` datetime(3) NOT NULL COMMENT 'Scheduled Time',
                               `interval` tinyint unsigned NOT NULL COMMENT 'Interval in Seconds',
                               `limit` bigint unsigned NOT NULL COMMENT 'Daily send limit',
                               `status` tinyint unsigned NOT NULL COMMENT 'Daily Status',
                               `errors` text COLLATE utf8mb4_general_ci NOT NULL COMMENT 'Errors',
                               `total` bigint unsigned NOT NULL DEFAULT '0' COMMENT 'Total Number',
                               `current` bigint unsigned NOT NULL DEFAULT '0' COMMENT 'Current Number',
                               `created_at` datetime(3) DEFAULT NULL COMMENT 'Creation Time',
                               `updated_at` datetime(3) DEFAULT NULL COMMENT 'Update Time',
                               PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

SET FOREIGN_KEY_CHECKS = 1;
