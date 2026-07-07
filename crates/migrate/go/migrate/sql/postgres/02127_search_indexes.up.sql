CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX IF NOT EXISTS "idx_order_order_no_pattern" ON "order" ("order_no" text_pattern_ops);
CREATE INDEX IF NOT EXISTS "idx_order_trade_no_pattern" ON "order" ("trade_no" text_pattern_ops);
CREATE INDEX IF NOT EXISTS "idx_order_coupon_pattern" ON "order" ("coupon" text_pattern_ops);

CREATE INDEX IF NOT EXISTS "idx_user_refer_code_pattern" ON "user" ("refer_code" text_pattern_ops);
CREATE INDEX IF NOT EXISTS "idx_user_auth_identifier_pattern" ON "user_auth_methods" ("auth_identifier" text_pattern_ops);

CREATE INDEX IF NOT EXISTS "idx_coupon_name_pattern" ON "coupon" ("name" text_pattern_ops);
CREATE INDEX IF NOT EXISTS "idx_coupon_code_pattern" ON "coupon" ("code" text_pattern_ops);
CREATE INDEX IF NOT EXISTS "idx_payment_name_pattern" ON "payment" ("name" text_pattern_ops);

CREATE INDEX IF NOT EXISTS "idx_servers_name_pattern" ON "servers" ("name" text_pattern_ops);
CREATE INDEX IF NOT EXISTS "idx_servers_address_pattern" ON "servers" ("address" text_pattern_ops);
CREATE INDEX IF NOT EXISTS "idx_nodes_name_pattern" ON "nodes" ("name" text_pattern_ops);
CREATE INDEX IF NOT EXISTS "idx_nodes_address_pattern" ON "nodes" ("address" text_pattern_ops);
CREATE INDEX IF NOT EXISTS "idx_nodes_tags_pattern" ON "nodes" ("tags" text_pattern_ops);
CREATE INDEX IF NOT EXISTS "idx_nodes_port" ON "nodes" ("port");

CREATE INDEX IF NOT EXISTS "idx_ads_title_trgm" ON "ads" USING GIN ("title" gin_trgm_ops);
CREATE INDEX IF NOT EXISTS "idx_ads_content_trgm" ON "ads" USING GIN ("content" gin_trgm_ops);
CREATE INDEX IF NOT EXISTS "idx_announcement_title_trgm" ON "announcement" USING GIN ("title" gin_trgm_ops);
CREATE INDEX IF NOT EXISTS "idx_announcement_content_trgm" ON "announcement" USING GIN ("content" gin_trgm_ops);
CREATE INDEX IF NOT EXISTS "idx_document_title_trgm" ON "document" USING GIN ("title" gin_trgm_ops);
CREATE INDEX IF NOT EXISTS "idx_document_content_trgm" ON "document" USING GIN ("content" gin_trgm_ops);
CREATE INDEX IF NOT EXISTS "idx_subscribe_name_trgm" ON "subscribe" USING GIN ("name" gin_trgm_ops);
CREATE INDEX IF NOT EXISTS "idx_subscribe_description_trgm" ON "subscribe" USING GIN ("description" gin_trgm_ops);
CREATE INDEX IF NOT EXISTS "idx_ticket_title_trgm" ON "ticket" USING GIN ("title" gin_trgm_ops);
CREATE INDEX IF NOT EXISTS "idx_ticket_description_trgm" ON "ticket" USING GIN ("description" gin_trgm_ops);
CREATE INDEX IF NOT EXISTS "idx_system_logs_content_trgm" ON "system_logs" USING GIN ("content" gin_trgm_ops);
