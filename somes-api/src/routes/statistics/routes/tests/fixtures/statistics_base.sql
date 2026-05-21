CREATE TABLE delegates (
    id integer PRIMARY KEY,
    name text NOT NULL,
    gender text,
    birthdate date,
    party text
);

CREATE TABLE plenar_infos (
    id integer PRIMARY KEY,
    legislative_period text,
    raw_data_created_at timestamp with time zone
);

CREATE TABLE debates (
    id integer PRIMARY KEY,
    plenar_id integer
);

CREATE TABLE plenar_speeches (
    id integer PRIMARY KEY,
    delegate_id integer,
    debate_id integer,
    duration_in_seconds integer
);

CREATE TABLE mandates (
    id integer PRIMARY KEY,
    delegate_id integer,
    party text,
    is_nr boolean,
    is_gov_official boolean,
    start_date date,
    end_date date
);

CREATE TABLE absences (
    id integer PRIMARY KEY,
    delegate_id integer,
    plenary_session_id integer
);

CREATE TABLE call_to_order (
    id text PRIMARY KEY,
    receiver_id integer,
    plenar_id integer
);

CREATE TABLE legislative_initiatives (
    id integer PRIMARY KEY,
    ityp text,
    gp text,
    nr_plenary_activity_date date,
    raw_data_created_at timestamp with time zone,
    created_at timestamp with time zone,
    requires_simple_majority boolean,
    accepted text
);

CREATE TABLE legis_init_delegates (
    legis_init_id integer,
    delegate_id integer
);

CREATE TABLE proposals (
    id text PRIMARY KEY,
    ityp text,
    gp text,
    created_at timestamp without time zone
);

CREATE TABLE proposal_delegates (
    proposal_id text,
    delegate_id integer,
    is_receiver boolean
);

CREATE TABLE delegate_votes (
    id integer PRIMARY KEY,
    delegate_id integer,
    plenar_id integer,
    vote text,
    outcome text
);

CREATE TABLE political_positions (
    delegate_id integer PRIMARY KEY,
    is_left double precision,
    is_not_left double precision,
    is_liberal double precision,
    is_not_liberal double precision,
    neutral_count bigint
);

CREATE TABLE votes (
    party text,
    legislative_initiatives_id integer,
    infavor boolean
);

INSERT INTO delegates (id, name, gender, birthdate, party) VALUES
    (1, 'Delegate A', 'M', '1980-01-01', 'Party X'),
    (2, 'Delegate B', 'F', '1970-01-01', 'Party Y'),
    (3, 'Delegate C', 'M', '1990-01-01', 'Party Z'),
    (4, 'Delegate D', 'F', '1985-01-01', 'Party X'),
    (5, 'Delegate E', 'M', '1965-01-01', 'Party Y'),
    (6, 'Delegate F', 'F', '1995-01-01', 'Party Z'),
    (7, 'Delegate G', 'M', '1975-01-01', 'Party X'),
    (8, 'Delegate H', 'F', NULL, 'Party W');

INSERT INTO plenar_infos (id, legislative_period, raw_data_created_at) VALUES
    (1, '51', '2020-01-01 00:00:00+00'),
    (2, '52', '2021-01-01 00:00:00+00'),
    (3, '53', '2022-01-01 00:00:00+00');

INSERT INTO debates (id, plenar_id) VALUES
    (1, 1),
    (2, 2),
    (3, 3);

INSERT INTO plenar_speeches (id, delegate_id, debate_id, duration_in_seconds) VALUES
    (1, 1, 1, 60),
    (2, 1, 1, 120),
    (3, 1, 1, NULL),
    (4, 2, 1, 240),
    (5, 3, 2, 300),
    (6, 4, 1, 100),
    (7, 4, 2, 200),
    (8, 5, 3, 180),
    (9, 6, 2, 90),
    (10, 7, 3, 60),
    (11, 8, 2, 30);

INSERT INTO mandates (id, delegate_id, party, is_nr, is_gov_official, start_date, end_date) VALUES
    (1, 1, 'Party X', true, false, '2019-01-01', NULL),
    (2, 2, 'Party Y', true, false, '2019-01-01', NULL),
    (3, 3, 'Party Z', true, false, '2021-01-01', NULL),
    (4, 4, 'Party X', true, false, '2019-01-01', '2020-12-31'),
    (5, 4, 'Party Y', true, false, '2021-01-01', NULL),
    (6, 5, 'Party Y', true, false, '2022-01-01', NULL),
    (7, 6, 'Party Z', true, false, '2021-01-01', NULL),
    (8, 7, NULL, false, true, '2022-01-01', NULL),
    (9, 8, 'Party W', true, false, '2021-01-01', NULL);

INSERT INTO absences (id, delegate_id, plenary_session_id) VALUES
    (1, 1, 1),
    (2, 1, 1),
    (3, 2, 1),
    (4, 3, 2),
    (5, 4, 1),
    (6, 4, 2),
    (7, 4, 2),
    (8, 5, 3),
    (9, 7, 3),
    (10, 8, 2);

