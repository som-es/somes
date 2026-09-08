-- Minimal schema for the mood barometer routes.
-- Mirrors the dataservice migration `2026-08-31-103616-0000_mood` and keeps only
-- the columns of `ministrial_proposals` that the mood queries use.

CREATE TABLE somes_user (
    id integer PRIMARY KEY
);

CREATE TABLE ministrial_proposals (
    id integer PRIMARY KEY,
    gp text NOT NULL,
    inr integer NOT NULL,
    UNIQUE (gp, inr)
);

CREATE TABLE mood (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    auto_mood double precision NOT NULL DEFAULT 0,
    pre_aggregated_user_mood double precision
);

CREATE TABLE user_mood (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_mood double precision NOT NULL DEFAULT 0,
    user_id integer NOT NULL REFERENCES somes_user(id),
    mood_id BIGINT NOT NULL REFERENCES mood(id),
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    updated_at timestamp with time zone,
    UNIQUE (user_id, mood_id)
);

CREATE TABLE gov_prop_mood (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    gov_prop_id integer NOT NULL REFERENCES ministrial_proposals(id),
    mood_id BIGINT NOT NULL REFERENCES mood(id),
    UNIQUE (gov_prop_id, mood_id)
);

INSERT INTO somes_user (id) VALUES
    (1),
    (2),
    (3);

-- Proposal 1 and 2 have a mood row, proposal 3 does not have one yet
INSERT INTO ministrial_proposals (id, gp, inr) VALUES
    (1, '2024-25', 1),
    (2, '2024-25', 2),
    (3, '2024-25', 3);

-- mood id 1 belongs to proposal 1, mood id 2 to proposal 2
INSERT INTO mood (auto_mood, pre_aggregated_user_mood) VALUES
    (0.7, 0.4),
    (0.1, NULL);

INSERT INTO gov_prop_mood (gov_prop_id, mood_id) VALUES
    (1, 1),
    (2, 2);

-- Proposal 1 is already rated by all three users, proposal 2 by none
INSERT INTO user_mood (user_mood, user_id, mood_id) VALUES
    (0.2, 1, 1),
    (0.4, 2, 1),
    (0.6, 3, 1);
