CREATE EXTENSION IF NOT EXISTS pgcrypto;

BEGIN;

CREATE TABLE "event" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "name" text NOT NULL,
    "date" date NULL,
    "time" time NULL,
    "created" timestamptz NOT NULL DEFAULT NOW()
);

INSERT INTO "event" ("name", "date") VALUES ('xmas2019', '2019-12-24');
INSERT INTO "event" ("name", "date") VALUES ('nye2019', '2019-12-31');
INSERT INTO "event" ("name", "date") VALUES ('easter2020', '2020-04-21');
INSERT INTO "event" ("name", "date") VALUES ('xmas2020', '2020-12-24');
INSERT INTO "event" ("name", "date") VALUES ('nye2020', '2020-12-31');

END;
