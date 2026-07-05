-- 000001_init_schema.up.sql
SET FOREIGN_KEY_CHECKS = 0;

CREATE TABLE IF NOT EXISTS `ads`
(
    `id`         bigint                                                        NOT NULL AUTO_INCREMENT,
    `title`      varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Ads title',
    `type`       varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Ads type',
    `content`    text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Ads content',
    `target_url` varchar(512) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci          DEFAULT '' COMMENT 'Ads target url',
    `start_time` datetime                                                               DEFAULT NULL COMMENT 'Ads start time',
    `end_time`   datetime                                                               DEFAULT NULL COMMENT 'Ads end time',
    `status`     tinyint(1)                                                             DEFAULT '0' COMMENT 'Ads status,0 disable,1 enable',
    `created_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Create Time',
    `updated_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `announcement`
(
    `id`         bigint                                                        NOT NULL AUTO_INCREMENT,
    `title`      varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Title',
    `content`    text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Content',
    `show`       tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Show',
    `pinned`     tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Pinned',
    `popup`      tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Popup',
    `created_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Create Time',
    `updated_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `application`
(
    `id`             bigint                                                        NOT NULL AUTO_INCREMENT,
    `name`           varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '应用名称',
    `icon`           text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci         NOT NULL COMMENT '应用图标',
    `description`    text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT '更新描述',
    `subscribe_type` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci  NOT NULL DEFAULT '' COMMENT '订阅类型',
    `created_at`     datetime(3)                                                            DEFAULT NULL COMMENT '创建时间',
    `updated_at`     datetime(3)                                                            DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `application_config`
(
    `id`                        bigint NOT NULL AUTO_INCREMENT,
    `app_id`                    bigint NOT NULL                                               DEFAULT '0' COMMENT 'App id',
    `encryption_key`            text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Encryption Key',
    `encryption_method`         varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT 'Encryption Method',
    `domains`                   text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,
    `startup_picture`           text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,
    `startup_picture_skip_time` bigint NOT NULL                                               DEFAULT '0' COMMENT 'Startup Picture Skip Time',
    `created_at`                datetime(3)                                                   DEFAULT NULL COMMENT 'Create Time',
    `updated_at`                datetime(3)                                                   DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `application_version`
(
    `id`             bigint                                                        NOT NULL AUTO_INCREMENT,
    `url`            varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '应用地址',
    `version`        varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '应用版本',
    `platform`       varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci  NOT NULL DEFAULT '' COMMENT '应用平台',
    `is_default`     tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT '默认版本',
    `description`    text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT '更新描述',
    `application_id` bigint                                                                 DEFAULT NULL COMMENT '所属应用',
    `created_at`     datetime(3)                                                            DEFAULT NULL COMMENT '创建时间',
    `updated_at`     datetime(3)                                                            DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),
    KEY `fk_application_application_versions` (`application_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `auth_method`
(
    `id`         bigint                                                        NOT NULL AUTO_INCREMENT,
    `method`     varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'method',
    `config`     text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci         NOT NULL COMMENT 'OAuth Configuration',
    `enabled`    tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Is Enabled',
    `created_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Create Time',
    `updated_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uni_auth_method` (`method`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `coupon`
(
    `id`          bigint                                                        NOT NULL AUTO_INCREMENT,
    `name`        varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Coupon Name',
    `code`        varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Coupon Code',
    `count`       bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Count Limit',
    `type`        tinyint(1)                                                    NOT NULL DEFAULT '1' COMMENT 'Coupon Type: 1: Percentage 2: Fixed Amount',
    `discount`    bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Coupon Discount',
    `start_time`  bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Start Time',
    `expire_time` bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Expire Time',
    `user_limit`  bigint                                                        NOT NULL DEFAULT '0' COMMENT 'User Limit',
    `subscribe`   varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Subscribe Limit',
    `used_count`  bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Used Count',
    `enable`      tinyint(1)                                                    NOT NULL DEFAULT '1' COMMENT 'Enable',
    `created_at`  datetime(3)                                                            DEFAULT NULL COMMENT 'Create Time',
    `updated_at`  datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uni_coupon_code` (`code`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `document`
(
    `id`         bigint                                                        NOT NULL AUTO_INCREMENT,
    `title`      varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Document Title',
    `content`    text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Document Content',
    `tags`       varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Document Tags',
    `show`       tinyint(1)                                                    NOT NULL DEFAULT '1' COMMENT 'Show',
    `created_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Create Time',
    `updated_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`)
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

CREATE TABLE IF NOT EXISTS `order`
(
    `id`              bigint                                                        NOT NULL AUTO_INCREMENT,
    `parent_id`       bigint                                                                 DEFAULT NULL COMMENT 'Parent Order Id',
    `user_id`         bigint                                                        NOT NULL DEFAULT '0' COMMENT 'User Id',
    `order_no`        varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Order No',
    `type`            tinyint(1)                                                    NOT NULL DEFAULT '1' COMMENT 'Order Type: 1: Subscribe, 2: Renewal, 3: ResetTraffic, 4: Recharge',
    `quantity`        bigint                                                        NOT NULL DEFAULT '1' COMMENT 'Quantity',
    `price`           bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Original price',
    `amount`          bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Order Amount',
    `gift_amount`     bigint                                                        NOT NULL DEFAULT '0' COMMENT 'User Gift Amount',
    `discount`        bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Discount Amount',
    `coupon`          varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci          DEFAULT NULL COMMENT 'Coupon',
    `coupon_discount` bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Coupon Discount Amount',
    `commission`      bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Order Commission',
    `payment_id`      bigint                                                        NOT NULL DEFAULT '-1' COMMENT 'Payment Id',
    `method`          varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Payment Method',
    `fee_amount`      bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Fee Amount',
    `trade_no`        varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci          DEFAULT NULL COMMENT 'Trade No',
    `status`          tinyint(1)                                                    NOT NULL DEFAULT '1' COMMENT 'Order Status: 1: Pending, 2: Paid, 3:Close, 4: Failed, 5:Finished',
    `subscribe_id`    bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Subscribe Id',
    `subscribe_token` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci          DEFAULT NULL COMMENT 'Renewal Subscribe Token',
    `is_new`          tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Is New Order',
    `created_at`      datetime(3)                                                            DEFAULT NULL COMMENT 'Create Time',
    `updated_at`      datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uni_order_order_no` (`order_no`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `payment`
(
    `id`          bigint                                                        NOT NULL AUTO_INCREMENT,
    `name`        varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Payment Name',
    `platform`    varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'Payment Platform',
    `description` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Payment Description',
    `icon`        varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci          DEFAULT '' COMMENT 'Payment Icon',
    `domain`      varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci          DEFAULT '' COMMENT 'Notification Domain',
    `config`      text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci         NOT NULL COMMENT 'Payment Configuration',
    `fee_mode`    tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Fee Mode: 0: No Fee 1: Percentage 2: Fixed Amount 3: Percentage + Fixed Amount',
    `fee_percent` bigint                                                                 DEFAULT '0' COMMENT 'Fee Percentage',
    `fee_amount`  bigint                                                                 DEFAULT '0' COMMENT 'Fixed Fee Amount',
    `enable`      tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Is Enabled',
    `token`       varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci          DEFAULT NULL COMMENT 'Payment Token',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uni_payment_token` (`token`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

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

CREATE TABLE IF NOT EXISTS `server_group`
(
    `id`          bigint                                                        NOT NULL AUTO_INCREMENT,
    `name`        varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Group Name',
    `description` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci          DEFAULT '' COMMENT 'Group Description',
    `created_at`  datetime(3)                                                            DEFAULT NULL COMMENT 'Creation Time',
    `updated_at`  datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

-- if `sms` not exist, create it
CREATE TABLE IF NOT EXISTS `sms`
(
    `id`         bigint    NOT NULL AUTO_INCREMENT,
    `content`    text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,
    `platform`   varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL,
    `area_code`  varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL,
    `telephone`  varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL,
    `status`     tinyint(1)                                                   DEFAULT '1',
    `created_at` timestamp NULL                                               DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `subscribe`
(
    `id`              bigint                                                        NOT NULL AUTO_INCREMENT,
    `name`            varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Subscribe Name',
    `description`     text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Subscribe Description',
    `unit_price`      bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Unit Price',
    `unit_time`       varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Unit Time',
    `discount`        text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Discount',
    `replacement`     bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Replacement',
    `inventory`       bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Inventory',
    `traffic`         bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Traffic',
    `speed_limit`     bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Speed Limit',
    `device_limit`    bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Device Limit',
    `quota`           bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Quota',
    `group_id`        bigint                                                                 DEFAULT NULL COMMENT 'Group Id',
    `server_group`    varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci          DEFAULT NULL COMMENT 'Server Group',
    `server`          varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci          DEFAULT NULL COMMENT 'Server',
    `show`            tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Show portal page',
    `sell`            tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Sell',
    `sort`            bigint                                                        NOT NULL DEFAULT '0' COMMENT 'Sort',
    `deduction_ratio` bigint                                                                 DEFAULT '0' COMMENT 'Deduction Ratio',
    `allow_deduction` tinyint(1)                                                             DEFAULT '1' COMMENT 'Allow deduction',
    `reset_cycle`     bigint                                                                 DEFAULT '0' COMMENT 'Reset Cycle: 0: No Reset, 1: 1st, 2: Monthly, 3: Yearly',
    `renewal_reset`   tinyint(1)                                                             DEFAULT '0' COMMENT 'Renew Reset',
    `created_at`      datetime(3)                                                            DEFAULT NULL COMMENT 'Create Time',
    `updated_at`      datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `subscribe_group`
(
    `id`          bigint                                                        NOT NULL AUTO_INCREMENT,
    `name`        varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Group Name',
    `description` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Group Description',
    `created_at`  datetime(3)                                                            DEFAULT NULL COMMENT 'Create Time',
    `updated_at`  datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `subscribe_type`
(
    `id`         bigint                                                        NOT NULL AUTO_INCREMENT,
    `name`       varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci  NOT NULL DEFAULT '' COMMENT '订阅类型',
    `mark`       varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT '订阅标识',
    `created_at` datetime(3)                                                            DEFAULT NULL COMMENT '创建时间',
    `updated_at` datetime(3)                                                            DEFAULT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `system`
(
    `id`         bigint                                                        NOT NULL AUTO_INCREMENT,
    `category`   varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Category',
    `key`        varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Key Name',
    `value`      text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci         NOT NULL COMMENT 'Key Value',
    `type`       varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci  NOT NULL DEFAULT '' COMMENT 'Type',
    `desc`       text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci         NOT NULL COMMENT 'Description',
    `created_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Creation Time',
    `updated_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uni_system_key` (`key`),
    KEY `index_key` (`key`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `ticket`
(
    `id`          bigint                                                        NOT NULL AUTO_INCREMENT,
    `title`       varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Title',
    `description` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Description',
    `user_id`     bigint                                                        NOT NULL DEFAULT '0' COMMENT 'UserId',
    `status`      tinyint(1)                                                    NOT NULL DEFAULT '1' COMMENT 'Status',
    `created_at`  datetime(3)                                                            DEFAULT NULL COMMENT 'Create Time',
    `updated_at`  datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `ticket_follow`
(
    `id`         bigint                                                        NOT NULL AUTO_INCREMENT,
    `ticket_id`  bigint                                                        NOT NULL DEFAULT '0' COMMENT 'TicketId',
    `from`       varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'From',
    `type`       tinyint(1)                                                    NOT NULL DEFAULT '1' COMMENT 'Type: 1 text, 2 image',
    `content`    text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Content',
    `created_at` datetime(3)                                                            DEFAULT NULL COMMENT 'Create Time',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `traffic_log`
(
    `id`           bigint      NOT NULL AUTO_INCREMENT,
    `server_id`    bigint      NOT NULL COMMENT 'Server ID',
    `user_id`      bigint      NOT NULL COMMENT 'User ID',
    `subscribe_id` bigint      NOT NULL COMMENT 'Subscription ID',
    `download`     bigint               DEFAULT '0' COMMENT 'Download Traffic',
    `upload`       bigint               DEFAULT '0' COMMENT 'Upload Traffic',
    `timestamp`    datetime(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3) COMMENT 'Traffic Log Time',
    PRIMARY KEY (`id`),
    KEY `idx_subscribe_id` (`subscribe_id`),
    KEY `idx_server_id` (`server_id`),
    KEY `idx_user_id` (`user_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `user`
(
    `id`                      bigint                                                        NOT NULL AUTO_INCREMENT,
    `password`                varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'User Password',
    `avatar`                  text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'User Avatar',
    `balance`                 bigint                                                                 DEFAULT '0' COMMENT 'User Balance',
    `telegram`                bigint                                                                 DEFAULT NULL COMMENT 'Telegram Account',
    `refer_code`              varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci           DEFAULT '' COMMENT 'Referral Code',
    `referer_id`              bigint                                                                 DEFAULT NULL COMMENT 'Referrer ID',
    `commission`              bigint                                                                 DEFAULT '0' COMMENT 'Commission',
    `gift_amount`             bigint                                                                 DEFAULT '0' COMMENT 'User Gift Amount',
    `enable`                  tinyint(1)                                                    NOT NULL DEFAULT '1' COMMENT 'Is Account Enabled',
    `is_admin`                tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Is Admin',
    `valid_email`             tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Is Email Verified',
    `enable_email_notify`     tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Enable Email Notifications',
    `enable_telegram_notify`  tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Enable Telegram Notifications',
    `enable_balance_notify`   tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Enable Balance Change Notifications',
    `enable_login_notify`     tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Enable Login Notifications',
    `enable_subscribe_notify` tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Enable Subscription Notifications',
    `enable_trade_notify`     tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Enable Trade Notifications',
    `created_at`              datetime(3)                                                            DEFAULT NULL COMMENT 'Creation Time',
    `updated_at`              datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    `deleted_at`              datetime(3)                                                            DEFAULT NULL COMMENT 'Deletion Time',
    `is_del`                  bigint unsigned                                                        DEFAULT NULL COMMENT '1: Normal 0: Deleted',
    PRIMARY KEY (`id`),
    KEY `idx_referer` (`referer_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `user_auth_methods`
(
    `id`              bigint                                                        NOT NULL AUTO_INCREMENT,
    `user_id`         bigint                                                        NOT NULL COMMENT 'User ID',
    `auth_type`       varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'Auth Type 1: apple 2: google 3: github 4: facebook 5: telegram 6: email 7: phone',
    `auth_identifier` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'Auth Identifier',
    `verified`        tinyint(1)                                                    NOT NULL DEFAULT '0' COMMENT 'Is Verified',
    `created_at`      datetime(3)                                                            DEFAULT NULL COMMENT 'Creation Time',
    `updated_at`      datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_auth_identifier` (`auth_identifier`),
    KEY `idx_user_id` (`user_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

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

CREATE TABLE IF NOT EXISTS `user_device`
(
    `id`           bigint     NOT NULL AUTO_INCREMENT,
    `user_id`      bigint     NOT NULL COMMENT 'User ID',
    `subscribe_id` bigint                                                        DEFAULT NULL COMMENT 'Subscribe ID',
    `ip`           varchar(191) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT 'Device Ip.',
    `Identifier`   varchar(191) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT 'Device Identifier.',
    `user_agent`   varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci  DEFAULT NULL COMMENT 'Device User Agent.',
    `online`       tinyint(1) NOT NULL                                           DEFAULT '0' COMMENT 'Online',
    `enabled`      tinyint(1) NOT NULL                                           DEFAULT '1' COMMENT 'EnableDeviceNumber',
    `created_at`   datetime(3)                                                   DEFAULT NULL COMMENT 'Creation Time',
    `updated_at`   datetime(3)                                                   DEFAULT NULL COMMENT 'Update Time',
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

CREATE TABLE IF NOT EXISTS `user_subscribe`
(
    `id`           bigint      NOT NULL AUTO_INCREMENT,
    `user_id`      bigint      NOT NULL COMMENT 'User ID',
    `order_id`     bigint      NOT NULL COMMENT 'Order ID',
    `subscribe_id` bigint      NOT NULL COMMENT 'Subscription ID',
    `start_time`   datetime(3) NOT NULL                                          DEFAULT CURRENT_TIMESTAMP(3) COMMENT 'Subscription Start Time',
    `expire_time`  datetime(3)                                                   DEFAULT NULL COMMENT 'Subscription Expire Time',
    `traffic`      bigint                                                        DEFAULT '0' COMMENT 'Traffic',
    `download`     bigint                                                        DEFAULT '0' COMMENT 'Download Traffic',
    `upload`       bigint                                                        DEFAULT '0' COMMENT 'Upload Traffic',
    `token`        varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT '' COMMENT 'Token',
    `uuid`         varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT '' COMMENT 'UUID',
    `status`       tinyint(1)                                                    DEFAULT '0' COMMENT 'Subscription Status: 0: Pending 1: Active 2: Finished 3: Expired 4: Deducted',
    `created_at`   datetime(3)                                                   DEFAULT NULL COMMENT 'Creation Time',
    `updated_at`   datetime(3)                                                   DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uni_user_subscribe_token` (`token`),
    UNIQUE KEY `uni_user_subscribe_uuid` (`uuid`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_order_id` (`order_id`),
    KEY `idx_subscribe_id` (`subscribe_id`),
    KEY `idx_token` (`token`),
    KEY `idx_uuid` (`uuid`)
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

CREATE TABLE IF NOT EXISTS `server_rule_group`
(
    `id`          bigint                                                        NOT NULL AUTO_INCREMENT,
    `name`        varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '' COMMENT 'Rule Group Name',
    `icon`        text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Rule Group Icon',
    `tags`        text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT 'Selected Node Tags',
    `description` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci          DEFAULT '' COMMENT 'Rule Group Description',
    `enable`      tinyint(1)                                                    NOT NULL DEFAULT '1' COMMENT 'Rule Group Enable',
    `created_at`  datetime(3)                                                            DEFAULT NULL COMMENT 'Creation Time',
    `updated_at`  datetime(3)                                                            DEFAULT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`),
    UNIQUE KEY `unique_name` (`name`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;


SET FOREIGN_KEY_CHECKS = 1;
