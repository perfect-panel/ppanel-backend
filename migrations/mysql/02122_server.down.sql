CREATE TABLE IF NOT EXISTS `server`
(
    `id`               bigint                                                        NOT NULL AUTO_INCREMENT,
    `name`             varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Node Name',
    `tags`             varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Tags',
    `country`          varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Country',
    `city`             varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'City',
    `latitude`         varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'latitude',
    `longitude`        varchar(128) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'longitude',
    `server_addr`      varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Server Address',
    `relay_mode`       varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci  NOT NULL DEFAULT 'none' COMMENT 'Relay Mode',
    `relay_node`       text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Relay Node',
    `speed_limit`      bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Speed Limit',
    `traffic_ratio`    decimal(4, 2)                                                 NOT NULL DEFAULT '0.00' COMMENT 'Traffic Ratio',
    `group_id`         bigint                                                                 DEFAULT NULL COMMENT 'Group ID',
    `protocol`         varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci  NOT NULL DEFAULT '' COMMENT 'Protocol',
    `config`           text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Config',
    `enable`           tinyint(1)                                                    NOT NULL DEFAULT '1' COMMENT 'Enabled',
    `sort`             bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Sort',
    `last_reported_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Last Reported Time',
    `created_at`       datetime(3)                                                            DEFAULT NULL COMMENT 'Creation Time',
    `updated_at`       datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`),
    KEY `idx_group_id` (`group_id`)
    ) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_general_ci;
