DELETE FROM dictionary_items
WHERE
    vocabulary_id NOT IN (
        SELECT MIN(vocabulary_id)
        FROM (
                SELECT vocabulary_id, STRING_AGG(vocabulary_translation, ', ' ORDER BY language) AS translations
                FROM (
                        SELECT
                            vocabulary_id, vocabulary_translation, language
                        FROM dictionary_items
                    ) AS sorted_items
                GROUP BY
                    vocabulary_id
            ) AS subquery_alias
        GROUP BY
            translations
    );

SELECT DISTINCT
    ON ("vocabulary_id") "vocabulary_id",
    "language" as "language!: Language",
    "vocabulary_translation",
    COUNT(*) OVER () AS "total"
FROM "dictionary_items"
WHERE
    "vocabulary_translation" &@* '可莉'
ORDER BY "vocabulary_id", LENGTH("vocabulary_translation")
LIMIT 20
OFFSET
    0;

SELECT *
FROM (
        SELECT DISTINCT
            ON ("vocabulary_id") "vocabulary_id", "language" as "language!: Language", "vocabulary_translation", COUNT(*) OVER () AS "total"
        FROM "dictionary_items"
        WHERE
            "vocabulary_translation" &@* 'Klee'
        ORDER BY "vocabulary_id"
    ) AS t
ORDER BY LENGTH("vocabulary_translation")
LIMIT 20
OFFSET
    0;

SELECT *
FROM (
        SELECT DISTINCT
            ON ("vocabulary_id") "vocabulary_id", "language" as "language!: Language", "vocabulary_translation", COUNT(*) OVER () AS "total", pgroonga_score(tableoid, ctid) AS "score"
        FROM "dictionary_items"
        WHERE
            "vocabulary_translation" &@~ 'Klee Fish'
        ORDER BY "vocabulary_id"
    ) AS t
ORDER BY LENGTH("vocabulary_translation"), score DESC
LIMIT 20
OFFSET
    0;