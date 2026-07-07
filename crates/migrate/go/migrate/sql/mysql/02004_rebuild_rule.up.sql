-- migrations/02003_rebuild_rule.up.sql
-- Purpose: rebuilding server rule table
-- Author: PPanel Team, 2025-04-21

DROP TABLE IF EXISTS `server_rule_group`;

CREATE TABLE `server_rule_group`
(
    `id`         BIGINT UNSIGNED                                              NOT NULL AUTO_INCREMENT,
    `name`       VARCHAR(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Rule Group Name',
    `icon`       VARCHAR(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Rule Group Icon',
    `tags`       TEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Selected Node Tags',
    `rules`      MEDIUMTEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Rules',
    `enable`     TINYINT(1)                                                   NOT NULL DEFAULT 1 COMMENT 'Rule Group Enable',
    `created_at` DATETIME(3) COMMENT 'Creation Time',
    `updated_at` DATETIME(3) COMMENT 'Update Time',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uni_server_rule_group_name` (`name`),
    INDEX `idx_enable` (`enable`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;