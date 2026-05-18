INSERT INTO 
    round_goals (name, condition, explanation) 
VALUES
    ('Total Birds', '{"type": "total_birds"}', 'Count the total number of birds you have played.'),
    ('Sets of Eggs in All Habitats', '{"type": "sets_of_eggs_in_all_habitats"}', 'One set of eggs consists of 1 egg in the wetland row, 1 egg in the grassland row, and 1 egg in the forest row. Count the number of sets.'),
    ('Birds in [forest]', '{"type": "birds_in_habitat", "value": "forest"}', 'Count the total number of birds you currently have in [forest] habitat row.'),
    ('Birds in [grassland]', '{"type": "birds_in_habitat", "value": "grassland"}', 'Count the total number of birds you currently have in [grassland] habitat row.'),
    ('Birds in [wetland]', '{"type": "birds_in_habitat", "value": "wetland"}', 'Count the total number of birds you currently have in [wetland] habitat row.'),
    ('Eggs in [forest]', '{"type": "eggs_in_habitat", "value": "forest"}', 'Count the total number of eggs your birds have laid in [forest] habitat row. Multiple eggs on one bird each count.'),
    ('Eggs in [grassland]', '{"type": "eggs_in_habitat", "value": "grassland"}', 'Count the total number of eggs your birds have laid in [grassland] habitat row. Multiple eggs on one bird each count.'),
    ('Eggs in [wetland]', '{"type": "eggs_in_habitat", "value": "wetland"}', 'Count the total number of eggs your birds have laid in [wetland] habitat row. Multiple eggs on one bird each count.'),
    ('Birds with eggs in [platform]', '{"type": "birds_with_eggs_in_nest", "value": "platform"}', 'Count the total number of birds with [platform] nest type that have at least 1 egg. Each bird counts just once, regardless of how many eggs it has. Star nests count toward this goal.'),
    ('Birds with eggs in [bowl]', '{"type": "birds_with_eggs_in_nest", "value": "cup"}', 'Count the total number of birds with [bowl] nest type that have at least 1 egg. Each bird counts just once, regardless of how many eggs it has. Star nests count toward this goal.'),
    ('Birds with eggs in [cavity]', '{"type": "birds_with_eggs_in_nest", "value": "cavity"}', 'Count the total number of birds with [cavity] nest type that have at least 1 egg. Each bird counts just once, regardless of how many eggs it has. Star nests count toward this goal.'),
    ('Birds with eggs in [ground]', '{"type": "birds_with_eggs_in_nest", "value": "ground"}', 'Count the total number of birds with [ground] nest type that have at least 1 egg. Each bird counts just once, regardless of how many eggs it has. Star nests count toward this goal.'),
    ('Eggs in [platform]', '{"type": "eggs_in_nest", "value": "platform"}', 'Count the total number of eggs on birds with [platform] nest type. Multiple eggs on one bird each count. Star nests count toward this goal.'),
    ('Eggs in [bowl]', '{"type": "eggs_in_nest", "value": "cup"}', 'Count the total number of eggs on birds with [bowl] nest type. Multiple eggs on one bird each count. Star nests count toward this goal.'),
    ('Eggs in [cavity]', '{"type": "eggs_in_nest", "value": "cavity"}', 'Count the total number of eggs on birds with [cavity] nest type. Multiple eggs on one bird each count. Star nests count toward this goal.'),
    ('Eggs in [ground]', '{"type": "eggs_in_nest", "value": "ground"}', 'Count the total number of eggs on birds with [ground] nest type. Multiple eggs on one bird each count. Star nests count toward this goal.');