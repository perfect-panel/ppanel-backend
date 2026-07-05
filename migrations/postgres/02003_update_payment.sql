-- migrate:up
-- PostgreSQL version of payment/order compatibility migration.
ALTER TABLE "order" ADD COLUMN IF NOT EXISTS "payment_id" BIGINT NOT NULL DEFAULT -1;
ALTER TABLE "payment" ADD COLUMN IF NOT EXISTS "platform" VARCHAR(100) NOT NULL DEFAULT '';
ALTER TABLE "payment" DROP COLUMN IF EXISTS "mark";
ALTER TABLE "payment" ADD COLUMN IF NOT EXISTS "description" TEXT;
ALTER TABLE "payment" ADD COLUMN IF NOT EXISTS "token" VARCHAR(255) DEFAULT NULL;

-- migrate:down
ALTER TABLE "order" DROP COLUMN IF EXISTS "payment_id";
ALTER TABLE "payment" DROP COLUMN IF EXISTS "platform";
ALTER TABLE "payment" DROP COLUMN IF EXISTS "description";
ALTER TABLE "payment" DROP COLUMN IF EXISTS "token";
ALTER TABLE "payment" ADD COLUMN IF NOT EXISTS "mark" VARCHAR(255) DEFAULT NULL;

