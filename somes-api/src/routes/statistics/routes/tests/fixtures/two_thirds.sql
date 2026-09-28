CREATE TABLE topics_legis_init (
    legislative_initiatives_id integer NOT NULL,
    topic text NOT NULL
);
INSERT INTO legislative_initiatives (id, gp, requires_simple_majority, accepted) VALUES
    (9001, 'TEST-I', false, 'a'),
    (9002, 'TEST-I', false, 'd'),
    (9003, 'TEST-I', false, 'p'),
    (9004, 'TEST-II', false, NULL),
    (9005, 'TEST-I', true, 'a'),
    (9006, 'TEST-I', false, 'a'),
    (9007, 'TEST-I', NULL, 'a');
INSERT INTO votes (party, legislative_initiatives_id, infavor_count, against_count, abstention_count, absence_count) VALUES
    ('Test A', 9001, 5, 2, 0, 0),
    ('Test A', 9002, 0, 5, 0, 0),
    ('Test A', 9003, 0, 1, 5, 0),
    ('Test A', 9004, 3, 3, 0, 0),
    ('Test A', 9005, 5, 0, 0, 0),
    ('Test A', 9007, 5, 0, 0, 0),
    ('Absent', 9001, 0, 0, 0, 5);
INSERT INTO topics_legis_init VALUES
    (9001, 'Health'), (9001, ' Health '), (9001, 'Education'),
    (9002, 'Health'), (9003, 'Health'), (9004, 'Health'),
    (9005, 'Health'), (9007, 'Health'), (9006, '');
