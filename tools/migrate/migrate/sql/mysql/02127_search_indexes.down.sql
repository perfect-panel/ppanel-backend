SET @index_exists = (SELECT COUNT(1)
                     FROM INFORMATION_SCHEMA.STATISTICS
                     WHERE TABLE_SCHEMA = DATABASE()
                       AND TABLE_NAME = 'nodes'
                       AND INDEX_NAME = 'idx_nodes_port');
SET @sql = IF(@index_exists > 0,
              'ALTER TABLE `nodes` DROP INDEX `idx_nodes_port`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @index_exists = (SELECT COUNT(1)
                     FROM INFORMATION_SCHEMA.STATISTICS
                     WHERE TABLE_SCHEMA = DATABASE()
                       AND TABLE_NAME = 'nodes'
                       AND INDEX_NAME = 'idx_nodes_tags');
SET @sql = IF(@index_exists > 0,
              'ALTER TABLE `nodes` DROP INDEX `idx_nodes_tags`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @index_exists = (SELECT COUNT(1)
                     FROM INFORMATION_SCHEMA.STATISTICS
                     WHERE TABLE_SCHEMA = DATABASE()
                       AND TABLE_NAME = 'nodes'
                       AND INDEX_NAME = 'idx_nodes_address');
SET @sql = IF(@index_exists > 0,
              'ALTER TABLE `nodes` DROP INDEX `idx_nodes_address`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @index_exists = (SELECT COUNT(1)
                     FROM INFORMATION_SCHEMA.STATISTICS
                     WHERE TABLE_SCHEMA = DATABASE()
                       AND TABLE_NAME = 'nodes'
                       AND INDEX_NAME = 'idx_nodes_name');
SET @sql = IF(@index_exists > 0,
              'ALTER TABLE `nodes` DROP INDEX `idx_nodes_name`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @index_exists = (SELECT COUNT(1)
                     FROM INFORMATION_SCHEMA.STATISTICS
                     WHERE TABLE_SCHEMA = DATABASE()
                       AND TABLE_NAME = 'servers'
                       AND INDEX_NAME = 'idx_servers_address');
SET @sql = IF(@index_exists > 0,
              'ALTER TABLE `servers` DROP INDEX `idx_servers_address`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @index_exists = (SELECT COUNT(1)
                     FROM INFORMATION_SCHEMA.STATISTICS
                     WHERE TABLE_SCHEMA = DATABASE()
                       AND TABLE_NAME = 'servers'
                       AND INDEX_NAME = 'idx_servers_name');
SET @sql = IF(@index_exists > 0,
              'ALTER TABLE `servers` DROP INDEX `idx_servers_name`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @index_exists = (SELECT COUNT(1)
                     FROM INFORMATION_SCHEMA.STATISTICS
                     WHERE TABLE_SCHEMA = DATABASE()
                       AND TABLE_NAME = 'payment'
                       AND INDEX_NAME = 'idx_payment_name');
SET @sql = IF(@index_exists > 0,
              'ALTER TABLE `payment` DROP INDEX `idx_payment_name`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @index_exists = (SELECT COUNT(1)
                     FROM INFORMATION_SCHEMA.STATISTICS
                     WHERE TABLE_SCHEMA = DATABASE()
                       AND TABLE_NAME = 'coupon'
                       AND INDEX_NAME = 'idx_coupon_name');
SET @sql = IF(@index_exists > 0,
              'ALTER TABLE `coupon` DROP INDEX `idx_coupon_name`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @index_exists = (SELECT COUNT(1)
                     FROM INFORMATION_SCHEMA.STATISTICS
                     WHERE TABLE_SCHEMA = DATABASE()
                       AND TABLE_NAME = 'user'
                       AND INDEX_NAME = 'idx_user_refer_code');
SET @sql = IF(@index_exists > 0,
              'ALTER TABLE `user` DROP INDEX `idx_user_refer_code`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @index_exists = (SELECT COUNT(1)
                     FROM INFORMATION_SCHEMA.STATISTICS
                     WHERE TABLE_SCHEMA = DATABASE()
                       AND TABLE_NAME = 'order'
                       AND INDEX_NAME = 'idx_order_coupon');
SET @sql = IF(@index_exists > 0,
              'ALTER TABLE `order` DROP INDEX `idx_order_coupon`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET @index_exists = (SELECT COUNT(1)
                     FROM INFORMATION_SCHEMA.STATISTICS
                     WHERE TABLE_SCHEMA = DATABASE()
                       AND TABLE_NAME = 'order'
                       AND INDEX_NAME = 'idx_order_trade_no');
SET @sql = IF(@index_exists > 0,
              'ALTER TABLE `order` DROP INDEX `idx_order_trade_no`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;
