DELETE FROM bonuses
WHERE
    name IN (
        'Anatomist',
        'Backyard Birder',
        'Bird Counter',
        'Bird Feeder',
        'Breeding Manager',
        'Cartographer',
        'Ecologist',
        'Enclosure Builder',
        'Falconer',
        'Fishery Manager',
        'Food Web Expert',
        'Forester',
        'Historian',
        'Large Bird Specialist',
        'Nest Box Builder',
        'Omnivore Specialist',
        'Oologist',
        'Passerine Specialist',
        'Photographer',
        'Platform Builder',
        'Prairie Manager',
        'Rodentologist',
        'Visionary Leader',
        'Viticulturalist',
        'Wetland Scientist',
        'Wildlife Gardener'
    );

DELETE FROM sqlite_sequence
WHERE
    name = 'bonuses';