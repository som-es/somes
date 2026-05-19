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
    raw_data_created_at timestamp
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
    id integer PRIMARY KEY,
    receiver_id integer,
    plenar_id integer
);

CREATE TABLE legislative_initiatives (
    id integer PRIMARY KEY,
    ityp text,
    gp text,
    nr_plenary_activity_date date,
    raw_data_created_at timestamp,
    created_at timestamp,
    requires_simple_majority boolean,
    accepted text
);

CREATE TABLE legis_init_delegates (
    legis_init_id integer,
    delegate_id integer
);

CREATE TABLE proposals (
    id integer PRIMARY KEY,
    ityp text,
    gp text,
    created_at timestamp
);

CREATE TABLE proposal_delegates (
    proposal_id integer,
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
    id integer PRIMARY KEY,
    party text,
    legislative_initiatives_id integer,
    infavor boolean
);

INSERT INTO delegates (id, name, gender, birthdate, party) VALUES
    (1, 'Delegate A', 'M', '1980-01-01', 'Party X'),
    (2, 'Delegate B', 'F', '1970-01-01', 'Party Y'),
    (3, 'Delegate C', 'M', '1990-01-01', 'Party Z');

INSERT INTO plenar_infos (id, legislative_period, raw_data_created_at) VALUES
    (1, '51', '2020-01-01 00:00:00'),
    (2, '52', '2021-01-01 00:00:00');

INSERT INTO debates (id, plenar_id) VALUES
    (1, 1),
    (2, 2);

INSERT INTO plenar_speeches (id, delegate_id, debate_id, duration_in_seconds) VALUES
    (1, 1, 1, 60),
    (2, 1, 1, 120),
    (3, 1, 1, NULL),
    (4, 2, 1, 240),
    (5, 3, 2, 300);

INSERT INTO mandates (id, delegate_id, party, is_nr, is_gov_official, start_date, end_date) VALUES
    (1, 1, 'Party X', true, false, '2019-01-01', NULL),
    (2, 2, 'Party Y', true, false, '2019-01-01', NULL),
    (3, 3, 'Party Z', true, false, '2019-01-01', NULL);

INSERT INTO absences (id, delegate_id, plenary_session_id) VALUES
    (1, 1, 1),
    (2, 1, 1),
    (3, 2, 1),
    (4, 3, 2);

INSERT INTO call_to_order (id, receiver_id, plenar_id) VALUES
    (1, 1, 1),
    (2, 1, 1),
    (3, 2, 1),
    (4, 3, 2);

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
    (1, 'A', '51', '2020-01-01', '2020-01-01 00:00:00', '2020-01-01 00:00:00', false, 'true'),
    (2, 'I', '51', '2020-01-02', '2020-01-02 00:00:00', '2020-01-02 00:00:00', true, 'false'),
    (3, 'J', '52', '2021-01-01', '2021-01-01 00:00:00', '2021-01-01 00:00:00', false, 'true');

INSERT INTO legis_init_delegates (legis_init_id, delegate_id) VALUES
    (1, 1),
    (2, 1),
    (3, 3);

INSERT INTO proposals (id, ityp, gp, created_at) VALUES
    (1, 'A', '51', '2020-01-01 00:00:00'),
    (2, 'I', '51', '2020-01-02 00:00:00'),
    (3, 'J', '52', '2021-01-01 00:00:00');

INSERT INTO proposal_delegates (proposal_id, delegate_id, is_receiver) VALUES
    (1, 1, false),
    (2, 1, false),
    (3, 3, false),
    (1, 2, true);

INSERT INTO delegate_votes (id, delegate_id, plenar_id, vote, outcome) VALUES
    (1, 1, 1, 'yes', 'yes'),
    (2, 1, 1, 'no', 'yes'),
    (3, 2, 1, 'yes', 'yes'),
    (4, 3, 2, 'yes', 'yes'),
    (5, 1, 1, 'abstain', NULL);

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
    (3, 0.1, 0.9, 0.2, 0.8, 15);

INSERT INTO votes (id, party, legislative_initiatives_id, infavor) VALUES
    (1, 'Party X', 1, true),
    (2, 'Party Y', 1, true),
    (3, 'Party Z', 1, false),
    (4, 'Party X', 2, false),
    (5, 'Party Y', 2, false);
