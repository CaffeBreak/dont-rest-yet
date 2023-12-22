-- CreateTable
CREATE TABLE "auth_data" (
    "id" TEXT NOT NULL,
    "google_token" TEXT NOT NULL,
    "discord_id" TEXT NOT NULL,

    CONSTRAINT "auth_data_pkey" PRIMARY KEY ("id")
);
