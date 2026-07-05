DROP INDEX IF EXISTS "idx_system_logs_content_trgm";
DROP INDEX IF EXISTS "idx_ticket_description_trgm";
DROP INDEX IF EXISTS "idx_ticket_title_trgm";
DROP INDEX IF EXISTS "idx_subscribe_description_trgm";
DROP INDEX IF EXISTS "idx_subscribe_name_trgm";
DROP INDEX IF EXISTS "idx_document_content_trgm";
DROP INDEX IF EXISTS "idx_document_title_trgm";
DROP INDEX IF EXISTS "idx_announcement_content_trgm";
DROP INDEX IF EXISTS "idx_announcement_title_trgm";
DROP INDEX IF EXISTS "idx_ads_content_trgm";
DROP INDEX IF EXISTS "idx_ads_title_trgm";

DROP INDEX IF EXISTS "idx_nodes_port";
DROP INDEX IF EXISTS "idx_nodes_tags_pattern";
DROP INDEX IF EXISTS "idx_nodes_address_pattern";
DROP INDEX IF EXISTS "idx_nodes_name_pattern";
DROP INDEX IF EXISTS "idx_servers_address_pattern";
DROP INDEX IF EXISTS "idx_servers_name_pattern";

DROP INDEX IF EXISTS "idx_payment_name_pattern";
DROP INDEX IF EXISTS "idx_coupon_code_pattern";
DROP INDEX IF EXISTS "idx_coupon_name_pattern";

DROP INDEX IF EXISTS "idx_user_auth_identifier_pattern";
DROP INDEX IF EXISTS "idx_user_refer_code_pattern";

DROP INDEX IF EXISTS "idx_order_coupon_pattern";
DROP INDEX IF EXISTS "idx_order_trade_no_pattern";
DROP INDEX IF EXISTS "idx_order_order_no_pattern";
