CREATE TABLE contacts (
    id integer PRIMARY KEY,
    mail varchar(255)
);

CREATE TABLE delegates (
    id integer PRIMARY KEY REFERENCES contacts(id),
    name varchar(255) NOT NULL,
    party varchar(16)
);

CREATE TABLE unique_eurovoc_topics (
    id serial PRIMARY KEY,
    topic_name varchar(255) NOT NULL,
    language text NOT NULL DEFAULT 'de',
    eurovoc_id text,
    eurovoc_type text,
    id_as_hash bigint
);

CREATE TABLE delegate_questions (
    id bigserial PRIMARY KEY,
    user_id integer NOT NULL,
    delegate_id integer NOT NULL REFERENCES delegates(id),
    recipient_email varchar(255) NOT NULL,
    recipient_kind varchar(16) NOT NULL CHECK (recipient_kind IN ('delegate', 'party')),
    recipient_name varchar(255) NOT NULL,
    subject varchar(255) NOT NULL,
    body text NOT NULL,
    outgoing_message_id varchar(998) NOT NULL UNIQUE,
    status varchar(16) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'sending', 'sent', 'failed', 'answered', 'rejected')),
    sent_at timestamp with time zone,
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    updated_at timestamp with time zone NOT NULL DEFAULT now()
);

CREATE TABLE delegate_question_answers (
    id bigserial PRIMARY KEY,
    question_id bigint NOT NULL REFERENCES delegate_questions(id),
    sender_email varchar(255) NOT NULL,
    body text NOT NULL,
    received_at timestamp with time zone NOT NULL DEFAULT now(),
    raw_message_id varchar(998) NOT NULL UNIQUE
);

CREATE TABLE delegate_question_topics (
    id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    question_id bigint NOT NULL REFERENCES delegate_questions(id) ON DELETE CASCADE,
    topic_id bigint NOT NULL,
    UNIQUE (question_id, topic_id)
);

INSERT INTO contacts (id, mail) VALUES
    (1, 'anna@example.at'),
    (2, NULL),
    (3, NULL);

INSERT INTO delegates (id, name, party) VALUES
    (1, 'Anna Alpha', 'OVP'),
    (2, 'Berta Beta', 'GRUE'),
    (3, 'Cleo Gamma', NULL);

INSERT INTO unique_eurovoc_topics (topic_name, language, id_as_hash) VALUES
    ('Ausschussbericht', 'de', 100),
    ('committee report', 'en', 100),
    ('Berufsverband', 'de', 200),
    ('professional association', 'en', 200),
    ('Nur Deutsch', 'de', 300);

INSERT INTO delegate_questions (id, user_id, delegate_id, recipient_email, recipient_kind, recipient_name, subject, body, outgoing_message_id, status, created_at) VALUES
    (1, 1, 1, 'anna@example.at', 'delegate', 'Anna Alpha', 'Frage eins', 'Text eins', 'msg-1@local', 'sent', '2026-01-01 10:00:00+00'),
    (2, 1, 1, 'anna@example.at', 'delegate', 'Anna Alpha', 'Frage zwei', 'Text zwei', 'msg-2@local', 'answered', '2026-01-02 10:00:00+00'),
    (3, 1, 2, 'klub@example.at', 'party', 'Klub', 'Frage drei', 'Text drei', 'msg-3@local', 'pending', '2026-01-03 10:00:00+00'),
    (4, 1, 2, 'klub@example.at', 'party', 'Klub', 'Frage vier', 'Text vier', 'msg-4@local', 'rejected', '2026-01-04 10:00:00+00'),
    (5, 1, 3, 'cleo@example.at', 'delegate', 'Cleo Gamma', 'Frage fünf', 'Text fünf', 'msg-5@local', 'failed', '2026-01-05 10:00:00+00');

ALTER SEQUENCE delegate_questions_id_seq RESTART WITH 100;

INSERT INTO delegate_question_answers (question_id, sender_email, body, received_at, raw_message_id) VALUES
    (2, 'anna@example.at', 'erste Antwort', '2026-01-02 11:00:00+00', 'answer-1@local'),
    (2, 'anna@example.at', 'zweite Antwort', '2026-01-02 12:00:00+00', 'answer-2@local');

INSERT INTO delegate_question_topics (question_id, topic_id) VALUES
    (1, 100),
    (1, 300),
    (2, 200),
    (3, 100),
    (5, 300);
