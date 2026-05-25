DELETE FROM round_goals
WHERE
    name IN (
        'Total Birds',
        'Sets of Eggs in All Habitats',
        'Birds in [forest]',
        'Birds in [grassland]',
        'Birds in [wetland]',
        'Eggs in [forest]',
        'Eggs in [grassland]',
        'Eggs in [wetland]',
        'Birds with eggs in [platform]',
        'Birds with eggs in [bowl]',
        'Birds with eggs in [cavity]',
        'Birds with eggs in [ground]',
        'Eggs in [platform]',
        'Eggs in [bowl]',
        'Eggs in [cavity]',
        'Eggs in [ground]'
    );

DELETE FROM sqlite_sequence
WHERE
    name = 'round_goals';