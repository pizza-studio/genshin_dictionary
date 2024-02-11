CREATE TYPE "language" AS ENUM (
    'cht',
    'chs',
    'de',
    'en',
    'es',
    'fr',
    'id',
    'jp',
    'kr',
    'pt',
    'ru',
    'th',
    'vi'
);

CREATE TABLE "dictionary_items" (
  "id" SERIAL NOT NULL PRIMARY KEY,
  "vocabulary_id" BIGINT NOT NULL,
  "language"  language NOT NULL,
  "vocabulary_translation" TEXT NOT NULL DEFAULT ''
);

CREATE EXTENSION IF NOT EXISTS pgroonga;

CREATE INDEX dictionary_items_vocabulary_id_index ON dictionary_items (vocabulary_id);

CREATE INDEX dictionary_items_translation_index ON dictionary_items USING pgroonga (vocabulary_translation);