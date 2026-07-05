CREATE TABLE IF NOT EXISTS `server_config_overrides`
(
    `id`          bigint                                                        NOT NULL AUTO_INCREMENT,
    `server_id`   bigint                                                        NOT NULL COMMENT 'Server ID',
    `ip_strategy` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci           DEFAULT NULL COMMENT 'IP strategy override, NULL means inherit',
    `dns`         text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'DNS override, NULL means inherit',
    `block`       text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Block override, NULL means inherit',
    `outbound`    text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Outbound override, NULL means inherit',
    `created_at`  datetime(3)                                                            DEFAULT NULL COMMENT 'Creation Time',
    `updated_at`  datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uni_server_config_overrides_server_id` (`server_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;
