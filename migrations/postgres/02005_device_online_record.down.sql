DROP TABLE IF EXISTS "user_device_online_record";
ALTER TABLE "user_subscribe" DROP COLUMN IF EXISTS "finished_at";
ALTER TABLE "application_config" DROP COLUMN IF EXISTS "invitation_link";
ALTER TABLE "application_config" DROP COLUMN IF EXISTS "kr_website_id";
