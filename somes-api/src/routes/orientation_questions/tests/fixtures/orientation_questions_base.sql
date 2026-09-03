CREATE TABLE political_questions (
    id integer PRIMARY KEY,
    question text NOT NULL,
    is_left boolean,
    is_liberal boolean,
    is_part_of text[]
);

CREATE TABLE political_answers (
    id integer PRIMARY KEY,
    question_id integer NOT NULL,
    answer text NOT NULL,
    stance_llm varchar(255) NOT NULL,
    is_strong_reference boolean,
    model_used varchar(255),
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    full_stance jsonb
);

CREATE TABLE political_questions_topics (
    id integer PRIMARY KEY,
    question_id integer NOT NULL,
    topic varchar(255) NOT NULL
);

CREATE TABLE political_questions_topics_influence (
    id integer PRIMARY KEY,
    question_id integer NOT NULL,
    topic varchar(255) NOT NULL,
    influence double precision NOT NULL
);

CREATE TABLE political_questions_detailed_topics (
    id integer PRIMARY KEY,
    question_id integer NOT NULL,
    topic varchar(255) NOT NULL
);

CREATE TABLE political_questions_detailed_topics_influence (
    id integer PRIMARY KEY,
    question_id integer NOT NULL,
    topic varchar(255) NOT NULL,
    influence double precision NOT NULL
);

INSERT INTO political_questions (id, question, is_left, is_liberal, is_part_of) VALUES
    (1, 'Should taxes be raised?', true, false, ARRAY['economy', 'tax']),
    (2, 'Is climate action needed?', false, true, ARRAY['environment']),
    (3, 'No orientation question', NULL, NULL, NULL);

INSERT INTO political_answers (id, question_id, answer, stance_llm, is_strong_reference, model_used, created_at, full_stance) VALUES
    (1, 1, 'Yes, raise taxes', 'positive', true, 'gpt4o', '2024-01-01 00:00:00+00', NULL),
    (2, 1, 'No, keep taxes low', 'negative', true, 'gpt4o', '2024-01-01 00:00:00+00', NULL),
    (3, 1, 'Neutral view', 'neutral', false, 'gpt4o', '2024-01-01 00:00:00+00', NULL),
    (4, 2, 'Yes, urgent action', 'positive', true, 'gpt4o', '2024-01-01 00:00:00+00', NULL);

INSERT INTO political_questions_topics (id, question_id, topic) VALUES
    (1, 1, 'Economy'),
    (2, 1, 'Taxation'),
    (3, 2, 'Environment');

INSERT INTO political_questions_topics_influence (id, question_id, topic, influence) VALUES
    (1, 1, 'Economy', 0.8),
    (2, 1, 'Taxation', 0.6),
    (3, 2, 'Environment', 0.9);

INSERT INTO political_questions_detailed_topics (id, question_id, topic) VALUES
    (1, 1, 'Income Tax'),
    (2, 1, 'Corporate Tax'),
    (3, 2, 'Renewable Energy');

INSERT INTO political_questions_detailed_topics_influence (id, question_id, topic, influence) VALUES
    (1, 1, 'Income Tax', 0.7),
    (2, 1, 'Corporate Tax', 0.5),
    (3, 2, 'Renewable Energy', 0.85);
