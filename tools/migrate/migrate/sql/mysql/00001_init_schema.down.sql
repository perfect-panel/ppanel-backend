-- 000001_init_schema.down.sql
SET FOREIGN_KEY_CHECKS = 0;

DROP TABLE IF EXISTS `user_subscribe_log`;
DROP TABLE IF EXISTS `user_subscribe`;
DROP TABLE IF EXISTS `user_login_log`;
DROP TABLE IF EXISTS `user_gift_amount_log`;
DROP TABLE IF EXISTS `user_device`;
DROP TABLE IF EXISTS `user_commission_log`;
DROP TABLE IF EXISTS `user_balance_log`;
DROP TABLE IF EXISTS `user_auth_methods`;
DROP TABLE IF EXISTS `user`;
DROP TABLE IF EXISTS `traffic_log`;
DROP TABLE IF EXISTS `ticket_follow`;
DROP TABLE IF EXISTS `ticket`;
DROP TABLE IF EXISTS `system`;
DROP TABLE IF EXISTS `subscribe_type`;
DROP TABLE IF EXISTS `subscribe_group`;
DROP TABLE IF EXISTS `subscribe`;
DROP TABLE IF EXISTS `sms`;
DROP TABLE IF EXISTS `server_rule_group`;
DROP TABLE IF EXISTS `server_group`;
DROP TABLE IF EXISTS `server`;
DROP TABLE IF EXISTS `payment`;
DROP TABLE IF EXISTS `order`;
DROP TABLE IF EXISTS `message_log`;
DROP TABLE IF EXISTS `document`;
DROP TABLE IF EXISTS `coupon`;
DROP TABLE IF EXISTS `auth_method`;
DROP TABLE IF EXISTS `application_version`;
DROP TABLE IF EXISTS `application_config`;
DROP TABLE IF EXISTS `application`;
DROP TABLE IF EXISTS `announcement`;
DROP TABLE IF EXISTS `ads`;

SET FOREIGN_KEY_CHECKS = 1;