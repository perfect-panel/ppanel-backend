CREATE TABLE IF NOT EXISTS `user_balance_log`
(
    `id`         bigint     NOT NULL AUTO_INCREMENT,
    `user_id`    bigint     NOT NULL COMMENT 'User ID',
    `amount`     bigint     NOT NULL COMMENT 'Amount',
    `type`       tinyint(1) NOT NULL COMMENT 'Type: 1: Recharge 2: Withdraw 3: Payment 4: Refund 5: Reward',
    `order_id`   bigint      DEFAULT NULL COMMENT 'Order ID',
    `balance`    bigint     NOT NULL COMMENT 'Balance',
    `created_at` datetime(3) DEFAULT NULL COMMENT 'Creation Time',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`)
    ) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `user_commission_log`
(
    `id`         bigint NOT NULL AUTO_INCREMENT,
    `user_id`    bigint NOT NULL COMMENT 'User ID',
    `order_no`   varchar(191) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT 'Order No.',
    `amount`     bigint NOT NULL COMMENT 'Amount',
    `created_at` datetime(3)                                                   DEFAULT NULL COMMENT 'Creation Time',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`)
    ) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `user_gift_amount_log`
(
    `id`                bigint     NOT NULL AUTO_INCREMENT,
    `user_id`           bigint     NOT NULL COMMENT 'User ID',
    `user_subscribe_id` bigint                                                        DEFAULT NULL COMMENT 'Deduction User Subscribe ID',
    `order_no`          varchar(191) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT 'Order No.',
    `type`              tinyint(1) NOT NULL COMMENT 'Type: 1: Increase 2: Reduce',
    `amount`            bigint     NOT NULL COMMENT 'Amount',
    `balance`           bigint     NOT NULL COMMENT 'Balance',
    `remark`            varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT '' COMMENT 'Remark',
    `created_at`        datetime(3)                                                   DEFAULT NULL COMMENT 'Creation Time',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`)
    ) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `user_login_log`
(
    `id`         bigint                                                        NOT NULL AUTO_INCREMENT,
    `user_id`    bigint                                                        NOT NULL COMMENT 'User ID',
    `login_ip`   varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'Login IP',
    `user_agent` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci         NOT NULL COMMENT 'UserAgent',
    `success`    tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Login Success',
    `created_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Creation Time',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`)
    ) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `user_reset_subscribe_log`
(
    `id`                BIGINT     NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `user_id`           BIGINT     NOT NULL COMMENT 'User ID',
    `type`              TINYINT(1) NOT NULL COMMENT 'Type: 1: Auto 2: Advance 3: Paid',
    `order_no`          VARCHAR(255)        DEFAULT NULL COMMENT 'Order No.',
    `user_subscribe_id` BIGINT     NOT NULL COMMENT 'User Subscribe ID',
    `created_at`        DATETIME   NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Creation Time',
    INDEX `idx_user_id` (`user_id`),
    INDEX `idx_user_subscribe_id` (`user_subscribe_id`)
    ) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `user_subscribe_log`
(
    `id`                bigint                                                        NOT NULL AUTO_INCREMENT,
    `user_id`           bigint                                                        NOT NULL COMMENT 'User ID',
    `user_subscribe_id` bigint                                                        NOT NULL COMMENT 'User Subscribe ID',
    `token`             varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'Token',
    `ip`                varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'IP',
    `user_agent`        text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci         NOT NULL COMMENT 'UserAgent',
    `created_at`        datetime(3) DEFAULT NULL COMMENT 'Creation Time',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_user_subscribe_id` (`user_subscribe_id`)
    ) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `message_log`
(
    `id`         bigint                                                        NOT NULL AUTO_INCREMENT,
    `type`       varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci  NOT NULL DEFAULT 'email' COMMENT 'Message Type',
    `platform`   varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci  NOT NULL DEFAULT 'smtp' COMMENT 'Platform',
    `to`         text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci         NOT NULL COMMENT 'To',
    `subject`    varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Subject',
    `content`    text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Content',
    `status`     tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Status',
    `created_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Create Time',
    `updated_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`)
    ) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_general_ci;

DROP TABLE IF EXISTS `system_logs`;