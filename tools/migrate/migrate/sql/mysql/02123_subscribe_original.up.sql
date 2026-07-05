ALTER TABLE `subscribe`
    ADD COLUMN `show_original_price` TINYINT(1) NOT NULL DEFAULT 0 COMMENT 'display the original price: 0 not display, 1 display'  AFTER `created_at`;
