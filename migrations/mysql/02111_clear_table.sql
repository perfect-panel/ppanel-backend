-- migrate:up
DROP TABLE IF EXISTS `subscribe_type`;
DROP TABLE IF EXISTS `sms`;
-- migrate:down
DROP TABLE IF EXISTS `subscribe_type`;
DROP TABLE IF EXISTS `sms`;
