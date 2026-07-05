ALTER TABLE "user"
    ADD COLUMN "referral_percentage" SMALLINT NOT NULL DEFAULT 0,
    ADD COLUMN "only_first_purchase" BOOLEAN NOT NULL DEFAULT true;