INSERT INTO call_to_order (id, receiver_id, plenar_id) VALUES
    ('cto_a_1', 1, 1),
    ('cto_a_2', 1, 1),
    ('cto_b_1', 2, 1),
    ('cto_c_1', 3, 2),
    ('cto_d_51_1', 4, 1),
    ('cto_d_52_1', 4, 2),
    ('cto_d_52_2', 4, 2),
    ('cto_d_52_3', 4, 2),
    ('cto_e_1', 5, 3),
    ('cto_g_1', 7, 3),
    ('cto_h_1', 8, 2);

INSERT INTO legislative_initiatives (
    id,
    ityp,
    gp,
    nr_plenary_activity_date,
    raw_data_created_at,
    created_at,
    requires_simple_majority,
    accepted
) VALUES
    (1, 'A', '51', '2020-01-01', '2020-01-01 00:00:00+00', '2020-01-01 00:00:00+00', false, 'true'),
    (2, 'I', '51', '2020-01-02', '2020-01-02 00:00:00+00', '2020-01-02 00:00:00+00', true, 'false'),
    (3, 'J', '52', '2021-01-01', '2021-01-01 00:00:00+00', '2021-01-01 00:00:00+00', false, 'true'),
    (4, 'A', '51', '2020-02-01', '2020-02-01 00:00:00+00', '2020-02-01 00:00:00+00', false, 'false'),
    (5, 'J', '52', '2021-02-01', '2021-02-01 00:00:00+00', '2021-02-01 00:00:00+00', false, 'true'),
    (6, 'UEA', '53', '2022-01-01', '2022-01-01 00:00:00+00', '2022-01-01 00:00:00+00', false, 'true'),
    (7, 'AA', '52', '2021-03-01', '2021-03-01 00:00:00+00', '2021-03-01 00:00:00+00', false, 'true'),
    (8, 'I', '53', '2022-02-01', '2022-02-01 00:00:00+00', '2022-02-01 00:00:00+00', false, 'false'),
    (9, 'J', '52', '2021-04-01', '2021-04-01 00:00:00+00', '2021-04-01 00:00:00+00', false, 'true');

INSERT INTO legis_init_delegates (legis_init_id, delegate_id) VALUES
    (1, 1),
    (2, 1),
    (3, 3),
    (4, 4),
    (5, 4),
    (6, 5),
    (7, 6),
    (8, 7),
    (9, 8);

INSERT INTO proposals (id, ityp, gp, created_at) VALUES
    ('p1', 'A', '51', '2020-01-01 00:00:00'),
    ('p2', 'I', '51', '2020-01-02 00:00:00'),
    ('p3', 'J', '52', '2021-01-01 00:00:00'),
    ('p4', 'A', '51', '2020-02-01 00:00:00'),
    ('p5', 'J', '52', '2021-02-01 00:00:00'),
    ('p6', 'UEA', '53', '2022-01-01 00:00:00'),
    ('p7', 'AA', '52', '2021-03-01 00:00:00'),
    ('p8', 'I', '53', '2022-02-01 00:00:00'),
    ('p9', 'J', '52', '2021-04-01 00:00:00');

INSERT INTO proposal_delegates (proposal_id, delegate_id, is_receiver) VALUES
    ('p1', 1, false),
    ('p1', 2, true),
    ('p2', 1, false),
    ('p3', 3, false),
    ('p4', 4, false),
    ('p5', 4, false),
    ('p6', 5, false),
    ('p7', 6, false),
    ('p8', 7, false),
    ('p9', 8, false);

INSERT INTO delegate_votes (id, delegate_id, plenar_id, vote, outcome) VALUES
    (1, 1, 1, 'yes', 'yes'),
    (2, 1, 1, 'no', 'yes'),
    (3, 1, 1, 'abstain', NULL),
    (4, 2, 1, 'yes', 'yes'),
    (5, 3, 2, 'yes', 'yes'),
    (6, 4, 1, 'no', 'yes'),
    (7, 4, 1, 'no', 'yes'),
    (8, 4, 1, 'no', 'yes'),
    (9, 4, 1, 'no', 'yes'),
    (10, 4, 2, 'yes', 'yes'),
    (11, 4, 2, 'no', 'yes'),
    (12, 5, 3, 'yes', 'yes'),
    (13, 6, 2, 'no', 'yes'),
    (14, 7, 3, 'no', 'yes'),
    (15, 8, 2, 'yes', 'yes');

INSERT INTO political_positions (
    delegate_id,
    is_left,
    is_not_left,
    is_liberal,
    is_not_liberal,
    neutral_count
) VALUES
    (1, 0.75, 0.25, 0.6, 0.4, 20),
    (2, 0.2, 0.8, 0.3, 0.7, 10),
    (3, 0.1, 0.9, 0.2, 0.8, 15),
    (4, 0.55, 0.45, 0.8, 0.2, 12),
    (5, 0.4, 0.6, 0.5, 0.5, 8),
    (6, 0.9, 0.1, 0.7, 0.3, 5),
    (7, 0.3, 0.7, 0.2, 0.8, 7),
    (8, 0.65, 0.35, 0.45, 0.55, 6);

INSERT INTO votes (party, legislative_initiatives_id, infavor) VALUES
    ('Party X', 1, true),
    ('Party Y', 1, true),
    ('Party Z', 1, false),
    ('Party X', 2, false),
    ('Party Y', 2, false);
