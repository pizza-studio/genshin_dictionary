ALTER TABLE "dictionary_items"
ADD CONSTRAINT voc_id_lan_uniq UNIQUE ("vocabulary_id", "language");