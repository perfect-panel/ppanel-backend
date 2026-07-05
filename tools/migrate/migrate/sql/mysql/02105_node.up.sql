CREATE TABLE IF NOT EXISTS `servers` (
        `id` bigint NOT NULL AUTO_INCREMENT,
        `name` varchar(100) COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Server Name',
        `country` varchar(128) COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Country',
        `city` varchar(128) COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'City',
        `ratio` decimal(4,2) NOT NULL DEFAULT '0.00' COMMENT 'Traffic Ratio',
        `address` varchar(100) COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Server Address',
        `sort` bigint NOT NULL DEFAULT '0' COMMENT 'Sort',
        `protocols` text COLLATE utf8mb4_general_ci COMMENT 'Protocol',
        `last_reported_at` datetime(3) DEFAULT NULL COMMENT 'Last Reported Time',
        `created_at` datetime(3) DEFAULT NULL COMMENT 'Creation Time',
        `updated_at` datetime(3) DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `nodes` (
     `id` bigint NOT NULL AUTO_INCREMENT,
     `name` varchar(100) COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Node Name',
     `tags` varchar(255) COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Tags',
     `port` smallint unsigned NOT NULL DEFAULT '0' COMMENT 'Connect Port',
     `address` varchar(255) COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Connect Address',
     `server_id` bigint NOT NULL DEFAULT '0' COMMENT 'Server ID',
     `protocol` varchar(100) COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Protocol',
     `enabled` tinyint(1) NOT NULL DEFAULT '1' COMMENT 'Enabled',
     `created_at` datetime(3) DEFAULT NULL COMMENT 'Creation Time',
     `updated_at` datetime(3) DEFAULT NULL COMMENT 'Update Time',
     PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
