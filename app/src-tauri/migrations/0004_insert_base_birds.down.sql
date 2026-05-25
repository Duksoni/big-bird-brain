DELETE FROM birds
WHERE
    id < 181;

DELETE FROM sqlite_sequence
WHERE
    name = 'birds';