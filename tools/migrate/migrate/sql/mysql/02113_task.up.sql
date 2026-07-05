DROP TABLE IF EXISTS `email_task`;
CREATE TABLE `task` (
                        `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'ID',
                        `type` tinyint NOT NULL COMMENT 'Task Type',
                        `scope` text COLLATE utf8mb4_general_ci COMMENT 'Task Scope',
                        `content` text COLLATE utf8mb4_general_ci COMMENT 'Task Content',
                        `status` tinyint NOT NULL DEFAULT '0' COMMENT 'Task Status: 0: Pending, 1: In Progress, 2: Completed, 3: Failed',
                        `errors` text COLLATE utf8mb4_general_ci COMMENT 'Task Errors',
                        `total` bigint unsigned NOT NULL DEFAULT '0' COMMENT 'Total Number',
                        `current` bigint unsigned NOT NULL DEFAULT '0' COMMENT 'Current Number',
                        `created_at` datetime(3) DEFAULT NULL COMMENT 'Creation Time',
                        `updated_at` datetime(3) DEFAULT NULL COMMENT 'Update Time',
                        PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;