--
-- PostgreSQL database dump
--

\restrict 22Xa7GDLzHnPbwlhDfvaSkmFFg85IsJ210jIPtz7IyzxYDFdyw1gH7p3TKP82LA

-- Dumped from database version 16.13 (Ubuntu 16.13-0ubuntu0.24.04.1)
-- Dumped by pg_dump version 16.13 (Ubuntu 16.13-0ubuntu0.24.04.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: db_ai_summary; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.db_ai_summary AS (
	id integer,
	full_summary jsonb,
	short_title character varying,
	short_summary character varying,
	detailed_summary character varying,
	complexity_scope_of_proposal character varying,
	model_used character varying,
	version character varying,
	generated_at timestamp with time zone
);


ALTER TYPE public.db_ai_summary OWNER TO fabi;

--
-- Name: db_legislative_initiative_query; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.db_legislative_initiative_query AS (
	id integer,
	ityp text,
	doktyp text,
	gp text,
	inr integer,
	emphasis text,
	ai_emphasis text,
	title text,
	description text,
	accepted text,
	nr_plenary_activity_date date,
	vote_date date,
	raw_data_created_at timestamp with time zone,
	raw_data_updated_at timestamp with time zone,
	created_at timestamp with time zone,
	updated_at timestamp with time zone,
	requires_simple_majority boolean,
	pre_declined_type text,
	voted_by_name boolean,
	plenary_session_id integer,
	is_emphasis_ai_generated boolean,
	is_law boolean,
	law_accepted boolean,
	has_reference boolean,
	is_voteable_on boolean,
	is_urgent boolean,
	voting text,
	hash bigint
);


ALTER TYPE public.db_legislative_initiative_query OWNER TO fabi;

--
-- Name: db_ministrial_proposal_query_meta; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.db_ministrial_proposal_query_meta AS (
	id integer,
	ityp text,
	gp text,
	inr integer,
	emphasis text,
	title text,
	description text,
	raw_data_created_at timestamp with time zone,
	raw_data_updated_at timestamp with time zone,
	created_at timestamp with time zone,
	updated_at timestamp with time zone,
	due_to date,
	ressort text,
	ressort_shortform text,
	legis_init_gp text,
	legis_init_inr integer,
	legis_init_ityp text,
	has_vote_result boolean
);


ALTER TYPE public.db_ministrial_proposal_query_meta OWNER TO fabi;

--
-- Name: db_named_vote; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.db_named_vote AS (
	id integer,
	infavor boolean,
	was_absent boolean,
	lev bigint,
	similiarity_score bigint,
	searched_with text,
	matched_with text,
	delegate_id integer,
	manually_matched boolean
);


ALTER TYPE public.db_named_vote OWNER TO fabi;

--
-- Name: db_named_vote_info; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.db_named_vote_info AS (
	pro_count integer,
	contra_count integer,
	given_vote_sum integer,
	invalid_count integer
);


ALTER TYPE public.db_named_vote_info OWNER TO fabi;

--
-- Name: db_named_votes; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.db_named_votes AS (
	named_vote_info public.db_named_vote_info,
	named_votes public.db_named_vote[]
);


ALTER TYPE public.db_named_votes OWNER TO fabi;

--
-- Name: db_party_named_vote_count; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.db_party_named_vote_count AS (
	party text,
	infavor boolean,
	count bigint
);


ALTER TYPE public.db_party_named_vote_count OWNER TO fabi;

--
-- Name: db_reference; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.db_reference AS (
	gp text,
	ityp text,
	inr integer
);


ALTER TYPE public.db_reference OWNER TO fabi;

--
-- Name: db_related_delegate; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.db_related_delegate AS (
	text text,
	delegate_id integer
);


ALTER TYPE public.db_related_delegate OWNER TO fabi;

--
-- Name: db_speech_with_link; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.db_speech_with_link AS (
	delegate_id integer,
	vote_result_ids integer[],
	infavor boolean,
	duration_in_seconds integer,
	opinion text,
	document_url text,
	about text
);


ALTER TYPE public.db_speech_with_link OWNER TO fabi;

--
-- Name: db_vote; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.db_vote AS (
	party text,
	code text,
	fraction integer,
	infavor boolean
);


ALTER TYPE public.db_vote OWNER TO fabi;

--
-- Name: document; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.document AS (
	title text,
	document_url text,
	document_type text
);


ALTER TYPE public.document OWNER TO fabi;

--
-- Name: full_mandate; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.full_mandate AS (
	start_date date,
	end_date date,
	name text,
	party text,
	is_nr boolean,
	is_gov_official boolean,
	is_ministry boolean,
	is_chancellor boolean,
	function text
);


ALTER TYPE public.full_mandate OWNER TO fabi;

--
-- Name: meilisearch_helper; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.meilisearch_helper AS (
	votes text[],
	issuer_parties text[]
);


ALTER TYPE public.meilisearch_helper OWNER TO fabi;

--
-- Name: optional_db_legislative_initiative_query; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.optional_db_legislative_initiative_query AS (
	id integer,
	ityp text,
	doktyp text,
	gp text,
	inr integer,
	emphasis text,
	title text,
	description text,
	accepted text,
	created_at date,
	appeared_at timestamp without time zone,
	updated_at timestamp without time zone,
	requires_simple_majority boolean,
	pre_declined_type text,
	voted_by_name boolean,
	plenary_session_id integer,
	is_emphasis_ai_generated boolean,
	is_law boolean,
	law_accepted boolean,
	law_come_into_effect_date date,
	law_expires_on_date date,
	by_publication boolean,
	has_reference boolean,
	is_voteable_on boolean,
	is_urgent boolean,
	voting text
);


ALTER TYPE public.optional_db_legislative_initiative_query OWNER TO fabi;

--
-- Name: optional_db_named_vote; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.optional_db_named_vote AS (
	id integer,
	infavor boolean,
	was_absent boolean,
	lev bigint,
	similiarity_score bigint,
	searched_with text,
	matched_with text,
	delegate_id integer,
	manually_matched boolean
);


ALTER TYPE public.optional_db_named_vote OWNER TO fabi;

--
-- Name: optional_db_named_vote_info; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.optional_db_named_vote_info AS (
	pro_count integer,
	contra_count integer,
	given_vote_sum integer,
	invalid_count integer
);


ALTER TYPE public.optional_db_named_vote_info OWNER TO fabi;

--
-- Name: optional_db_named_votes; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.optional_db_named_votes AS (
	named_vote_info public.optional_db_named_vote_info,
	named_votes public.optional_db_named_vote[]
);


ALTER TYPE public.optional_db_named_votes OWNER TO fabi;

--
-- Name: optional_db_party_named_vote_count; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.optional_db_party_named_vote_count AS (
	party text,
	infavor boolean,
	count bigint
);


ALTER TYPE public.optional_db_party_named_vote_count OWNER TO fabi;

--
-- Name: optional_db_reference; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.optional_db_reference AS (
	gp text,
	ityp text,
	inr integer
);


ALTER TYPE public.optional_db_reference OWNER TO fabi;

--
-- Name: optional_db_related_delegate; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.optional_db_related_delegate AS (
	text text,
	delegate_id integer
);


ALTER TYPE public.optional_db_related_delegate OWNER TO fabi;

--
-- Name: optional_db_speech_with_link; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.optional_db_speech_with_link AS (
	delegate_id integer,
	legis_init_id integer,
	infavor boolean,
	duration_in_seconds integer,
	opinion text,
	document_url text,
	about text
);


ALTER TYPE public.optional_db_speech_with_link OWNER TO fabi;

--
-- Name: optional_db_vote; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.optional_db_vote AS (
	party text,
	code text,
	fraction integer,
	infavor boolean
);


ALTER TYPE public.optional_db_vote OWNER TO fabi;

--
-- Name: optional_meilisearch_helper; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.optional_meilisearch_helper AS (
	votes text[]
);


ALTER TYPE public.optional_meilisearch_helper OWNER TO fabi;

--
-- Name: optional_topic; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.optional_topic AS (
	topic text
);


ALTER TYPE public.optional_topic OWNER TO fabi;

--
-- Name: topic; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.topic AS (
	topic text
);


ALTER TYPE public.topic OWNER TO fabi;

--
-- Name: vote_result; Type: TYPE; Schema: public; Owner: fabi
--

CREATE TYPE public.vote_result AS (
	id integer,
	legislative_initiative public.db_legislative_initiative_query,
	votes public.db_vote[],
	speeches public.db_speech_with_link[],
	named_votes public.db_named_votes,
	topics public.topic[],
	eurovoc_topics public.topic[],
	other_keyword_topics public.topic[],
	documents public.document[],
	absences integer[],
	issued_by_dels public.db_related_delegate[],
	referenced_by_others_ids integer[],
	references_to public.db_reference[],
	ai_summary public.db_ai_summary,
	meilisearch_helper public.meilisearch_helper
);


ALTER TYPE public.vote_result OWNER TO fabi;

--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: fabi
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO fabi;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: fabi
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO fabi;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO fabi;

--
-- Name: absences; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.absences (
    id integer NOT NULL,
    lev bigint NOT NULL,
    similiarity_score bigint NOT NULL,
    searched_with character varying(255),
    matched_with character varying(255) NOT NULL,
    delegate_id integer NOT NULL,
    manually_matched boolean DEFAULT false,
    plenary_session_id integer NOT NULL
);


ALTER TABLE public.absences OWNER TO fabi;

--
-- Name: absences_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.absences_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.absences_id_seq OWNER TO fabi;

--
-- Name: absences_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.absences_id_seq OWNED BY public.absences.id;


--
-- Name: answers; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.answers (
    id integer NOT NULL,
    question_id integer NOT NULL,
    delegate_id integer,
    via_mail boolean,
    received_at timestamp without time zone NOT NULL,
    text_type character varying(10),
    mail_subject character varying(1024),
    body text NOT NULL
);


ALTER TABLE public.answers OWNER TO fabi;

--
-- Name: answers_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.answers_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.answers_id_seq OWNER TO fabi;

--
-- Name: answers_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.answers_id_seq OWNED BY public.answers.id;


--
-- Name: call_to_order; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.call_to_order (
    id character varying(20) NOT NULL,
    plenar_id integer NOT NULL,
    inr integer NOT NULL,
    receiver_id integer NOT NULL
);


ALTER TABLE public.call_to_order OWNER TO fabi;

--
-- Name: contacts; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.contacts (
    id integer NOT NULL,
    mail character varying(255),
    phone_number character varying(255),
    twitter_url character varying(255),
    facebook_url character varying(255),
    website_url character varying(255),
    tiktok_url character varying(255),
    instagram_url character varying(255),
    youtube_url character varying(255)
);


ALTER TABLE public.contacts OWNER TO fabi;

--
-- Name: dates; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.dates (
    id integer NOT NULL,
    date date NOT NULL,
    date_sort timestamp with time zone,
    "time" time without time zone,
    title text NOT NULL,
    link text,
    appointment_type text NOT NULL,
    appointment_type_display text,
    info_link text,
    location text,
    committee text NOT NULL,
    year_month text,
    repeated_date date,
    month_year text,
    media_relevant boolean DEFAULT false NOT NULL,
    start timestamp with time zone,
    "end" timestamp with time zone,
    past boolean DEFAULT false NOT NULL,
    day_with_date text,
    tour_format text
);


ALTER TABLE public.dates OWNER TO fabi;

--
-- Name: dates_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.dates_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.dates_id_seq OWNER TO fabi;

--
-- Name: dates_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.dates_id_seq OWNED BY public.dates.id;


--
-- Name: dates_topics; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.dates_topics (
    id integer NOT NULL,
    date_id integer NOT NULL,
    topic text NOT NULL
);


ALTER TABLE public.dates_topics OWNER TO fabi;

--
-- Name: dates_topics_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.dates_topics_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.dates_topics_id_seq OWNER TO fabi;

--
-- Name: dates_topics_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.dates_topics_id_seq OWNED BY public.dates_topics.id;


--
-- Name: debates; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.debates (
    id integer NOT NULL,
    plenar_id integer NOT NULL,
    about character varying(512) NOT NULL
);


ALTER TABLE public.debates OWNER TO fabi;

--
-- Name: debates_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.debates_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.debates_id_seq OWNER TO fabi;

--
-- Name: debates_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.debates_id_seq OWNED BY public.debates.id;


--
-- Name: decree_email_info; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.decree_email_info (
    id integer NOT NULL,
    already_sent_id integer NOT NULL
);


ALTER TABLE public.decree_email_info OWNER TO fabi;

--
-- Name: decree_email_info_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.decree_email_info_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.decree_email_info_id_seq OWNER TO fabi;

--
-- Name: decree_email_info_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.decree_email_info_id_seq OWNED BY public.decree_email_info.id;


--
-- Name: decree_summaries; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.decree_summaries (
    id integer NOT NULL,
    decree_id integer,
    summary_id integer
);


ALTER TABLE public.decree_summaries OWNER TO fabi;

--
-- Name: decree_summaries_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.decree_summaries_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.decree_summaries_id_seq OWNER TO fabi;

--
-- Name: decree_summaries_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.decree_summaries_id_seq OWNED BY public.decree_summaries.id;


--
-- Name: delegate_ages; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.delegate_ages (
    id integer NOT NULL,
    delegate_id integer,
    legislative_period character varying NOT NULL,
    age_at_start integer NOT NULL
);


ALTER TABLE public.delegate_ages OWNER TO fabi;

--
-- Name: delegate_ages_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.delegate_ages_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.delegate_ages_id_seq OWNER TO fabi;

--
-- Name: delegate_ages_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.delegate_ages_id_seq OWNED BY public.delegate_ages.id;


--
-- Name: delegate_matching; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.delegate_matching (
    id integer NOT NULL,
    similiarity_score bigint NOT NULL,
    searched_with character varying(255),
    matched_with character varying(255) NOT NULL,
    delegate_id integer NOT NULL,
    manually_matched boolean DEFAULT false
);


ALTER TABLE public.delegate_matching OWNER TO fabi;

--
-- Name: delegate_matching_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.delegate_matching_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.delegate_matching_id_seq OWNER TO fabi;

--
-- Name: delegate_matching_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.delegate_matching_id_seq OWNED BY public.delegate_matching.id;


--
-- Name: delegates; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.delegates (
    id integer NOT NULL,
    name character varying(255) NOT NULL,
    party character varying(100),
    image_url character varying(300),
    constituency character varying(200),
    council character varying(200),
    seat_row integer,
    seat_col integer,
    gender character varying(1),
    is_active boolean,
    birthdate date,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone,
    prev_name character varying(255),
    prev_name_till date,
    hash bigint,
    image_copyright character varying(255)
);


ALTER TABLE public.delegates OWNER TO fabi;

--
-- Name: delegates_divisions; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.delegates_divisions (
    id integer NOT NULL,
    delegate_id integer NOT NULL,
    division character varying(255) NOT NULL,
    insertion_date date DEFAULT now()
);


ALTER TABLE public.delegates_divisions OWNER TO fabi;

--
-- Name: delegates_divisions_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.delegates_divisions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.delegates_divisions_id_seq OWNER TO fabi;

--
-- Name: delegates_divisions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.delegates_divisions_id_seq OWNED BY public.delegates_divisions.id;


--
-- Name: mandates; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.mandates (
    id integer NOT NULL,
    delegate_id integer NOT NULL,
    name character varying(312) NOT NULL,
    party character varying(100),
    start_date date NOT NULL,
    end_date date,
    is_nr boolean DEFAULT false,
    is_gov_official boolean DEFAULT false,
    function character varying(255),
    is_ministry boolean DEFAULT false,
    is_chancellor boolean DEFAULT false,
    ministry character varying(312)
);


ALTER TABLE public.mandates OWNER TO fabi;

--
-- Name: plenar_infos; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.plenar_infos (
    id integer NOT NULL,
    raw_data_created_at timestamp with time zone NOT NULL,
    inr integer NOT NULL,
    title character varying(255) NOT NULL,
    description character varying(255) NOT NULL,
    legislative_period character varying(9) NOT NULL,
    raw_data_updated_at timestamp with time zone,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone
);


ALTER TABLE public.plenar_infos OWNER TO fabi;

--
-- Name: delegates_with_mandates; Type: MATERIALIZED VIEW; Schema: public; Owner: fabi
--

CREATE MATERIALIZED VIEW public.delegates_with_mandates AS
 WITH period_starts AS (
         SELECT plenar_infos.legislative_period AS gp,
            ((min(plenar_infos.raw_data_created_at) AT TIME ZONE 'Europe/Vienna'::text))::date AS start_date
           FROM public.plenar_infos
          GROUP BY plenar_infos.legislative_period
         HAVING (count(*) > 1)
        ), periods AS (
         SELECT period_starts.gp,
            period_starts.start_date,
            lead(period_starts.start_date) OVER (ORDER BY period_starts.start_date) AS end_date
           FROM period_starts
        )
 SELECT id,
    name,
    party,
    party AS current_party,
    image_url,
    image_copyright,
    constituency,
    council,
    seat_row,
    seat_col,
    gender,
    is_active,
    birthdate,
    created_at,
    updated_at,
    ARRAY( SELECT ROW(m.start_date, m.end_date, (m.name)::text, (m.party)::text, m.is_nr, m.is_gov_official, m.is_ministry, m.is_chancellor, (m.function)::text)::public.full_mandate AS "row"
           FROM public.mandates m
          WHERE ((m.delegate_id = delegates.id) AND (m.end_date IS NULL))) AS "mandates_at_time: Vec<FullMandate>",
    ARRAY( SELECT delegates_divisions.division
           FROM public.delegates_divisions
          WHERE (delegates_divisions.delegate_id = delegates.id)
          ORDER BY delegates_divisions.insertion_date DESC
         LIMIT 1) AS divisions,
    ARRAY( SELECT ROW(m.start_date, m.end_date, (m.name)::text, (m.party)::text, m.is_nr, m.is_gov_official, m.is_ministry, m.is_chancellor, (m.function)::text)::public.full_mandate AS "row"
           FROM public.mandates m
          WHERE (m.delegate_id = delegates.id)) AS "mandates: Vec<FullMandate>",
    ARRAY( SELECT ROW(m.start_date, m.end_date, (m.name)::text, (m.party)::text, m.is_nr, m.is_gov_official, m.is_ministry, m.is_chancellor, (m.function)::text)::public.full_mandate AS "row"
           FROM public.mandates m
          WHERE ((m.delegate_id = delegates.id) AND (m.end_date IS NULL))) AS "active_mandates: Vec<FullMandate>",
    ARRAY( SELECT DISTINCT p.gp
           FROM (public.mandates m
             JOIN periods p ON (((m.start_date <= COALESCE(p.end_date, 'infinity'::date)) AND (COALESCE(m.end_date, 'infinity'::date) >= p.start_date))))
          WHERE (m.delegate_id = delegates.id)) AS "active_gps: Vec<String>",
    ARRAY( SELECT DISTINCT p.gp
           FROM (public.mandates m
             JOIN periods p ON (((m.start_date <= COALESCE(p.end_date, 'infinity'::date)) AND (COALESCE(m.end_date, 'infinity'::date) >= p.start_date))))
          WHERE ((m.delegate_id = delegates.id) AND (m.is_nr = true))) AS "active_nr_gps: Vec<String>",
    ARRAY( SELECT DISTINCT p.gp
           FROM (public.mandates m
             JOIN periods p ON (((m.start_date <= COALESCE(p.end_date, 'infinity'::date)) AND (COALESCE(m.end_date, 'infinity'::date) >= p.start_date))))
          WHERE ((m.delegate_id = delegates.id) AND (m.is_gov_official = true))) AS "active_gov_gps: Vec<String>"
   FROM public.delegates
  WITH NO DATA;


ALTER MATERIALIZED VIEW public.delegates_with_mandates OWNER TO fabi;

--
-- Name: division_interest_score; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.division_interest_score (
    id integer NOT NULL,
    "timestamp" timestamp with time zone DEFAULT now(),
    score real NOT NULL,
    delegate_id integer NOT NULL
);


ALTER TABLE public.division_interest_score OWNER TO fabi;

--
-- Name: division_interest_score_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.division_interest_score_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.division_interest_score_id_seq OWNER TO fabi;

--
-- Name: division_interest_score_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.division_interest_score_id_seq OWNED BY public.division_interest_score.id;


--
-- Name: emphasis_dataset; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.emphasis_dataset (
    id integer NOT NULL,
    instruction character varying(1024) NOT NULL,
    input text NOT NULL,
    output text NOT NULL
);


ALTER TABLE public.emphasis_dataset OWNER TO fabi;

--
-- Name: emphasis_dataset_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.emphasis_dataset_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.emphasis_dataset_id_seq OWNER TO fabi;

--
-- Name: emphasis_dataset_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.emphasis_dataset_id_seq OWNED BY public.emphasis_dataset.id;


--
-- Name: eurovoc_topics_legis_init; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.eurovoc_topics_legis_init (
    id integer NOT NULL,
    topic character varying(255) NOT NULL,
    legislative_initiatives_id integer NOT NULL
);


ALTER TABLE public.eurovoc_topics_legis_init OWNER TO fabi;

--
-- Name: eurovoc_topics_legis_init_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.eurovoc_topics_legis_init_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.eurovoc_topics_legis_init_id_seq OWNER TO fabi;

--
-- Name: eurovoc_topics_legis_init_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.eurovoc_topics_legis_init_id_seq OWNED BY public.eurovoc_topics_legis_init.id;


--
-- Name: eurovoc_topics_ministrial_proposals; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.eurovoc_topics_ministrial_proposals (
    id integer NOT NULL,
    topic character varying(255) NOT NULL,
    ministrial_proposal_id integer NOT NULL
);


ALTER TABLE public.eurovoc_topics_ministrial_proposals OWNER TO fabi;

--
-- Name: eurovoc_topics_ministrial_proposals_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.eurovoc_topics_ministrial_proposals_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.eurovoc_topics_ministrial_proposals_id_seq OWNER TO fabi;

--
-- Name: eurovoc_topics_ministrial_proposals_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.eurovoc_topics_ministrial_proposals_id_seq OWNED BY public.eurovoc_topics_ministrial_proposals.id;


--
-- Name: eurovoc_topics_proposals; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.eurovoc_topics_proposals (
    id integer NOT NULL,
    topic character varying(255) NOT NULL,
    proposals_id character varying(20) NOT NULL
);


ALTER TABLE public.eurovoc_topics_proposals OWNER TO fabi;

--
-- Name: eurovoc_topics_proposals_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.eurovoc_topics_proposals_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.eurovoc_topics_proposals_id_seq OWNER TO fabi;

--
-- Name: eurovoc_topics_proposals_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.eurovoc_topics_proposals_id_seq OWNED BY public.eurovoc_topics_proposals.id;


--
-- Name: events; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.events (
    id integer NOT NULL,
    title text NOT NULL,
    location text NOT NULL,
    event_date date NOT NULL,
    start_time time without time zone NOT NULL,
    description text NOT NULL,
    image text,
    requires_membership boolean DEFAULT false,
    requires_registration boolean DEFAULT false
);


ALTER TABLE public.events OWNER TO fabi;

--
-- Name: events_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.events_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.events_id_seq OWNER TO fabi;

--
-- Name: events_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.events_id_seq OWNED BY public.events.id;


--
-- Name: favo_dels; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.favo_dels (
    id integer NOT NULL,
    user_id integer NOT NULL,
    delegate_id integer NOT NULL,
    favo_on timestamp with time zone DEFAULT now(),
    user_info_days integer DEFAULT 30 NOT NULL
);


ALTER TABLE public.favo_dels OWNER TO fabi;

--
-- Name: favo_dels_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.favo_dels_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.favo_dels_id_seq OWNER TO fabi;

--
-- Name: favo_dels_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.favo_dels_id_seq OWNED BY public.favo_dels.id;


--
-- Name: favo_legis_inits; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.favo_legis_inits (
    id integer NOT NULL,
    user_id integer NOT NULL,
    legis_init_id integer NOT NULL,
    favo_on timestamp with time zone DEFAULT now()
);


ALTER TABLE public.favo_legis_inits OWNER TO fabi;

--
-- Name: favo_legis_inits_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.favo_legis_inits_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.favo_legis_inits_id_seq OWNER TO fabi;

--
-- Name: favo_legis_inits_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.favo_legis_inits_id_seq OWNED BY public.favo_legis_inits.id;


--
-- Name: generated_eurovoc_topics; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.generated_eurovoc_topics (
    id integer NOT NULL,
    summary_id integer,
    topic text NOT NULL
);


ALTER TABLE public.generated_eurovoc_topics OWNER TO fabi;

--
-- Name: generated_eurovoc_topics_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.generated_eurovoc_topics_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.generated_eurovoc_topics_id_seq OWNER TO fabi;

--
-- Name: generated_eurovoc_topics_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.generated_eurovoc_topics_id_seq OWNED BY public.generated_eurovoc_topics.id;


--
-- Name: legis_init_delegates; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.legis_init_delegates (
    id integer NOT NULL,
    legis_init_id integer NOT NULL,
    delegate_id integer NOT NULL,
    delegate_text character varying(255)
);


ALTER TABLE public.legis_init_delegates OWNER TO fabi;

--
-- Name: legis_inits_refs; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.legis_inits_refs (
    id integer NOT NULL,
    origin_legis_init_id integer NOT NULL,
    ref_gp character varying(255) NOT NULL,
    ref_ityp character varying(255) NOT NULL,
    ref_inr integer NOT NULL
);


ALTER TABLE public.legis_inits_refs OWNER TO fabi;

--
-- Name: legislative_documents; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.legislative_documents (
    id integer NOT NULL,
    legislative_initiatives_id integer NOT NULL,
    title character varying(1024),
    document_type character varying(100) NOT NULL,
    document_url character varying(500) NOT NULL
);


ALTER TABLE public.legislative_documents OWNER TO fabi;

--
-- Name: legislative_initiative_summaries; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.legislative_initiative_summaries (
    id integer NOT NULL,
    legis_init_id integer,
    summary_id integer
);


ALTER TABLE public.legislative_initiative_summaries OWNER TO fabi;

--
-- Name: legislative_initiatives; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.legislative_initiatives (
    id integer NOT NULL,
    ityp character varying(7) NOT NULL,
    gp character varying(7) NOT NULL,
    inr integer NOT NULL,
    emphasis character varying(16000),
    title text NOT NULL,
    description text NOT NULL,
    accepted character varying(10),
    nr_plenary_activity_date date DEFAULT now() NOT NULL,
    raw_data_created_at timestamp with time zone NOT NULL,
    raw_data_updated_at timestamp with time zone,
    requires_simple_majority boolean DEFAULT true NOT NULL,
    voted_by_name boolean DEFAULT false NOT NULL,
    pre_declined_type character varying(10),
    plenary_session_id integer,
    doktyp character varying(20) NOT NULL,
    is_emphasis_ai_generated boolean DEFAULT false NOT NULL,
    is_law boolean DEFAULT false NOT NULL,
    law_accepted boolean,
    has_reference boolean DEFAULT false,
    is_voteable_on boolean DEFAULT false NOT NULL,
    is_urgent boolean DEFAULT false NOT NULL,
    voting character varying(255),
    ai_emphasis character varying(16000),
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    vote_date date,
    hash bigint
);


ALTER TABLE public.legislative_initiatives OWNER TO fabi;

--
-- Name: ministerial_proposal_summaries; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.ministerial_proposal_summaries (
    id integer NOT NULL,
    ministerial_proposal_id integer,
    summary_id integer
);


ALTER TABLE public.ministerial_proposal_summaries OWNER TO fabi;

--
-- Name: ministrial_issuer; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.ministrial_issuer (
    id integer NOT NULL,
    delegate_id integer NOT NULL,
    ministrial_proposal_id integer NOT NULL
);


ALTER TABLE public.ministrial_issuer OWNER TO fabi;

--
-- Name: ministrial_proposals; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.ministrial_proposals (
    id integer NOT NULL,
    ityp character varying(7) NOT NULL,
    gp character varying(14) NOT NULL,
    inr integer NOT NULL,
    emphasis character varying(6000),
    title text NOT NULL,
    description text NOT NULL,
    raw_data_created_at timestamp with time zone NOT NULL,
    raw_data_updated_at timestamp with time zone,
    due_to date NOT NULL,
    ressort character varying(255) NOT NULL,
    ressort_shortform character varying(12) NOT NULL,
    legis_init_gp character varying(14),
    legis_init_inr integer,
    legis_init_ityp character varying(7),
    has_vote_result boolean DEFAULT false NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone,
    hash bigint
);


ALTER TABLE public.ministrial_proposals OWNER TO fabi;

--
-- Name: ministrial_proposals_documents; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.ministrial_proposals_documents (
    id integer NOT NULL,
    ministrial_proposal_id integer NOT NULL,
    title character varying(1024) NOT NULL,
    document_type character varying(100) NOT NULL,
    document_url character varying(500) NOT NULL
);


ALTER TABLE public.ministrial_proposals_documents OWNER TO fabi;

--
-- Name: named_vote_info; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.named_vote_info (
    id integer NOT NULL,
    legis_init_id integer NOT NULL,
    pro_count integer NOT NULL,
    contra_count integer NOT NULL,
    given_vote_sum integer NOT NULL,
    invalid_count integer NOT NULL
);


ALTER TABLE public.named_vote_info OWNER TO fabi;

--
-- Name: named_votes; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.named_votes (
    id integer NOT NULL,
    infavor boolean,
    was_absent boolean DEFAULT false,
    lev bigint NOT NULL,
    similiarity_score bigint NOT NULL,
    searched_with character varying(255),
    matched_with character varying(255) NOT NULL,
    delegate_id integer NOT NULL,
    named_vote_info_id integer NOT NULL,
    manually_matched boolean DEFAULT false
);


ALTER TABLE public.named_votes OWNER TO fabi;

--
-- Name: other_keyword_topics_legis_init; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.other_keyword_topics_legis_init (
    id integer NOT NULL,
    topic character varying(255) NOT NULL,
    legislative_initiatives_id integer NOT NULL
);


ALTER TABLE public.other_keyword_topics_legis_init OWNER TO fabi;

--
-- Name: other_keyword_topics_ministrial_proposals; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.other_keyword_topics_ministrial_proposals (
    id integer NOT NULL,
    topic character varying(255) NOT NULL,
    ministrial_proposal_id integer NOT NULL
);


ALTER TABLE public.other_keyword_topics_ministrial_proposals OWNER TO fabi;

--
-- Name: plenar_speech_legis_inits; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.plenar_speech_legis_inits (
    id integer NOT NULL,
    gp character varying(255) NOT NULL,
    ityp character varying(255) NOT NULL,
    inr integer NOT NULL,
    speech_id integer NOT NULL,
    legis_init_id integer
);


ALTER TABLE public.plenar_speech_legis_inits OWNER TO fabi;

--
-- Name: plenar_speech_links; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.plenar_speech_links (
    id integer NOT NULL,
    plenar_speech_id integer NOT NULL,
    site_start integer,
    site_end integer,
    document_url character varying(500)
);


ALTER TABLE public.plenar_speech_links OWNER TO fabi;

--
-- Name: plenar_speeches; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.plenar_speeches (
    id integer NOT NULL,
    delegate_id integer NOT NULL,
    debate_id integer NOT NULL,
    opinion character varying(255),
    spoken_text text,
    duration_in_seconds integer NOT NULL,
    ty character varying(40) NOT NULL,
    delegate_display_name character varying(255) NOT NULL,
    summarized_spoken_text text
);


ALTER TABLE public.plenar_speeches OWNER TO fabi;

--
-- Name: summaries; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.summaries (
    id integer NOT NULL,
    full_summary jsonb NOT NULL,
    short_summary text NOT NULL,
    detailed_summary text NOT NULL,
    complexity_scope_of_proposal text NOT NULL,
    model_used text NOT NULL,
    version text NOT NULL,
    generated_at timestamp with time zone DEFAULT now() NOT NULL,
    short_title text NOT NULL
);


ALTER TABLE public.summaries OWNER TO fabi;

--
-- Name: topics_legis_init; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.topics_legis_init (
    id integer NOT NULL,
    topic character varying(100) NOT NULL,
    legislative_initiatives_id integer NOT NULL
);


ALTER TABLE public.topics_legis_init OWNER TO fabi;

--
-- Name: topics_ministrial_proposals; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.topics_ministrial_proposals (
    id integer NOT NULL,
    topic character varying(255) NOT NULL,
    ministrial_proposal_id integer NOT NULL
);


ALTER TABLE public.topics_ministrial_proposals OWNER TO fabi;

--
-- Name: votes; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.votes (
    party character varying(30) NOT NULL,
    fraction integer NOT NULL,
    infavor boolean NOT NULL,
    legislative_initiatives_id integer NOT NULL
);


ALTER TABLE public.votes OWNER TO fabi;

--
-- Name: vote_results; Type: VIEW; Schema: public; Owner: fabi
--

CREATE VIEW public.vote_results AS
 SELECT id,
    ( SELECT ROW(legislative_initiatives.id, (legislative_initiatives.ityp)::text, (legislative_initiatives.doktyp)::text, (legislative_initiatives.gp)::text, legislative_initiatives.inr, (legislative_initiatives.emphasis)::text, (legislative_initiatives.ai_emphasis)::text, legislative_initiatives.title, legislative_initiatives.description, (legislative_initiatives.accepted)::text, legislative_initiatives.nr_plenary_activity_date, legislative_initiatives.vote_date, legislative_initiatives.raw_data_created_at, legislative_initiatives.raw_data_updated_at, legislative_initiatives.created_at, legislative_initiatives.updated_at, legislative_initiatives.requires_simple_majority, (legislative_initiatives.pre_declined_type)::text, legislative_initiatives.voted_by_name, legislative_initiatives.plenary_session_id, legislative_initiatives.is_emphasis_ai_generated, legislative_initiatives.is_law, legislative_initiatives.law_accepted, legislative_initiatives.has_reference, legislative_initiatives.is_voteable_on, legislative_initiatives.is_urgent, (legislative_initiatives.voting)::text, legislative_initiatives.hash)::public.db_legislative_initiative_query AS "row"
           FROM public.legislative_initiatives
          WHERE (legislative_initiatives.id = li.id)) AS "legislative_initiative: DbLegislativeInitiativeQuery",
        CASE
            WHEN (EXISTS ( SELECT 1
               FROM public.named_vote_info nvi
              WHERE (nvi.legis_init_id = li.id))) THEN ARRAY( SELECT ROW((m.party)::text, NULL::text, (count(*))::integer, nv.infavor)::public.db_vote AS "row"
               FROM ((public.named_vote_info nvi
                 JOIN public.named_votes nv ON ((nvi.id = nv.named_vote_info_id)))
                 JOIN public.mandates m ON ((m.delegate_id = nv.delegate_id)))
              WHERE ((nvi.legis_init_id = li.id) AND m.is_nr AND (m.start_date <= li.nr_plenary_activity_date) AND (COALESCE(m.end_date, li.nr_plenary_activity_date) >= li.nr_plenary_activity_date) AND (nv.infavor IS NOT NULL) AND (m.party IS NOT NULL))
              GROUP BY m.party, nv.infavor)
            ELSE ARRAY( SELECT ROW((v.party)::text, NULL::text, v.fraction, v.infavor)::public.db_vote AS "row"
               FROM public.votes v
              WHERE (v.legislative_initiatives_id = li.id))
        END AS "votes: Vec<DbVote>",
    ARRAY( SELECT ROW(ps.delegate_id, array_remove(ARRAY( SELECT plenar_speech_legis_inits.legis_init_id
                   FROM public.plenar_speech_legis_inits
                  WHERE (plenar_speech_legis_inits.speech_id = ps.id)), NULL::integer),
                CASE
                    WHEN ((ps.opinion)::text = 'Pro'::text) THEN ((li.pre_declined_type)::text !~~ '%p%'::text)
                    WHEN ((ps.opinion)::text = 'Contra'::text) THEN ((li.pre_declined_type)::text ~~ '%p%'::text)
                    ELSE NULL::boolean
                END, ps.duration_in_seconds, (ps.opinion)::text, (psl.document_url)::text, (deb.about)::text)::public.db_speech_with_link AS "row"
           FROM (((public.plenar_speeches ps
             JOIN public.plenar_speech_links psl ON ((psl.plenar_speech_id = ps.id)))
             JOIN public.plenar_speech_legis_inits pl ON ((pl.speech_id = ps.id)))
             JOIN public.debates deb ON ((deb.id = ps.debate_id)))
          WHERE (pl.legis_init_id = li.id)) AS "speeches: Vec<DbSpeechWithLink>",
    ( SELECT ROW(ROW(nvi.pro_count, nvi.contra_count, nvi.given_vote_sum, nvi.invalid_count)::public.db_named_vote_info, ARRAY( SELECT ROW(nv.id, nv.infavor, nv.was_absent, nv.lev, nv.similiarity_score, (nv.searched_with)::text, (nv.matched_with)::text, nv.delegate_id, nv.manually_matched)::public.db_named_vote AS "row"
                   FROM public.named_votes nv
                  WHERE (nv.named_vote_info_id = nvi.id)))::public.db_named_votes AS "row"
           FROM public.named_vote_info nvi
          WHERE ((nvi.legis_init_id = li.id) AND li.voted_by_name)
         LIMIT 1) AS "named_votes: DbNamedVotes",
    ARRAY( SELECT ROW((topics_legis_init.topic)::text)::public.topic AS "row"
           FROM public.topics_legis_init
          WHERE (topics_legis_init.legislative_initiatives_id = li.id)) AS "topics: Vec<Topic>",
    ARRAY( SELECT ROW((eurovoc_topics_legis_init.topic)::text)::public.topic AS "row"
           FROM public.eurovoc_topics_legis_init
          WHERE (eurovoc_topics_legis_init.legislative_initiatives_id = li.id)) AS "eurovoc_topics: Vec<Topic>",
    ARRAY( SELECT ROW((other_keyword_topics_legis_init.topic)::text)::public.topic AS "row"
           FROM public.other_keyword_topics_legis_init
          WHERE (other_keyword_topics_legis_init.legislative_initiatives_id = li.id)) AS "other_keyword_topics: Vec<Topic>",
    ARRAY( SELECT ROW((legislative_documents.title)::text, (legislative_documents.document_url)::text, (legislative_documents.document_type)::text)::public.document AS "row"
           FROM public.legislative_documents
          WHERE (legislative_documents.legislative_initiatives_id = li.id)) AS "documents: Vec<Document>",
    ( SELECT ARRAY( SELECT a.delegate_id
                   FROM public.absences a
                  WHERE (a.plenary_session_id = li.plenary_session_id)) AS "array") AS "absences: Vec<i32>",
    ARRAY( SELECT ROW((lid.delegate_text)::text, lid.delegate_id)::public.db_related_delegate AS "row"
           FROM public.legis_init_delegates lid
          WHERE (lid.legis_init_id = li.id)) AS "issued_by_dels: Vec<DbRelatedDelegate>",
    ( SELECT ARRAY( SELECT lir.origin_legis_init_id
                   FROM (public.legis_inits_refs lir
                     JOIN public.legislative_initiatives li2 ON ((li2.id = lir.origin_legis_init_id)))
                  WHERE (((lir.ref_gp)::text = (li.gp)::text) AND ((lir.ref_ityp)::text = (li.ityp)::text) AND (lir.ref_inr = li.inr) AND li2.is_voteable_on)) AS "array") AS "referenced_by_others_ids: Vec<i32>",
    ( SELECT ARRAY( SELECT ROW((lir.ref_gp)::text, (lir.ref_ityp)::text, lir.ref_inr)::public.db_reference AS "row"
                   FROM (public.legis_inits_refs lir
                     JOIN public.legislative_initiatives li2 ON ((((li2.gp)::text = (lir.ref_gp)::text) AND ((li2.ityp)::text = (lir.ref_ityp)::text) AND (li2.inr = lir.ref_inr))))
                  WHERE ((lir.origin_legis_init_id = li.id) AND li2.is_voteable_on)) AS "array") AS "references: Vec<DbReference>",
    ( SELECT ROW(s.id, s.full_summary, (s.short_title)::character varying, (s.short_summary)::character varying, (s.detailed_summary)::character varying, (s.complexity_scope_of_proposal)::character varying, (s.model_used)::character varying, (s.version)::character varying, s.generated_at)::public.db_ai_summary AS "row"
           FROM (public.legislative_initiative_summaries lis
             JOIN public.summaries s ON ((s.id = lis.summary_id)))
          WHERE (lis.legis_init_id = li.id)
          ORDER BY s.generated_at DESC
         LIMIT 1) AS "ai_summary: DbAiSummary",
    ( SELECT ROW(ARRAY[]::text[], (ARRAY( SELECT DISTINCT m.party
                   FROM (public.legis_init_delegates lid
                     JOIN public.mandates m ON ((m.delegate_id = lid.delegate_id)))
                  WHERE ((lid.legis_init_id = li.id) AND m.is_nr AND (m.start_date <= li.nr_plenary_activity_date) AND (COALESCE(m.end_date, li.nr_plenary_activity_date) >= li.nr_plenary_activity_date) AND (m.party IS NOT NULL))))::text[])::public.meilisearch_helper AS "row") AS "meilisearch_helper: MeilisearchHelper"
   FROM public.legislative_initiatives li;


ALTER VIEW public.vote_results OWNER TO fabi;

--
-- Name: gov_proposals; Type: VIEW; Schema: public; Owner: fabi
--

CREATE VIEW public.gov_proposals AS
 SELECT id,
    ( SELECT ROW(inner_mp.id, (inner_mp.ityp)::text, (inner_mp.gp)::text, inner_mp.inr, (inner_mp.emphasis)::text, inner_mp.title, inner_mp.description, inner_mp.raw_data_created_at, inner_mp.raw_data_updated_at, inner_mp.created_at, inner_mp.updated_at, inner_mp.due_to, (inner_mp.ressort)::text, (inner_mp.ressort_shortform)::text, (inner_mp.legis_init_gp)::text, inner_mp.legis_init_inr, (inner_mp.legis_init_ityp)::text, inner_mp.has_vote_result)::public.db_ministrial_proposal_query_meta AS "row"
           FROM public.ministrial_proposals inner_mp
          WHERE (inner_mp.id = mp.id)) AS "ministrial_proposal: DbMinistrialProposalQueryMeta",
    ( SELECT ROW(vr.id, vr."legislative_initiative: DbLegislativeInitiativeQuery", vr."votes: Vec<DbVote>", vr."speeches: Vec<DbSpeechWithLink>", vr."named_votes: DbNamedVotes", vr."topics: Vec<Topic>", vr."eurovoc_topics: Vec<Topic>", vr."other_keyword_topics: Vec<Topic>", vr."documents: Vec<Document>", vr."absences: Vec<i32>", vr."issued_by_dels: Vec<DbRelatedDelegate>", vr."referenced_by_others_ids: Vec<i32>", vr."references: Vec<DbReference>", vr."ai_summary: DbAiSummary", vr."meilisearch_helper: MeilisearchHelper")::public.vote_result AS "row"
           FROM public.vote_results vr
          WHERE (vr.id = ( SELECT li.id
                   FROM public.legislative_initiatives li
                  WHERE (((li.gp)::text = (mp.legis_init_gp)::text) AND (li.inr = mp.legis_init_inr) AND ((li.ityp)::text = (mp.legis_init_ityp)::text))))) AS "vote_result: OptionalVoteResult",
    ARRAY( SELECT ROW((topics_ministrial_proposals.topic)::text)::public.topic AS "row"
           FROM public.topics_ministrial_proposals
          WHERE (topics_ministrial_proposals.ministrial_proposal_id = mp.id)) AS "topics: Vec<Topic>",
    ARRAY( SELECT ROW((eurovoc_topics_ministrial_proposals.topic)::text)::public.topic AS "row"
           FROM public.eurovoc_topics_ministrial_proposals
          WHERE (eurovoc_topics_ministrial_proposals.ministrial_proposal_id = mp.id)) AS "eurovoc_topics: Vec<Topic>",
    ARRAY( SELECT ROW((other_keyword_topics_ministrial_proposals.topic)::text)::public.topic AS "row"
           FROM public.other_keyword_topics_ministrial_proposals
          WHERE (other_keyword_topics_ministrial_proposals.ministrial_proposal_id = mp.id)) AS "other_keyword_topics: Vec<Topic>",
    ARRAY( SELECT ROW((ministrial_proposals_documents.title)::text, (ministrial_proposals_documents.document_url)::text, (ministrial_proposals_documents.document_type)::text)::public.document AS "row"
           FROM public.ministrial_proposals_documents
          WHERE (ministrial_proposals_documents.ministrial_proposal_id = mp.id)) AS "documents: Vec<Document>",
    ( SELECT ARRAY( SELECT ministrial_issuer.delegate_id
                   FROM public.ministrial_issuer
                  WHERE (ministrial_issuer.ministrial_proposal_id = mp.id)) AS "array") AS "ministerial_issuers: Vec<i32>",
    ( SELECT ROW(s.id, s.full_summary, (s.short_title)::character varying, (s.short_summary)::character varying, (s.detailed_summary)::character varying, (s.complexity_scope_of_proposal)::character varying, (s.model_used)::character varying, (s.version)::character varying, s.generated_at)::public.db_ai_summary AS "row"
           FROM (public.ministerial_proposal_summaries mps
             JOIN public.summaries s ON ((s.id = mps.summary_id)))
          WHERE (mps.ministerial_proposal_id = mp.id)
          ORDER BY s.generated_at DESC
         LIMIT 1) AS "ai_summary: DbAiSummary"
   FROM public.ministrial_proposals mp;


ALTER VIEW public.gov_proposals OWNER TO fabi;

--
-- Name: interjections; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.interjections (
    id integer NOT NULL,
    interjection_text text,
    interjector_delegate_id integer NOT NULL,
    plenar_speech_id integer NOT NULL,
    delegate_matching_id integer NOT NULL,
    rel_start_idx integer NOT NULL,
    rel_end_idx integer NOT NULL
);


ALTER TABLE public.interjections OWNER TO fabi;

--
-- Name: interjections_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.interjections_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.interjections_id_seq OWNER TO fabi;

--
-- Name: interjections_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.interjections_id_seq OWNED BY public.interjections.id;


--
-- Name: introduction_transcriptions; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.introduction_transcriptions (
    id integer NOT NULL,
    delegate_id integer
);


ALTER TABLE public.introduction_transcriptions OWNER TO fabi;

--
-- Name: introduction_transcriptions_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.introduction_transcriptions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.introduction_transcriptions_id_seq OWNER TO fabi;

--
-- Name: introduction_transcriptions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.introduction_transcriptions_id_seq OWNED BY public.introduction_transcriptions.id;


--
-- Name: last_vector_speech_update; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.last_vector_speech_update (
    id integer NOT NULL,
    delegate_id integer NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.last_vector_speech_update OWNER TO fabi;

--
-- Name: last_vector_speech_update_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.last_vector_speech_update_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.last_vector_speech_update_id_seq OWNER TO fabi;

--
-- Name: last_vector_speech_update_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.last_vector_speech_update_id_seq OWNED BY public.last_vector_speech_update.id;


--
-- Name: latest_legislative_initiatives; Type: MATERIALIZED VIEW; Schema: public; Owner: fabi
--

CREATE MATERIALIZED VIEW public.latest_legislative_initiatives AS
 SELECT id,
    ityp,
    gp,
    inr,
    emphasis,
    title,
    description,
    accepted,
    nr_plenary_activity_date,
    raw_data_created_at,
    raw_data_updated_at,
    requires_simple_majority,
    voted_by_name,
    pre_declined_type,
    plenary_session_id,
    doktyp,
    is_emphasis_ai_generated,
    is_law,
    law_accepted,
    has_reference,
    is_voteable_on,
    is_urgent,
    voting,
    ai_emphasis,
    created_at,
    updated_at,
    vote_date,
    hash
   FROM public.legislative_initiatives
  WHERE ((nr_plenary_activity_date = ( SELECT max(legislative_initiatives_1.nr_plenary_activity_date) AS max
           FROM public.legislative_initiatives legislative_initiatives_1
          WHERE (legislative_initiatives_1.accepted IS NOT NULL))) AND (accepted IS NOT NULL) AND is_voteable_on)
  WITH NO DATA;


ALTER MATERIALIZED VIEW public.latest_legislative_initiatives OWNER TO fabi;

--
-- Name: legis_email_info; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.legis_email_info (
    id integer NOT NULL,
    already_sent_id integer NOT NULL,
    marked_on timestamp with time zone DEFAULT now()
);


ALTER TABLE public.legis_email_info OWNER TO fabi;

--
-- Name: legis_email_info_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.legis_email_info_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.legis_email_info_id_seq OWNER TO fabi;

--
-- Name: legis_email_info_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.legis_email_info_id_seq OWNED BY public.legis_email_info.id;


--
-- Name: legis_init_delegates_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.legis_init_delegates_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.legis_init_delegates_id_seq OWNER TO fabi;

--
-- Name: legis_init_delegates_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.legis_init_delegates_id_seq OWNED BY public.legis_init_delegates.id;


--
-- Name: legis_init_was_updated; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.legis_init_was_updated (
    id integer NOT NULL,
    legis_init_id integer NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.legis_init_was_updated OWNER TO fabi;

--
-- Name: legis_init_was_updated_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.legis_init_was_updated_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.legis_init_was_updated_id_seq OWNER TO fabi;

--
-- Name: legis_init_was_updated_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.legis_init_was_updated_id_seq OWNED BY public.legis_init_was_updated.id;


--
-- Name: legis_inits_refs_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.legis_inits_refs_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.legis_inits_refs_id_seq OWNER TO fabi;

--
-- Name: legis_inits_refs_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.legis_inits_refs_id_seq OWNED BY public.legis_inits_refs.id;


--
-- Name: legislative_documents_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.legislative_documents_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.legislative_documents_id_seq OWNER TO fabi;

--
-- Name: legislative_documents_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.legislative_documents_id_seq OWNED BY public.legislative_documents.id;


--
-- Name: legislative_initiative_summaries_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.legislative_initiative_summaries_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.legislative_initiative_summaries_id_seq OWNER TO fabi;

--
-- Name: legislative_initiative_summaries_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.legislative_initiative_summaries_id_seq OWNED BY public.legislative_initiative_summaries.id;


--
-- Name: legislative_initiatives_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.legislative_initiatives_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.legislative_initiatives_id_seq OWNER TO fabi;

--
-- Name: legislative_initiatives_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.legislative_initiatives_id_seq OWNED BY public.legislative_initiatives.id;


--
-- Name: legislative_initiatives_with_votes; Type: MATERIALIZED VIEW; Schema: public; Owner: fabi
--

CREATE MATERIALIZED VIEW public.legislative_initiatives_with_votes AS
 SELECT id,
    gp,
    ARRAY( SELECT ROW((v.party)::text, NULL::text, v.fraction, v.infavor)::public.db_vote AS "row"
           FROM public.votes v
          WHERE (v.legislative_initiatives_id = li.id)) AS "votes: Vec<DbVote>"
   FROM public.legislative_initiatives li
  WHERE ((accepted)::text = 'a'::text)
  WITH NO DATA;


ALTER MATERIALIZED VIEW public.legislative_initiatives_with_votes OWNER TO fabi;

--
-- Name: mandates_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.mandates_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.mandates_id_seq OWNER TO fabi;

--
-- Name: mandates_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.mandates_id_seq OWNED BY public.mandates.id;


--
-- Name: ministerial_proposal_summaries_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.ministerial_proposal_summaries_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.ministerial_proposal_summaries_id_seq OWNER TO fabi;

--
-- Name: ministerial_proposal_summaries_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.ministerial_proposal_summaries_id_seq OWNED BY public.ministerial_proposal_summaries.id;


--
-- Name: ministrial_decrees; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.ministrial_decrees (
    id integer NOT NULL,
    gov_official_id integer NOT NULL,
    ris_id character varying(255) NOT NULL,
    ministrial_issuer character varying(1024) NOT NULL,
    title character varying(16384) NOT NULL,
    short_title character varying(1024) NOT NULL,
    publication_date date NOT NULL,
    part character varying(255) NOT NULL,
    emphasis character varying(16384),
    gp character varying(255),
    eli character varying(1024),
    document_url character varying(1024),
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone,
    hash bigint
);


ALTER TABLE public.ministrial_decrees OWNER TO fabi;

--
-- Name: ministrial_decrees_documents; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.ministrial_decrees_documents (
    id integer NOT NULL,
    ministrial_decree_id integer NOT NULL,
    title character varying(1024),
    document_type character varying(128) NOT NULL,
    document_url character varying(512) NOT NULL
);


ALTER TABLE public.ministrial_decrees_documents OWNER TO fabi;

--
-- Name: ministrial_decrees_documents_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.ministrial_decrees_documents_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.ministrial_decrees_documents_id_seq OWNER TO fabi;

--
-- Name: ministrial_decrees_documents_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.ministrial_decrees_documents_id_seq OWNED BY public.ministrial_decrees_documents.id;


--
-- Name: ministrial_decrees_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.ministrial_decrees_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.ministrial_decrees_id_seq OWNER TO fabi;

--
-- Name: ministrial_decrees_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.ministrial_decrees_id_seq OWNED BY public.ministrial_decrees.id;


--
-- Name: ministrial_decrees_with_docs; Type: VIEW; Schema: public; Owner: fabi
--

CREATE VIEW public.ministrial_decrees_with_docs AS
 SELECT id,
    gov_official_id,
    ris_id,
    ministrial_issuer,
    title,
    short_title,
    publication_date,
    part,
    emphasis,
    gp,
    eli,
    document_url,
    created_at,
    updated_at,
    ARRAY( SELECT ROW((doc.title)::text, (doc.document_url)::text, (doc.document_type)::text)::public.document AS "row"
           FROM public.ministrial_decrees_documents doc
          WHERE (doc.ministrial_decree_id = d.id)) AS "documents: Vec<Document>",
    ( SELECT ROW(s.id, s.full_summary, (s.short_title)::character varying, (s.short_summary)::character varying, (s.detailed_summary)::character varying, (s.complexity_scope_of_proposal)::character varying, (s.model_used)::character varying, (s.version)::character varying, s.generated_at)::public.db_ai_summary AS "row"
           FROM (public.decree_summaries mps
             JOIN public.summaries s ON ((s.id = mps.summary_id)))
          WHERE (mps.decree_id = d.id)
          ORDER BY s.generated_at DESC
         LIMIT 1) AS "ai_summary: DbAiSummary"
   FROM public.ministrial_decrees d;


ALTER VIEW public.ministrial_decrees_with_docs OWNER TO fabi;

--
-- Name: ministrial_email_info; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.ministrial_email_info (
    id integer NOT NULL,
    already_sent_id integer NOT NULL,
    marked_on timestamp with time zone DEFAULT now()
);


ALTER TABLE public.ministrial_email_info OWNER TO fabi;

--
-- Name: ministrial_email_info_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.ministrial_email_info_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.ministrial_email_info_id_seq OWNER TO fabi;

--
-- Name: ministrial_email_info_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.ministrial_email_info_id_seq OWNED BY public.ministrial_email_info.id;


--
-- Name: ministrial_issuer_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.ministrial_issuer_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.ministrial_issuer_id_seq OWNER TO fabi;

--
-- Name: ministrial_issuer_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.ministrial_issuer_id_seq OWNED BY public.ministrial_issuer.id;


--
-- Name: ministrial_proposals_documents_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.ministrial_proposals_documents_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.ministrial_proposals_documents_id_seq OWNER TO fabi;

--
-- Name: ministrial_proposals_documents_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.ministrial_proposals_documents_id_seq OWNED BY public.ministrial_proposals_documents.id;


--
-- Name: ministrial_proposals_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.ministrial_proposals_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.ministrial_proposals_id_seq OWNER TO fabi;

--
-- Name: ministrial_proposals_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.ministrial_proposals_id_seq OWNED BY public.ministrial_proposals.id;


--
-- Name: named_vote_info_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.named_vote_info_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.named_vote_info_id_seq OWNER TO fabi;

--
-- Name: named_vote_info_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.named_vote_info_id_seq OWNED BY public.named_vote_info.id;


--
-- Name: named_votes_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.named_votes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.named_votes_id_seq OWNER TO fabi;

--
-- Name: named_votes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.named_votes_id_seq OWNED BY public.named_votes.id;


--
-- Name: other_keyword_topics_legis_init_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.other_keyword_topics_legis_init_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.other_keyword_topics_legis_init_id_seq OWNER TO fabi;

--
-- Name: other_keyword_topics_legis_init_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.other_keyword_topics_legis_init_id_seq OWNED BY public.other_keyword_topics_legis_init.id;


--
-- Name: other_keyword_topics_ministrial_proposals_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.other_keyword_topics_ministrial_proposals_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.other_keyword_topics_ministrial_proposals_id_seq OWNER TO fabi;

--
-- Name: other_keyword_topics_ministrial_proposals_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.other_keyword_topics_ministrial_proposals_id_seq OWNED BY public.other_keyword_topics_ministrial_proposals.id;


--
-- Name: other_keyword_topics_proposals; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.other_keyword_topics_proposals (
    id integer NOT NULL,
    topic character varying(255) NOT NULL,
    proposals_id character varying(20) NOT NULL
);


ALTER TABLE public.other_keyword_topics_proposals OWNER TO fabi;

--
-- Name: other_keyword_topics_proposals_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.other_keyword_topics_proposals_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.other_keyword_topics_proposals_id_seq OWNER TO fabi;

--
-- Name: other_keyword_topics_proposals_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.other_keyword_topics_proposals_id_seq OWNED BY public.other_keyword_topics_proposals.id;


--
-- Name: parties; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.parties (
    id integer NOT NULL,
    name character varying(8) NOT NULL,
    gp character varying(14) NOT NULL,
    code character varying(8) NOT NULL,
    fraction integer NOT NULL,
    color character varying(7) NOT NULL
);


ALTER TABLE public.parties OWNER TO fabi;

--
-- Name: parties_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.parties_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.parties_id_seq OWNER TO fabi;

--
-- Name: parties_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.parties_id_seq OWNED BY public.parties.id;


--
-- Name: plenar_infos_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.plenar_infos_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.plenar_infos_id_seq OWNER TO fabi;

--
-- Name: plenar_infos_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.plenar_infos_id_seq OWNED BY public.plenar_infos.id;


--
-- Name: plenar_speech_legis_inits_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.plenar_speech_legis_inits_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.plenar_speech_legis_inits_id_seq OWNER TO fabi;

--
-- Name: plenar_speech_legis_inits_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.plenar_speech_legis_inits_id_seq OWNED BY public.plenar_speech_legis_inits.id;


--
-- Name: plenar_speech_links_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.plenar_speech_links_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.plenar_speech_links_id_seq OWNER TO fabi;

--
-- Name: plenar_speech_links_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.plenar_speech_links_id_seq OWNED BY public.plenar_speech_links.id;


--
-- Name: plenar_speeches_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.plenar_speeches_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.plenar_speeches_id_seq OWNER TO fabi;

--
-- Name: plenar_speeches_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.plenar_speeches_id_seq OWNED BY public.plenar_speeches.id;


--
-- Name: political_answers; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.political_answers (
    id integer NOT NULL,
    question_id integer NOT NULL,
    delegate_id integer,
    answer text NOT NULL,
    stance_llm character varying(255) NOT NULL,
    is_strong_reference boolean,
    model_used character varying(255),
    created_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.political_answers OWNER TO fabi;

--
-- Name: political_answers_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.political_answers_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.political_answers_id_seq OWNER TO fabi;

--
-- Name: political_answers_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.political_answers_id_seq OWNED BY public.political_answers.id;


--
-- Name: political_opinions; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.political_opinions (
    id integer NOT NULL,
    delegate_id integer NOT NULL,
    question_id integer NOT NULL,
    answer_id integer NOT NULL,
    stance character varying(255) NOT NULL,
    stance_acc double precision,
    pro_strong_ref_score double precision NOT NULL,
    contra_strong_ref_score double precision NOT NULL,
    ref_score double precision NOT NULL
);


ALTER TABLE public.political_opinions OWNER TO fabi;

--
-- Name: political_opinions_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.political_opinions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.political_opinions_id_seq OWNER TO fabi;

--
-- Name: political_opinions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.political_opinions_id_seq OWNED BY public.political_opinions.id;


--
-- Name: political_positions; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.political_positions (
    id integer NOT NULL,
    delegate_id integer NOT NULL,
    is_left double precision NOT NULL,
    is_not_left double precision NOT NULL,
    is_liberal double precision NOT NULL,
    is_not_liberal double precision NOT NULL,
    neutral_count integer NOT NULL
);


ALTER TABLE public.political_positions OWNER TO fabi;

--
-- Name: political_positions_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.political_positions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.political_positions_id_seq OWNER TO fabi;

--
-- Name: political_positions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.political_positions_id_seq OWNED BY public.political_positions.id;


--
-- Name: political_questions; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.political_questions (
    id integer NOT NULL,
    question text NOT NULL,
    is_left boolean,
    is_liberal boolean
);


ALTER TABLE public.political_questions OWNER TO fabi;

--
-- Name: political_questions_detailed_topics; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.political_questions_detailed_topics (
    id integer NOT NULL,
    question_id integer NOT NULL,
    topic character varying(255) NOT NULL
);


ALTER TABLE public.political_questions_detailed_topics OWNER TO fabi;

--
-- Name: political_questions_detailed_topics_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.political_questions_detailed_topics_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.political_questions_detailed_topics_id_seq OWNER TO fabi;

--
-- Name: political_questions_detailed_topics_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.political_questions_detailed_topics_id_seq OWNED BY public.political_questions_detailed_topics.id;


--
-- Name: political_questions_detailed_topics_influence; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.political_questions_detailed_topics_influence (
    id integer NOT NULL,
    question_id integer NOT NULL,
    topic character varying(255) NOT NULL,
    influence double precision NOT NULL
);


ALTER TABLE public.political_questions_detailed_topics_influence OWNER TO fabi;

--
-- Name: political_questions_detailed_topics_influence_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.political_questions_detailed_topics_influence_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.political_questions_detailed_topics_influence_id_seq OWNER TO fabi;

--
-- Name: political_questions_detailed_topics_influence_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.political_questions_detailed_topics_influence_id_seq OWNED BY public.political_questions_detailed_topics_influence.id;


--
-- Name: political_questions_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.political_questions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.political_questions_id_seq OWNER TO fabi;

--
-- Name: political_questions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.political_questions_id_seq OWNED BY public.political_questions.id;


--
-- Name: political_questions_topics; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.political_questions_topics (
    id integer NOT NULL,
    question_id integer NOT NULL,
    topic character varying(255) NOT NULL
);


ALTER TABLE public.political_questions_topics OWNER TO fabi;

--
-- Name: political_questions_topics_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.political_questions_topics_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.political_questions_topics_id_seq OWNER TO fabi;

--
-- Name: political_questions_topics_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.political_questions_topics_id_seq OWNED BY public.political_questions_topics.id;


--
-- Name: political_questions_topics_influence; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.political_questions_topics_influence (
    id integer NOT NULL,
    question_id integer NOT NULL,
    topic character varying(255) NOT NULL,
    influence double precision NOT NULL
);


ALTER TABLE public.political_questions_topics_influence OWNER TO fabi;

--
-- Name: political_questions_topics_influence_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.political_questions_topics_influence_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.political_questions_topics_influence_id_seq OWNER TO fabi;

--
-- Name: political_questions_topics_influence_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.political_questions_topics_influence_id_seq OWNED BY public.political_questions_topics_influence.id;


--
-- Name: proposal_delegates; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.proposal_delegates (
    proposal_id character varying(20) NOT NULL,
    delegate_id integer NOT NULL,
    is_receiver boolean NOT NULL
);


ALTER TABLE public.proposal_delegates OWNER TO fabi;

--
-- Name: proposal_documents; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.proposal_documents (
    id integer NOT NULL,
    proposal_id character varying(20) NOT NULL,
    title character varying(1024),
    document_type character varying(100) NOT NULL,
    document_url character varying(500) NOT NULL
);


ALTER TABLE public.proposal_documents OWNER TO fabi;

--
-- Name: proposal_documents_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.proposal_documents_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.proposal_documents_id_seq OWNER TO fabi;

--
-- Name: proposal_documents_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.proposal_documents_id_seq OWNED BY public.proposal_documents.id;


--
-- Name: proposal_email_info; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.proposal_email_info (
    id integer NOT NULL,
    already_sent_id integer NOT NULL,
    marked_on timestamp with time zone DEFAULT now()
);


ALTER TABLE public.proposal_email_info OWNER TO fabi;

--
-- Name: proposal_email_info_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.proposal_email_info_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.proposal_email_info_id_seq OWNER TO fabi;

--
-- Name: proposal_email_info_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.proposal_email_info_id_seq OWNED BY public.proposal_email_info.id;


--
-- Name: proposals; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.proposals (
    id character varying(20) NOT NULL,
    ityp character varying(7) NOT NULL,
    gp character varying(7) NOT NULL,
    title text NOT NULL,
    description text,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone
);


ALTER TABLE public.proposals OWNER TO fabi;

--
-- Name: questions; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.questions (
    id integer NOT NULL,
    issuer_id integer NOT NULL,
    delegate_id integer NOT NULL,
    title character varying(1024) NOT NULL,
    body text NOT NULL,
    created_at timestamp without time zone DEFAULT now() NOT NULL,
    updated_at timestamp without time zone DEFAULT now() NOT NULL,
    is_sent boolean DEFAULT false NOT NULL,
    is_sendable boolean DEFAULT false NOT NULL,
    is_answerable boolean DEFAULT true NOT NULL
);


ALTER TABLE public.questions OWNER TO fabi;

--
-- Name: questions_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.questions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.questions_id_seq OWNER TO fabi;

--
-- Name: questions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.questions_id_seq OWNED BY public.questions.id;


--
-- Name: quiz; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.quiz (
    id integer NOT NULL,
    title character varying(255) NOT NULL,
    description character varying(2048),
    stars integer DEFAULT 0
);


ALTER TABLE public.quiz OWNER TO fabi;

--
-- Name: quiz_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.quiz_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.quiz_id_seq OWNER TO fabi;

--
-- Name: quiz_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.quiz_id_seq OWNED BY public.quiz.id;


--
-- Name: quiz_questions; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.quiz_questions (
    id integer NOT NULL,
    quiz_id integer,
    question character varying(2048) NOT NULL,
    answer1 character varying(255) NOT NULL,
    answer2 character varying(255) NOT NULL,
    answer3 character varying(255) NOT NULL,
    answer4 character varying(255) NOT NULL,
    correct_answer integer NOT NULL
);


ALTER TABLE public.quiz_questions OWNER TO fabi;

--
-- Name: quiz_questions_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.quiz_questions_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.quiz_questions_id_seq OWNER TO fabi;

--
-- Name: quiz_questions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.quiz_questions_id_seq OWNED BY public.quiz_questions.id;


--
-- Name: seat_history; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.seat_history (
    id integer NOT NULL,
    delegate_id integer,
    seat_col integer NOT NULL,
    seat_row integer NOT NULL,
    council character varying(255) NOT NULL,
    insertion_date date DEFAULT now(),
    gp character varying(255) NOT NULL
);


ALTER TABLE public.seat_history OWNER TO fabi;

--
-- Name: seat_history_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.seat_history_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.seat_history_id_seq OWNER TO fabi;

--
-- Name: seat_history_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.seat_history_id_seq OWNED BY public.seat_history.id;


--
-- Name: somes_user; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.somes_user (
    id integer NOT NULL,
    email character varying(356) NOT NULL,
    is_email_hashed boolean DEFAULT false NOT NULL,
    is_admin boolean DEFAULT false NOT NULL,
    send_new_vote_results_mails boolean DEFAULT true NOT NULL,
    send_new_delegate_activity_mails boolean DEFAULT true NOT NULL,
    send_new_ministrial_prop_mails boolean DEFAULT false NOT NULL,
    send_new_ministrial_prop_by_favo_mails boolean DEFAULT false NOT NULL,
    send_new_decree_mails boolean DEFAULT false NOT NULL,
    send_new_decree_by_favo_mails boolean DEFAULT false NOT NULL,
    send_new_proposal_mails boolean DEFAULT false NOT NULL,
    send_new_proposal_by_favo_mails boolean DEFAULT false NOT NULL,
    send_new_vote_result_by_favo_mails boolean DEFAULT false NOT NULL
);


ALTER TABLE public.somes_user OWNER TO fabi;

--
-- Name: somes_user_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.somes_user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.somes_user_id_seq OWNER TO fabi;

--
-- Name: somes_user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.somes_user_id_seq OWNED BY public.somes_user.id;


--
-- Name: speech_complexity; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.speech_complexity (
    id integer NOT NULL,
    speech_id integer NOT NULL,
    flesch_kincaid real,
    smog real,
    gunning_fog real,
    coleman_liau real
);


ALTER TABLE public.speech_complexity OWNER TO fabi;

--
-- Name: speech_complexity_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.speech_complexity_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.speech_complexity_id_seq OWNER TO fabi;

--
-- Name: speech_complexity_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.speech_complexity_id_seq OWNED BY public.speech_complexity.id;


--
-- Name: speech_summaries; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.speech_summaries (
    id integer NOT NULL,
    speech_id integer,
    summary_id integer
);


ALTER TABLE public.speech_summaries OWNER TO fabi;

--
-- Name: speech_summaries_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.speech_summaries_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.speech_summaries_id_seq OWNER TO fabi;

--
-- Name: speech_summaries_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.speech_summaries_id_seq OWNED BY public.speech_summaries.id;


--
-- Name: speeches; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.speeches (
    id integer NOT NULL,
    delegate_id integer NOT NULL,
    infavor boolean,
    opinion character varying(255),
    legislative_initiatives_id integer NOT NULL
);


ALTER TABLE public.speeches OWNER TO fabi;

--
-- Name: speeches_html_urls; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.speeches_html_urls (
    id integer NOT NULL,
    speech_id integer NOT NULL,
    document_url character varying(500) NOT NULL
);


ALTER TABLE public.speeches_html_urls OWNER TO fabi;

--
-- Name: speeches_html_urls_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.speeches_html_urls_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.speeches_html_urls_id_seq OWNER TO fabi;

--
-- Name: speeches_html_urls_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.speeches_html_urls_id_seq OWNED BY public.speeches_html_urls.id;


--
-- Name: speeches_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.speeches_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.speeches_id_seq OWNER TO fabi;

--
-- Name: speeches_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.speeches_id_seq OWNED BY public.speeches.id;


--
-- Name: stance_citations; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.stance_citations (
    id integer NOT NULL,
    answer_id integer NOT NULL,
    speech_id integer NOT NULL,
    point_uuid uuid NOT NULL,
    text_snippet text,
    created_at timestamp with time zone DEFAULT now()
);


ALTER TABLE public.stance_citations OWNER TO fabi;

--
-- Name: stance_citations_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.stance_citations_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.stance_citations_id_seq OWNER TO fabi;

--
-- Name: stance_citations_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.stance_citations_id_seq OWNED BY public.stance_citations.id;


--
-- Name: summaries_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.summaries_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.summaries_id_seq OWNER TO fabi;

--
-- Name: summaries_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.summaries_id_seq OWNED BY public.summaries.id;


--
-- Name: topics_legis_init_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.topics_legis_init_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.topics_legis_init_id_seq OWNER TO fabi;

--
-- Name: topics_legis_init_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.topics_legis_init_id_seq OWNED BY public.topics_legis_init.id;


--
-- Name: topics_ministrial_proposals_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.topics_ministrial_proposals_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.topics_ministrial_proposals_id_seq OWNER TO fabi;

--
-- Name: topics_ministrial_proposals_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.topics_ministrial_proposals_id_seq OWNED BY public.topics_ministrial_proposals.id;


--
-- Name: topics_proposals; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.topics_proposals (
    id integer NOT NULL,
    topic character varying(255) NOT NULL,
    proposals_id character varying(20) NOT NULL
);


ALTER TABLE public.topics_proposals OWNER TO fabi;

--
-- Name: topics_proposals_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.topics_proposals_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.topics_proposals_id_seq OWNER TO fabi;

--
-- Name: topics_proposals_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.topics_proposals_id_seq OWNED BY public.topics_proposals.id;


--
-- Name: transcriptionsqa; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.transcriptionsqa (
    id integer NOT NULL,
    question text NOT NULL,
    answer text NOT NULL,
    transcription_id integer NOT NULL
);


ALTER TABLE public.transcriptionsqa OWNER TO fabi;

--
-- Name: transcriptionsqa_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.transcriptionsqa_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.transcriptionsqa_id_seq OWNER TO fabi;

--
-- Name: transcriptionsqa_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.transcriptionsqa_id_seq OWNED BY public.transcriptionsqa.id;


--
-- Name: unique_eurovoc_topics; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.unique_eurovoc_topics (
    id integer NOT NULL,
    topic_name character varying(255) NOT NULL
);


ALTER TABLE public.unique_eurovoc_topics OWNER TO fabi;

--
-- Name: unique_topics; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.unique_topics (
    id integer NOT NULL,
    topic_name character varying(255) NOT NULL
);


ALTER TABLE public.unique_topics OWNER TO fabi;

--
-- Name: unique_topics_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.unique_topics_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.unique_topics_id_seq OWNER TO fabi;

--
-- Name: unique_topics_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.unique_topics_id_seq OWNED BY public.unique_eurovoc_topics.id;


--
-- Name: unique_topics_id_seq1; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.unique_topics_id_seq1
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.unique_topics_id_seq1 OWNER TO fabi;

--
-- Name: unique_topics_id_seq1; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.unique_topics_id_seq1 OWNED BY public.unique_topics.id;


--
-- Name: user_topics; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.user_topics (
    id integer NOT NULL,
    user_id integer NOT NULL,
    topic_id integer NOT NULL
);


ALTER TABLE public.user_topics OWNER TO fabi;

--
-- Name: user_topics_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.user_topics_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.user_topics_id_seq OWNER TO fabi;

--
-- Name: user_topics_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.user_topics_id_seq OWNED BY public.user_topics.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.users (
    id integer NOT NULL,
    username character varying(255) NOT NULL,
    email character varying(300) NOT NULL,
    password_hash character varying(356) NOT NULL
);


ALTER TABLE public.users OWNER TO fabi;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.users_id_seq OWNER TO fabi;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: walo; Type: TABLE; Schema: public; Owner: fabi
--

CREATE TABLE public.walo (
    id integer NOT NULL,
    question_statement text,
    new_keywords_topics text,
    spoe_justification text,
    gruene_justification text,
    neos_justification text,
    fpoe_justification text,
    oevp_justification text,
    somes_link text,
    law_link text,
    erklaerbaer text
);


ALTER TABLE public.walo OWNER TO fabi;

--
-- Name: walo_id_seq; Type: SEQUENCE; Schema: public; Owner: fabi
--

CREATE SEQUENCE public.walo_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.walo_id_seq OWNER TO fabi;

--
-- Name: walo_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: fabi
--

ALTER SEQUENCE public.walo_id_seq OWNED BY public.walo.id;


--
-- Name: absences id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.absences ALTER COLUMN id SET DEFAULT nextval('public.absences_id_seq'::regclass);


--
-- Name: answers id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.answers ALTER COLUMN id SET DEFAULT nextval('public.answers_id_seq'::regclass);


--
-- Name: dates id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.dates ALTER COLUMN id SET DEFAULT nextval('public.dates_id_seq'::regclass);


--
-- Name: dates_topics id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.dates_topics ALTER COLUMN id SET DEFAULT nextval('public.dates_topics_id_seq'::regclass);


--
-- Name: debates id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.debates ALTER COLUMN id SET DEFAULT nextval('public.debates_id_seq'::regclass);


--
-- Name: decree_email_info id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.decree_email_info ALTER COLUMN id SET DEFAULT nextval('public.decree_email_info_id_seq'::regclass);


--
-- Name: decree_summaries id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.decree_summaries ALTER COLUMN id SET DEFAULT nextval('public.decree_summaries_id_seq'::regclass);


--
-- Name: delegate_ages id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.delegate_ages ALTER COLUMN id SET DEFAULT nextval('public.delegate_ages_id_seq'::regclass);


--
-- Name: delegate_matching id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.delegate_matching ALTER COLUMN id SET DEFAULT nextval('public.delegate_matching_id_seq'::regclass);


--
-- Name: delegates_divisions id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.delegates_divisions ALTER COLUMN id SET DEFAULT nextval('public.delegates_divisions_id_seq'::regclass);


--
-- Name: division_interest_score id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.division_interest_score ALTER COLUMN id SET DEFAULT nextval('public.division_interest_score_id_seq'::regclass);


--
-- Name: emphasis_dataset id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.emphasis_dataset ALTER COLUMN id SET DEFAULT nextval('public.emphasis_dataset_id_seq'::regclass);


--
-- Name: eurovoc_topics_legis_init id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.eurovoc_topics_legis_init ALTER COLUMN id SET DEFAULT nextval('public.eurovoc_topics_legis_init_id_seq'::regclass);


--
-- Name: eurovoc_topics_ministrial_proposals id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.eurovoc_topics_ministrial_proposals ALTER COLUMN id SET DEFAULT nextval('public.eurovoc_topics_ministrial_proposals_id_seq'::regclass);


--
-- Name: eurovoc_topics_proposals id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.eurovoc_topics_proposals ALTER COLUMN id SET DEFAULT nextval('public.eurovoc_topics_proposals_id_seq'::regclass);


--
-- Name: events id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.events ALTER COLUMN id SET DEFAULT nextval('public.events_id_seq'::regclass);


--
-- Name: favo_dels id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.favo_dels ALTER COLUMN id SET DEFAULT nextval('public.favo_dels_id_seq'::regclass);


--
-- Name: favo_legis_inits id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.favo_legis_inits ALTER COLUMN id SET DEFAULT nextval('public.favo_legis_inits_id_seq'::regclass);


--
-- Name: generated_eurovoc_topics id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.generated_eurovoc_topics ALTER COLUMN id SET DEFAULT nextval('public.generated_eurovoc_topics_id_seq'::regclass);


--
-- Name: interjections id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.interjections ALTER COLUMN id SET DEFAULT nextval('public.interjections_id_seq'::regclass);


--
-- Name: introduction_transcriptions id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.introduction_transcriptions ALTER COLUMN id SET DEFAULT nextval('public.introduction_transcriptions_id_seq'::regclass);


--
-- Name: last_vector_speech_update id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.last_vector_speech_update ALTER COLUMN id SET DEFAULT nextval('public.last_vector_speech_update_id_seq'::regclass);


--
-- Name: legis_email_info id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_email_info ALTER COLUMN id SET DEFAULT nextval('public.legis_email_info_id_seq'::regclass);


--
-- Name: legis_init_delegates id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_init_delegates ALTER COLUMN id SET DEFAULT nextval('public.legis_init_delegates_id_seq'::regclass);


--
-- Name: legis_init_was_updated id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_init_was_updated ALTER COLUMN id SET DEFAULT nextval('public.legis_init_was_updated_id_seq'::regclass);


--
-- Name: legis_inits_refs id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_inits_refs ALTER COLUMN id SET DEFAULT nextval('public.legis_inits_refs_id_seq'::regclass);


--
-- Name: legislative_documents id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legislative_documents ALTER COLUMN id SET DEFAULT nextval('public.legislative_documents_id_seq'::regclass);


--
-- Name: legislative_initiative_summaries id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legislative_initiative_summaries ALTER COLUMN id SET DEFAULT nextval('public.legislative_initiative_summaries_id_seq'::regclass);


--
-- Name: legislative_initiatives id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legislative_initiatives ALTER COLUMN id SET DEFAULT nextval('public.legislative_initiatives_id_seq'::regclass);


--
-- Name: mandates id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.mandates ALTER COLUMN id SET DEFAULT nextval('public.mandates_id_seq'::regclass);


--
-- Name: ministerial_proposal_summaries id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministerial_proposal_summaries ALTER COLUMN id SET DEFAULT nextval('public.ministerial_proposal_summaries_id_seq'::regclass);


--
-- Name: ministrial_decrees id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_decrees ALTER COLUMN id SET DEFAULT nextval('public.ministrial_decrees_id_seq'::regclass);


--
-- Name: ministrial_decrees_documents id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_decrees_documents ALTER COLUMN id SET DEFAULT nextval('public.ministrial_decrees_documents_id_seq'::regclass);


--
-- Name: ministrial_email_info id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_email_info ALTER COLUMN id SET DEFAULT nextval('public.ministrial_email_info_id_seq'::regclass);


--
-- Name: ministrial_issuer id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_issuer ALTER COLUMN id SET DEFAULT nextval('public.ministrial_issuer_id_seq'::regclass);


--
-- Name: ministrial_proposals id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_proposals ALTER COLUMN id SET DEFAULT nextval('public.ministrial_proposals_id_seq'::regclass);


--
-- Name: ministrial_proposals_documents id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_proposals_documents ALTER COLUMN id SET DEFAULT nextval('public.ministrial_proposals_documents_id_seq'::regclass);


--
-- Name: named_vote_info id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.named_vote_info ALTER COLUMN id SET DEFAULT nextval('public.named_vote_info_id_seq'::regclass);


--
-- Name: named_votes id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.named_votes ALTER COLUMN id SET DEFAULT nextval('public.named_votes_id_seq'::regclass);


--
-- Name: other_keyword_topics_legis_init id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.other_keyword_topics_legis_init ALTER COLUMN id SET DEFAULT nextval('public.other_keyword_topics_legis_init_id_seq'::regclass);


--
-- Name: other_keyword_topics_ministrial_proposals id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.other_keyword_topics_ministrial_proposals ALTER COLUMN id SET DEFAULT nextval('public.other_keyword_topics_ministrial_proposals_id_seq'::regclass);


--
-- Name: other_keyword_topics_proposals id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.other_keyword_topics_proposals ALTER COLUMN id SET DEFAULT nextval('public.other_keyword_topics_proposals_id_seq'::regclass);


--
-- Name: parties id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.parties ALTER COLUMN id SET DEFAULT nextval('public.parties_id_seq'::regclass);


--
-- Name: plenar_infos id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_infos ALTER COLUMN id SET DEFAULT nextval('public.plenar_infos_id_seq'::regclass);


--
-- Name: plenar_speech_legis_inits id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_speech_legis_inits ALTER COLUMN id SET DEFAULT nextval('public.plenar_speech_legis_inits_id_seq'::regclass);


--
-- Name: plenar_speech_links id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_speech_links ALTER COLUMN id SET DEFAULT nextval('public.plenar_speech_links_id_seq'::regclass);


--
-- Name: plenar_speeches id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_speeches ALTER COLUMN id SET DEFAULT nextval('public.plenar_speeches_id_seq'::regclass);


--
-- Name: political_answers id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_answers ALTER COLUMN id SET DEFAULT nextval('public.political_answers_id_seq'::regclass);


--
-- Name: political_opinions id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_opinions ALTER COLUMN id SET DEFAULT nextval('public.political_opinions_id_seq'::regclass);


--
-- Name: political_positions id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_positions ALTER COLUMN id SET DEFAULT nextval('public.political_positions_id_seq'::regclass);


--
-- Name: political_questions id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions ALTER COLUMN id SET DEFAULT nextval('public.political_questions_id_seq'::regclass);


--
-- Name: political_questions_detailed_topics id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_detailed_topics ALTER COLUMN id SET DEFAULT nextval('public.political_questions_detailed_topics_id_seq'::regclass);


--
-- Name: political_questions_detailed_topics_influence id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_detailed_topics_influence ALTER COLUMN id SET DEFAULT nextval('public.political_questions_detailed_topics_influence_id_seq'::regclass);


--
-- Name: political_questions_topics id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_topics ALTER COLUMN id SET DEFAULT nextval('public.political_questions_topics_id_seq'::regclass);


--
-- Name: political_questions_topics_influence id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_topics_influence ALTER COLUMN id SET DEFAULT nextval('public.political_questions_topics_influence_id_seq'::regclass);


--
-- Name: proposal_documents id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.proposal_documents ALTER COLUMN id SET DEFAULT nextval('public.proposal_documents_id_seq'::regclass);


--
-- Name: proposal_email_info id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.proposal_email_info ALTER COLUMN id SET DEFAULT nextval('public.proposal_email_info_id_seq'::regclass);


--
-- Name: questions id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.questions ALTER COLUMN id SET DEFAULT nextval('public.questions_id_seq'::regclass);


--
-- Name: quiz id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.quiz ALTER COLUMN id SET DEFAULT nextval('public.quiz_id_seq'::regclass);


--
-- Name: quiz_questions id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.quiz_questions ALTER COLUMN id SET DEFAULT nextval('public.quiz_questions_id_seq'::regclass);


--
-- Name: seat_history id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.seat_history ALTER COLUMN id SET DEFAULT nextval('public.seat_history_id_seq'::regclass);


--
-- Name: somes_user id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.somes_user ALTER COLUMN id SET DEFAULT nextval('public.somes_user_id_seq'::regclass);


--
-- Name: speech_complexity id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speech_complexity ALTER COLUMN id SET DEFAULT nextval('public.speech_complexity_id_seq'::regclass);


--
-- Name: speech_summaries id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speech_summaries ALTER COLUMN id SET DEFAULT nextval('public.speech_summaries_id_seq'::regclass);


--
-- Name: speeches id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speeches ALTER COLUMN id SET DEFAULT nextval('public.speeches_id_seq'::regclass);


--
-- Name: speeches_html_urls id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speeches_html_urls ALTER COLUMN id SET DEFAULT nextval('public.speeches_html_urls_id_seq'::regclass);


--
-- Name: stance_citations id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.stance_citations ALTER COLUMN id SET DEFAULT nextval('public.stance_citations_id_seq'::regclass);


--
-- Name: summaries id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.summaries ALTER COLUMN id SET DEFAULT nextval('public.summaries_id_seq'::regclass);


--
-- Name: topics_legis_init id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.topics_legis_init ALTER COLUMN id SET DEFAULT nextval('public.topics_legis_init_id_seq'::regclass);


--
-- Name: topics_ministrial_proposals id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.topics_ministrial_proposals ALTER COLUMN id SET DEFAULT nextval('public.topics_ministrial_proposals_id_seq'::regclass);


--
-- Name: topics_proposals id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.topics_proposals ALTER COLUMN id SET DEFAULT nextval('public.topics_proposals_id_seq'::regclass);


--
-- Name: transcriptionsqa id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.transcriptionsqa ALTER COLUMN id SET DEFAULT nextval('public.transcriptionsqa_id_seq'::regclass);


--
-- Name: unique_eurovoc_topics id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.unique_eurovoc_topics ALTER COLUMN id SET DEFAULT nextval('public.unique_topics_id_seq'::regclass);


--
-- Name: unique_topics id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.unique_topics ALTER COLUMN id SET DEFAULT nextval('public.unique_topics_id_seq1'::regclass);


--
-- Name: user_topics id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.user_topics ALTER COLUMN id SET DEFAULT nextval('public.user_topics_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: walo id; Type: DEFAULT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.walo ALTER COLUMN id SET DEFAULT nextval('public.walo_id_seq'::regclass);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: absences absences_delegate_id_plenary_session_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.absences
    ADD CONSTRAINT absences_delegate_id_plenary_session_id_key UNIQUE (delegate_id, plenary_session_id);


--
-- Name: absences absences_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.absences
    ADD CONSTRAINT absences_pkey PRIMARY KEY (id);


--
-- Name: answers answers_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.answers
    ADD CONSTRAINT answers_pkey PRIMARY KEY (id);


--
-- Name: call_to_order call_to_order_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.call_to_order
    ADD CONSTRAINT call_to_order_pkey PRIMARY KEY (id);


--
-- Name: contacts contacts_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.contacts
    ADD CONSTRAINT contacts_pkey PRIMARY KEY (id);


--
-- Name: contacts contacts_twitter_url_facebook_url_website_url_youtube_url_i_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.contacts
    ADD CONSTRAINT contacts_twitter_url_facebook_url_website_url_youtube_url_i_key UNIQUE (twitter_url, facebook_url, website_url, youtube_url, instagram_url, tiktok_url);


--
-- Name: dates dates_date_title_appointment_type_committee_media_relevant_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.dates
    ADD CONSTRAINT dates_date_title_appointment_type_committee_media_relevant_key UNIQUE (date, title, appointment_type, committee, media_relevant);


--
-- Name: dates dates_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.dates
    ADD CONSTRAINT dates_pkey PRIMARY KEY (id);


--
-- Name: dates_topics dates_topics_date_id_topic_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.dates_topics
    ADD CONSTRAINT dates_topics_date_id_topic_key UNIQUE (date_id, topic);


--
-- Name: dates_topics dates_topics_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.dates_topics
    ADD CONSTRAINT dates_topics_pkey PRIMARY KEY (id);


--
-- Name: debates debates_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.debates
    ADD CONSTRAINT debates_pkey PRIMARY KEY (id);


--
-- Name: debates debates_plenar_id_about_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.debates
    ADD CONSTRAINT debates_plenar_id_about_key UNIQUE (plenar_id, about);


--
-- Name: decree_email_info decree_email_info_already_sent_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.decree_email_info
    ADD CONSTRAINT decree_email_info_already_sent_id_key UNIQUE (already_sent_id);


--
-- Name: decree_email_info decree_email_info_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.decree_email_info
    ADD CONSTRAINT decree_email_info_pkey PRIMARY KEY (id);


--
-- Name: decree_summaries decree_summaries_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.decree_summaries
    ADD CONSTRAINT decree_summaries_pkey PRIMARY KEY (id);


--
-- Name: delegate_ages delegate_ages_delegate_id_legislative_period_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.delegate_ages
    ADD CONSTRAINT delegate_ages_delegate_id_legislative_period_key UNIQUE (delegate_id, legislative_period);


--
-- Name: delegate_ages delegate_ages_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.delegate_ages
    ADD CONSTRAINT delegate_ages_pkey PRIMARY KEY (id);


--
-- Name: delegate_matching delegate_matching_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.delegate_matching
    ADD CONSTRAINT delegate_matching_pkey PRIMARY KEY (id);


--
-- Name: delegates_divisions delegates_divisions_delegate_id_division_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.delegates_divisions
    ADD CONSTRAINT delegates_divisions_delegate_id_division_key UNIQUE (delegate_id, division);


--
-- Name: delegates_divisions delegates_divisions_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.delegates_divisions
    ADD CONSTRAINT delegates_divisions_pkey PRIMARY KEY (id);


--
-- Name: delegates delegates_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.delegates
    ADD CONSTRAINT delegates_pkey PRIMARY KEY (id);


--
-- Name: division_interest_score division_interest_score_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.division_interest_score
    ADD CONSTRAINT division_interest_score_pkey PRIMARY KEY (id);


--
-- Name: emphasis_dataset emphasis_dataset_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.emphasis_dataset
    ADD CONSTRAINT emphasis_dataset_pkey PRIMARY KEY (id);


--
-- Name: eurovoc_topics_legis_init eurovoc_topics_legis_init_legislative_initiatives_id_topic_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.eurovoc_topics_legis_init
    ADD CONSTRAINT eurovoc_topics_legis_init_legislative_initiatives_id_topic_key UNIQUE (legislative_initiatives_id, topic);


--
-- Name: eurovoc_topics_legis_init eurovoc_topics_legis_init_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.eurovoc_topics_legis_init
    ADD CONSTRAINT eurovoc_topics_legis_init_pkey PRIMARY KEY (id);


--
-- Name: eurovoc_topics_ministrial_proposals eurovoc_topics_ministrial_prop_ministrial_proposal_id_topic_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.eurovoc_topics_ministrial_proposals
    ADD CONSTRAINT eurovoc_topics_ministrial_prop_ministrial_proposal_id_topic_key UNIQUE (ministrial_proposal_id, topic);


--
-- Name: eurovoc_topics_ministrial_proposals eurovoc_topics_ministrial_proposals_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.eurovoc_topics_ministrial_proposals
    ADD CONSTRAINT eurovoc_topics_ministrial_proposals_pkey PRIMARY KEY (id);


--
-- Name: eurovoc_topics_proposals eurovoc_topics_proposals_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.eurovoc_topics_proposals
    ADD CONSTRAINT eurovoc_topics_proposals_pkey PRIMARY KEY (id);


--
-- Name: eurovoc_topics_proposals eurovoc_topics_proposals_proposals_id_topic_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.eurovoc_topics_proposals
    ADD CONSTRAINT eurovoc_topics_proposals_proposals_id_topic_key UNIQUE (proposals_id, topic);


--
-- Name: events events_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.events
    ADD CONSTRAINT events_pkey PRIMARY KEY (id);


--
-- Name: favo_dels favo_dels_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.favo_dels
    ADD CONSTRAINT favo_dels_pkey PRIMARY KEY (id);


--
-- Name: favo_legis_inits favo_legis_inits_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.favo_legis_inits
    ADD CONSTRAINT favo_legis_inits_pkey PRIMARY KEY (id);


--
-- Name: generated_eurovoc_topics generated_eurovoc_topics_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.generated_eurovoc_topics
    ADD CONSTRAINT generated_eurovoc_topics_pkey PRIMARY KEY (id);


--
-- Name: interjections interjections_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.interjections
    ADD CONSTRAINT interjections_pkey PRIMARY KEY (id);


--
-- Name: interjections interjections_rel_start_idx_rel_end_idx_interjection_text_i_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.interjections
    ADD CONSTRAINT interjections_rel_start_idx_rel_end_idx_interjection_text_i_key UNIQUE (rel_start_idx, rel_end_idx, interjection_text, interjector_delegate_id, plenar_speech_id);


--
-- Name: introduction_transcriptions introduction_transcriptions_delegate_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.introduction_transcriptions
    ADD CONSTRAINT introduction_transcriptions_delegate_id_key UNIQUE (delegate_id);


--
-- Name: introduction_transcriptions introduction_transcriptions_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.introduction_transcriptions
    ADD CONSTRAINT introduction_transcriptions_pkey PRIMARY KEY (id);


--
-- Name: political_answers is_strong_ref_unique; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_answers
    ADD CONSTRAINT is_strong_ref_unique UNIQUE (is_strong_reference, question_id, stance_llm);


--
-- Name: last_vector_speech_update last_vector_speech_update_delegate_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.last_vector_speech_update
    ADD CONSTRAINT last_vector_speech_update_delegate_id_key UNIQUE (delegate_id);


--
-- Name: last_vector_speech_update last_vector_speech_update_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.last_vector_speech_update
    ADD CONSTRAINT last_vector_speech_update_pkey PRIMARY KEY (id);


--
-- Name: legis_email_info legis_email_info_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_email_info
    ADD CONSTRAINT legis_email_info_pkey PRIMARY KEY (id);


--
-- Name: legis_init_delegates legis_init_delegates_legis_init_id_delegate_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_init_delegates
    ADD CONSTRAINT legis_init_delegates_legis_init_id_delegate_id_key UNIQUE (legis_init_id, delegate_id);


--
-- Name: legis_init_delegates legis_init_delegates_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_init_delegates
    ADD CONSTRAINT legis_init_delegates_pkey PRIMARY KEY (id);


--
-- Name: legis_init_was_updated legis_init_was_updated_legis_init_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_init_was_updated
    ADD CONSTRAINT legis_init_was_updated_legis_init_id_key UNIQUE (legis_init_id);


--
-- Name: legis_init_was_updated legis_init_was_updated_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_init_was_updated
    ADD CONSTRAINT legis_init_was_updated_pkey PRIMARY KEY (id);


--
-- Name: legis_inits_refs legis_inits_refs_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_inits_refs
    ADD CONSTRAINT legis_inits_refs_pkey PRIMARY KEY (id);


--
-- Name: legis_inits_refs legis_inits_refs_ref_gp_ref_ityp_ref_inr_origin_legis_init__key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_inits_refs
    ADD CONSTRAINT legis_inits_refs_ref_gp_ref_ityp_ref_inr_origin_legis_init__key UNIQUE (ref_gp, ref_ityp, ref_inr, origin_legis_init_id);


--
-- Name: legislative_documents legislative_documents_legislative_initiatives_id_document_u_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legislative_documents
    ADD CONSTRAINT legislative_documents_legislative_initiatives_id_document_u_key UNIQUE (legislative_initiatives_id, document_url);


--
-- Name: legislative_documents legislative_documents_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legislative_documents
    ADD CONSTRAINT legislative_documents_pkey PRIMARY KEY (id);


--
-- Name: legislative_initiative_summaries legislative_initiative_summaries_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legislative_initiative_summaries
    ADD CONSTRAINT legislative_initiative_summaries_pkey PRIMARY KEY (id);


--
-- Name: legislative_initiatives legislative_initiatives_inr_ityp_gp_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legislative_initiatives
    ADD CONSTRAINT legislative_initiatives_inr_ityp_gp_key UNIQUE (inr, ityp, gp);


--
-- Name: legislative_initiatives legislative_initiatives_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legislative_initiatives
    ADD CONSTRAINT legislative_initiatives_pkey PRIMARY KEY (id);


--
-- Name: mandates mandates_delegate_id_name_start_date_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.mandates
    ADD CONSTRAINT mandates_delegate_id_name_start_date_key UNIQUE (delegate_id, name, start_date);


--
-- Name: mandates mandates_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.mandates
    ADD CONSTRAINT mandates_pkey PRIMARY KEY (id);


--
-- Name: ministerial_proposal_summaries ministerial_proposal_summaries_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministerial_proposal_summaries
    ADD CONSTRAINT ministerial_proposal_summaries_pkey PRIMARY KEY (id);


--
-- Name: ministrial_decrees_documents ministrial_decrees_documents_ministrial_decree_id_document__key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_decrees_documents
    ADD CONSTRAINT ministrial_decrees_documents_ministrial_decree_id_document__key UNIQUE (ministrial_decree_id, document_url);


--
-- Name: ministrial_decrees_documents ministrial_decrees_documents_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_decrees_documents
    ADD CONSTRAINT ministrial_decrees_documents_pkey PRIMARY KEY (id);


--
-- Name: ministrial_decrees ministrial_decrees_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_decrees
    ADD CONSTRAINT ministrial_decrees_pkey PRIMARY KEY (id);


--
-- Name: ministrial_decrees ministrial_decrees_ris_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_decrees
    ADD CONSTRAINT ministrial_decrees_ris_id_key UNIQUE (ris_id);


--
-- Name: ministrial_email_info ministrial_email_info_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_email_info
    ADD CONSTRAINT ministrial_email_info_pkey PRIMARY KEY (id);


--
-- Name: ministrial_issuer ministrial_issuer_delegate_id_ministrial_proposal_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_issuer
    ADD CONSTRAINT ministrial_issuer_delegate_id_ministrial_proposal_id_key UNIQUE (delegate_id, ministrial_proposal_id);


--
-- Name: ministrial_issuer ministrial_issuer_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_issuer
    ADD CONSTRAINT ministrial_issuer_pkey PRIMARY KEY (id);


--
-- Name: ministrial_proposals_documents ministrial_proposals_document_ministrial_proposal_id_docume_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_proposals_documents
    ADD CONSTRAINT ministrial_proposals_document_ministrial_proposal_id_docume_key UNIQUE (ministrial_proposal_id, document_url);


--
-- Name: ministrial_proposals_documents ministrial_proposals_documents_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_proposals_documents
    ADD CONSTRAINT ministrial_proposals_documents_pkey PRIMARY KEY (id);


--
-- Name: ministrial_proposals ministrial_proposals_inr_ityp_gp_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_proposals
    ADD CONSTRAINT ministrial_proposals_inr_ityp_gp_key UNIQUE (inr, ityp, gp);


--
-- Name: ministrial_proposals ministrial_proposals_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_proposals
    ADD CONSTRAINT ministrial_proposals_pkey PRIMARY KEY (id);


--
-- Name: named_vote_info named_vote_info_legis_init_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.named_vote_info
    ADD CONSTRAINT named_vote_info_legis_init_id_key UNIQUE (legis_init_id);


--
-- Name: named_vote_info named_vote_info_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.named_vote_info
    ADD CONSTRAINT named_vote_info_pkey PRIMARY KEY (id);


--
-- Name: named_votes named_votes_delegate_id_named_vote_info_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.named_votes
    ADD CONSTRAINT named_votes_delegate_id_named_vote_info_id_key UNIQUE (delegate_id, named_vote_info_id);


--
-- Name: named_votes named_votes_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.named_votes
    ADD CONSTRAINT named_votes_pkey PRIMARY KEY (id);


--
-- Name: other_keyword_topics_legis_init other_keyword_topics_legis_in_legislative_initiatives_id_to_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.other_keyword_topics_legis_init
    ADD CONSTRAINT other_keyword_topics_legis_in_legislative_initiatives_id_to_key UNIQUE (legislative_initiatives_id, topic);


--
-- Name: other_keyword_topics_legis_init other_keyword_topics_legis_init_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.other_keyword_topics_legis_init
    ADD CONSTRAINT other_keyword_topics_legis_init_pkey PRIMARY KEY (id);


--
-- Name: other_keyword_topics_ministrial_proposals other_keyword_topics_ministria_ministrial_proposal_id_topic_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.other_keyword_topics_ministrial_proposals
    ADD CONSTRAINT other_keyword_topics_ministria_ministrial_proposal_id_topic_key UNIQUE (ministrial_proposal_id, topic);


--
-- Name: other_keyword_topics_ministrial_proposals other_keyword_topics_ministrial_proposals_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.other_keyword_topics_ministrial_proposals
    ADD CONSTRAINT other_keyword_topics_ministrial_proposals_pkey PRIMARY KEY (id);


--
-- Name: other_keyword_topics_proposals other_keyword_topics_proposals_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.other_keyword_topics_proposals
    ADD CONSTRAINT other_keyword_topics_proposals_pkey PRIMARY KEY (id);


--
-- Name: other_keyword_topics_proposals other_keyword_topics_proposals_proposals_id_topic_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.other_keyword_topics_proposals
    ADD CONSTRAINT other_keyword_topics_proposals_proposals_id_topic_key UNIQUE (proposals_id, topic);


--
-- Name: parties parties_code_gp_name_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.parties
    ADD CONSTRAINT parties_code_gp_name_key UNIQUE (code, gp, name);


--
-- Name: parties parties_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.parties
    ADD CONSTRAINT parties_pkey PRIMARY KEY (id);


--
-- Name: plenar_infos plenar_infos_inr_legislative_period_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_infos
    ADD CONSTRAINT plenar_infos_inr_legislative_period_key UNIQUE (inr, legislative_period);


--
-- Name: plenar_infos plenar_infos_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_infos
    ADD CONSTRAINT plenar_infos_pkey PRIMARY KEY (id);


--
-- Name: plenar_speech_legis_inits plenar_speech_legis_inits_gp_ityp_inr_speech_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_speech_legis_inits
    ADD CONSTRAINT plenar_speech_legis_inits_gp_ityp_inr_speech_id_key UNIQUE (gp, ityp, inr, speech_id);


--
-- Name: plenar_speech_legis_inits plenar_speech_legis_inits_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_speech_legis_inits
    ADD CONSTRAINT plenar_speech_legis_inits_pkey PRIMARY KEY (id);


--
-- Name: plenar_speech_links plenar_speech_links_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_speech_links
    ADD CONSTRAINT plenar_speech_links_pkey PRIMARY KEY (id);


--
-- Name: plenar_speech_links plenar_speech_links_plenar_speech_id_document_url_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_speech_links
    ADD CONSTRAINT plenar_speech_links_plenar_speech_id_document_url_key UNIQUE (plenar_speech_id, document_url);


--
-- Name: plenar_speeches plenar_speeches_delegate_id_debate_id_duration_in_seconds_t_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_speeches
    ADD CONSTRAINT plenar_speeches_delegate_id_debate_id_duration_in_seconds_t_key UNIQUE (delegate_id, debate_id, duration_in_seconds, ty);


--
-- Name: plenar_speeches plenar_speeches_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_speeches
    ADD CONSTRAINT plenar_speeches_pkey PRIMARY KEY (id);


--
-- Name: political_answers political_answers_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_answers
    ADD CONSTRAINT political_answers_pkey PRIMARY KEY (id);


--
-- Name: political_opinions political_opinions_delegate_id_question_id_answer_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_opinions
    ADD CONSTRAINT political_opinions_delegate_id_question_id_answer_id_key UNIQUE (delegate_id, question_id, answer_id);


--
-- Name: political_opinions political_opinions_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_opinions
    ADD CONSTRAINT political_opinions_pkey PRIMARY KEY (id);


--
-- Name: political_positions political_positions_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_positions
    ADD CONSTRAINT political_positions_pkey PRIMARY KEY (id);


--
-- Name: political_questions_detailed_topics_influence political_questions_detailed_topics_influ_question_id_topic_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_detailed_topics_influence
    ADD CONSTRAINT political_questions_detailed_topics_influ_question_id_topic_key UNIQUE (question_id, topic);


--
-- Name: political_questions_detailed_topics_influence political_questions_detailed_topics_influence_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_detailed_topics_influence
    ADD CONSTRAINT political_questions_detailed_topics_influence_pkey PRIMARY KEY (id);


--
-- Name: political_questions_detailed_topics political_questions_detailed_topics_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_detailed_topics
    ADD CONSTRAINT political_questions_detailed_topics_pkey PRIMARY KEY (id);


--
-- Name: political_questions_detailed_topics political_questions_detailed_topics_question_id_topic_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_detailed_topics
    ADD CONSTRAINT political_questions_detailed_topics_question_id_topic_key UNIQUE (question_id, topic);


--
-- Name: political_questions political_questions_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions
    ADD CONSTRAINT political_questions_pkey PRIMARY KEY (id);


--
-- Name: political_questions political_questions_question_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions
    ADD CONSTRAINT political_questions_question_key UNIQUE (question);


--
-- Name: political_questions_topics_influence political_questions_topics_influence_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_topics_influence
    ADD CONSTRAINT political_questions_topics_influence_pkey PRIMARY KEY (id);


--
-- Name: political_questions_topics_influence political_questions_topics_influence_question_id_topic_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_topics_influence
    ADD CONSTRAINT political_questions_topics_influence_question_id_topic_key UNIQUE (question_id, topic);


--
-- Name: political_questions_topics political_questions_topics_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_topics
    ADD CONSTRAINT political_questions_topics_pkey PRIMARY KEY (id);


--
-- Name: political_questions_topics political_questions_topics_question_id_topic_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_topics
    ADD CONSTRAINT political_questions_topics_question_id_topic_key UNIQUE (question_id, topic);


--
-- Name: proposal_delegates proposal_delegates_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.proposal_delegates
    ADD CONSTRAINT proposal_delegates_pkey PRIMARY KEY (proposal_id, delegate_id);


--
-- Name: proposal_documents proposal_documents_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.proposal_documents
    ADD CONSTRAINT proposal_documents_pkey PRIMARY KEY (id);


--
-- Name: proposal_documents proposal_documents_proposal_id_document_url_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.proposal_documents
    ADD CONSTRAINT proposal_documents_proposal_id_document_url_key UNIQUE (proposal_id, document_url);


--
-- Name: proposal_email_info proposal_email_info_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.proposal_email_info
    ADD CONSTRAINT proposal_email_info_pkey PRIMARY KEY (id);


--
-- Name: proposals proposals_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.proposals
    ADD CONSTRAINT proposals_pkey PRIMARY KEY (id);


--
-- Name: questions questions_issuer_id_delegate_id_title_body_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.questions
    ADD CONSTRAINT questions_issuer_id_delegate_id_title_body_key UNIQUE (issuer_id, delegate_id, title, body);


--
-- Name: questions questions_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.questions
    ADD CONSTRAINT questions_pkey PRIMARY KEY (id);


--
-- Name: quiz quiz_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.quiz
    ADD CONSTRAINT quiz_pkey PRIMARY KEY (id);


--
-- Name: quiz_questions quiz_questions_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.quiz_questions
    ADD CONSTRAINT quiz_questions_pkey PRIMARY KEY (id);


--
-- Name: quiz_questions quiz_questions_quiz_id_question_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.quiz_questions
    ADD CONSTRAINT quiz_questions_quiz_id_question_key UNIQUE (quiz_id, question);


--
-- Name: quiz quiz_title_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.quiz
    ADD CONSTRAINT quiz_title_key UNIQUE (title);


--
-- Name: seat_history seat_history_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.seat_history
    ADD CONSTRAINT seat_history_pkey PRIMARY KEY (id);


--
-- Name: seat_history seat_history_unique_delegate_seat; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.seat_history
    ADD CONSTRAINT seat_history_unique_delegate_seat UNIQUE (delegate_id, seat_col, seat_row, council, gp);


--
-- Name: somes_user somes_user_email_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.somes_user
    ADD CONSTRAINT somes_user_email_key UNIQUE (email);


--
-- Name: somes_user somes_user_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.somes_user
    ADD CONSTRAINT somes_user_pkey PRIMARY KEY (id);


--
-- Name: speech_complexity speech_complexity_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speech_complexity
    ADD CONSTRAINT speech_complexity_pkey PRIMARY KEY (id);


--
-- Name: speech_complexity speech_complexity_speech_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speech_complexity
    ADD CONSTRAINT speech_complexity_speech_id_key UNIQUE (speech_id);


--
-- Name: speech_summaries speech_summaries_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speech_summaries
    ADD CONSTRAINT speech_summaries_pkey PRIMARY KEY (id);


--
-- Name: speeches speeches_delegate_id_legislative_initiatives_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speeches
    ADD CONSTRAINT speeches_delegate_id_legislative_initiatives_id_key UNIQUE (delegate_id, legislative_initiatives_id);


--
-- Name: speeches_html_urls speeches_html_urls_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speeches_html_urls
    ADD CONSTRAINT speeches_html_urls_pkey PRIMARY KEY (id);


--
-- Name: speeches_html_urls speeches_html_urls_speech_id_document_url_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speeches_html_urls
    ADD CONSTRAINT speeches_html_urls_speech_id_document_url_key UNIQUE (speech_id, document_url);


--
-- Name: speeches speeches_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speeches
    ADD CONSTRAINT speeches_pkey PRIMARY KEY (id);


--
-- Name: stance_citations stance_citations_answer_id_point_uuid_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.stance_citations
    ADD CONSTRAINT stance_citations_answer_id_point_uuid_key UNIQUE (answer_id, point_uuid);


--
-- Name: stance_citations stance_citations_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.stance_citations
    ADD CONSTRAINT stance_citations_pkey PRIMARY KEY (id);


--
-- Name: summaries summaries_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.summaries
    ADD CONSTRAINT summaries_pkey PRIMARY KEY (id);


--
-- Name: topics_legis_init topics_legis_init_legislative_initiatives_id_topic_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.topics_legis_init
    ADD CONSTRAINT topics_legis_init_legislative_initiatives_id_topic_key UNIQUE (legislative_initiatives_id, topic);


--
-- Name: topics_legis_init topics_legis_init_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.topics_legis_init
    ADD CONSTRAINT topics_legis_init_pkey PRIMARY KEY (id);


--
-- Name: topics_ministrial_proposals topics_ministrial_proposals_ministrial_proposal_id_topic_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.topics_ministrial_proposals
    ADD CONSTRAINT topics_ministrial_proposals_ministrial_proposal_id_topic_key UNIQUE (ministrial_proposal_id, topic);


--
-- Name: topics_ministrial_proposals topics_ministrial_proposals_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.topics_ministrial_proposals
    ADD CONSTRAINT topics_ministrial_proposals_pkey PRIMARY KEY (id);


--
-- Name: topics_proposals topics_proposals_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.topics_proposals
    ADD CONSTRAINT topics_proposals_pkey PRIMARY KEY (id);


--
-- Name: topics_proposals topics_proposals_proposals_id_topic_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.topics_proposals
    ADD CONSTRAINT topics_proposals_proposals_id_topic_key UNIQUE (proposals_id, topic);


--
-- Name: transcriptionsqa transcriptionsqa_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.transcriptionsqa
    ADD CONSTRAINT transcriptionsqa_pkey PRIMARY KEY (id);


--
-- Name: transcriptionsqa transcriptionsqa_question_answer_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.transcriptionsqa
    ADD CONSTRAINT transcriptionsqa_question_answer_key UNIQUE (question, answer);


--
-- Name: transcriptionsqa transcriptionsqa_question_transcription_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.transcriptionsqa
    ADD CONSTRAINT transcriptionsqa_question_transcription_id_key UNIQUE (question, transcription_id);


--
-- Name: political_positions unique_del; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_positions
    ADD CONSTRAINT unique_del UNIQUE (delegate_id);


--
-- Name: unique_eurovoc_topics unique_topics_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.unique_eurovoc_topics
    ADD CONSTRAINT unique_topics_pkey PRIMARY KEY (id);


--
-- Name: unique_topics unique_topics_pkey1; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.unique_topics
    ADD CONSTRAINT unique_topics_pkey1 PRIMARY KEY (id);


--
-- Name: unique_eurovoc_topics unique_topics_topic_name_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.unique_eurovoc_topics
    ADD CONSTRAINT unique_topics_topic_name_key UNIQUE (topic_name);


--
-- Name: unique_topics unique_topics_topic_name_key1; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.unique_topics
    ADD CONSTRAINT unique_topics_topic_name_key1 UNIQUE (topic_name);


--
-- Name: user_topics user_topics_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.user_topics
    ADD CONSTRAINT user_topics_pkey PRIMARY KEY (id);


--
-- Name: user_topics user_topics_user_id_topic_id_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.user_topics
    ADD CONSTRAINT user_topics_user_id_topic_id_key UNIQUE (user_id, topic_id);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users users_username_key; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_username_key UNIQUE (username);


--
-- Name: votes votes_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.votes
    ADD CONSTRAINT votes_pkey PRIMARY KEY (party, legislative_initiatives_id);


--
-- Name: walo walo_pkey; Type: CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.walo
    ADD CONSTRAINT walo_pkey PRIMARY KEY (id);


--
-- Name: idx_absences_delegate_plenary; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_absences_delegate_plenary ON public.absences USING btree (delegate_id, plenary_session_id);


--
-- Name: idx_ai_summary_lookup; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_ai_summary_lookup ON public.legislative_initiative_summaries USING btree (legis_init_id);


--
-- Name: idx_debates_plenar_id; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_debates_plenar_id ON public.debates USING btree (plenar_id, id);


--
-- Name: idx_delegates_with_mandates_id; Type: INDEX; Schema: public; Owner: fabi
--

CREATE UNIQUE INDEX idx_delegates_with_mandates_id ON public.delegates_with_mandates USING btree (id);


--
-- Name: idx_eurovoc_legis_id; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_eurovoc_legis_id ON public.eurovoc_topics_legis_init USING btree (legislative_initiatives_id);


--
-- Name: idx_legis_docs_legis_id; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_legis_docs_legis_id ON public.legislative_documents USING btree (legislative_initiatives_id);


--
-- Name: idx_legis_inits_refs_origin; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_legis_inits_refs_origin ON public.legis_inits_refs USING btree (origin_legis_init_id);


--
-- Name: idx_mandates_delegate_id_dates; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_mandates_delegate_id_dates ON public.mandates USING btree (delegate_id, start_date, end_date);


--
-- Name: idx_named_vote_info_legis_id; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_named_vote_info_legis_id ON public.named_vote_info USING btree (legis_init_id);


--
-- Name: idx_other_kw_legis_id; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_other_kw_legis_id ON public.other_keyword_topics_legis_init USING btree (legislative_initiatives_id);


--
-- Name: idx_plenar_infos_id_add_date; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_plenar_infos_id_add_date ON public.plenar_infos USING btree (id, raw_data_created_at);


--
-- Name: idx_plenar_speech_legis_id; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_plenar_speech_legis_id ON public.plenar_speech_legis_inits USING btree (legis_init_id);


--
-- Name: idx_plenar_speeches_delegate_debate; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_plenar_speeches_delegate_debate ON public.plenar_speeches USING btree (delegate_id, debate_id);


--
-- Name: idx_speech_complexity_speech_id; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_speech_complexity_speech_id ON public.speech_complexity USING btree (speech_id);


--
-- Name: idx_summaries_generated_at; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_summaries_generated_at ON public.summaries USING btree (id, generated_at DESC);


--
-- Name: idx_topics_legis_id; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX idx_topics_legis_id ON public.topics_legis_init USING btree (legislative_initiatives_id);


--
-- Name: index_ref_gp; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX index_ref_gp ON public.legis_inits_refs USING btree (ref_gp, ref_ityp, ref_inr);


--
-- Name: latest_legislative_initiatives_uidx; Type: INDEX; Schema: public; Owner: fabi
--

CREATE UNIQUE INDEX latest_legislative_initiatives_uidx ON public.latest_legislative_initiatives USING btree (id);


--
-- Name: legislative_initiatives_with_votes_uidx; Type: INDEX; Schema: public; Owner: fabi
--

CREATE UNIQUE INDEX legislative_initiatives_with_votes_uidx ON public.legislative_initiatives_with_votes USING btree (id);


--
-- Name: plenar_speeches_spoken_text_hash_idx; Type: INDEX; Schema: public; Owner: fabi
--

CREATE INDEX plenar_speeches_spoken_text_hash_idx ON public.plenar_speeches USING hash (spoken_text);


--
-- Name: absences absences_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.absences
    ADD CONSTRAINT absences_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: absences absences_plenary_session_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.absences
    ADD CONSTRAINT absences_plenary_session_id_fkey FOREIGN KEY (plenary_session_id) REFERENCES public.plenar_infos(id);


--
-- Name: answers answers_question_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.answers
    ADD CONSTRAINT answers_question_id_fkey FOREIGN KEY (question_id) REFERENCES public.questions(id);


--
-- Name: call_to_order call_to_order_plenar_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.call_to_order
    ADD CONSTRAINT call_to_order_plenar_id_fkey FOREIGN KEY (plenar_id) REFERENCES public.plenar_infos(id);


--
-- Name: call_to_order call_to_order_receiver_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.call_to_order
    ADD CONSTRAINT call_to_order_receiver_id_fkey FOREIGN KEY (receiver_id) REFERENCES public.delegates(id);


--
-- Name: dates_topics dates_topics_date_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.dates_topics
    ADD CONSTRAINT dates_topics_date_id_fkey FOREIGN KEY (date_id) REFERENCES public.dates(id) ON DELETE CASCADE;


--
-- Name: debates debates_plenar_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.debates
    ADD CONSTRAINT debates_plenar_id_fkey FOREIGN KEY (plenar_id) REFERENCES public.plenar_infos(id);


--
-- Name: decree_email_info decree_email_info_already_sent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.decree_email_info
    ADD CONSTRAINT decree_email_info_already_sent_id_fkey FOREIGN KEY (already_sent_id) REFERENCES public.ministrial_decrees(id) ON DELETE CASCADE;


--
-- Name: decree_summaries decree_summaries_decree_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.decree_summaries
    ADD CONSTRAINT decree_summaries_decree_id_fkey FOREIGN KEY (decree_id) REFERENCES public.ministrial_decrees(id) ON DELETE CASCADE;


--
-- Name: decree_summaries decree_summaries_summary_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.decree_summaries
    ADD CONSTRAINT decree_summaries_summary_id_fkey FOREIGN KEY (summary_id) REFERENCES public.summaries(id) ON DELETE CASCADE;


--
-- Name: delegate_ages delegate_ages_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.delegate_ages
    ADD CONSTRAINT delegate_ages_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id) ON DELETE CASCADE;


--
-- Name: delegates_divisions delegates_divisions_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.delegates_divisions
    ADD CONSTRAINT delegates_divisions_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: delegates delegates_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.delegates
    ADD CONSTRAINT delegates_id_fkey FOREIGN KEY (id) REFERENCES public.contacts(id);


--
-- Name: eurovoc_topics_legis_init eurovoc_topics_legis_init_legislative_initiatives_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.eurovoc_topics_legis_init
    ADD CONSTRAINT eurovoc_topics_legis_init_legislative_initiatives_id_fkey FOREIGN KEY (legislative_initiatives_id) REFERENCES public.legislative_initiatives(id);


--
-- Name: eurovoc_topics_ministrial_proposals eurovoc_topics_ministrial_proposals_ministrial_proposal_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.eurovoc_topics_ministrial_proposals
    ADD CONSTRAINT eurovoc_topics_ministrial_proposals_ministrial_proposal_id_fkey FOREIGN KEY (ministrial_proposal_id) REFERENCES public.ministrial_proposals(id);


--
-- Name: eurovoc_topics_proposals eurovoc_topics_proposals_proposals_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.eurovoc_topics_proposals
    ADD CONSTRAINT eurovoc_topics_proposals_proposals_id_fkey FOREIGN KEY (proposals_id) REFERENCES public.proposals(id);


--
-- Name: favo_dels favo_dels_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.favo_dels
    ADD CONSTRAINT favo_dels_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id) ON DELETE CASCADE;


--
-- Name: favo_dels favo_dels_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.favo_dels
    ADD CONSTRAINT favo_dels_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.somes_user(id) ON DELETE CASCADE;


--
-- Name: favo_legis_inits favo_legis_inits_legis_init_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.favo_legis_inits
    ADD CONSTRAINT favo_legis_inits_legis_init_id_fkey FOREIGN KEY (legis_init_id) REFERENCES public.legislative_initiatives(id) ON DELETE CASCADE;


--
-- Name: favo_legis_inits favo_legis_inits_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.favo_legis_inits
    ADD CONSTRAINT favo_legis_inits_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.somes_user(id) ON DELETE CASCADE;


--
-- Name: division_interest_score fk_delegate; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.division_interest_score
    ADD CONSTRAINT fk_delegate FOREIGN KEY (delegate_id) REFERENCES public.delegates(id) ON DELETE CASCADE;


--
-- Name: generated_eurovoc_topics generated_eurovoc_topics_summary_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.generated_eurovoc_topics
    ADD CONSTRAINT generated_eurovoc_topics_summary_id_fkey FOREIGN KEY (summary_id) REFERENCES public.summaries(id) ON DELETE CASCADE;


--
-- Name: interjections interjections_delegate_matching_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.interjections
    ADD CONSTRAINT interjections_delegate_matching_id_fkey FOREIGN KEY (delegate_matching_id) REFERENCES public.delegate_matching(id);


--
-- Name: interjections interjections_interjector_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.interjections
    ADD CONSTRAINT interjections_interjector_delegate_id_fkey FOREIGN KEY (interjector_delegate_id) REFERENCES public.delegates(id);


--
-- Name: interjections interjections_plenar_speech_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.interjections
    ADD CONSTRAINT interjections_plenar_speech_id_fkey FOREIGN KEY (plenar_speech_id) REFERENCES public.plenar_speeches(id);


--
-- Name: introduction_transcriptions introduction_transcriptions_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.introduction_transcriptions
    ADD CONSTRAINT introduction_transcriptions_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: last_vector_speech_update last_vector_speech_update_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.last_vector_speech_update
    ADD CONSTRAINT last_vector_speech_update_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: legis_email_info legis_email_info_already_sent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_email_info
    ADD CONSTRAINT legis_email_info_already_sent_id_fkey FOREIGN KEY (already_sent_id) REFERENCES public.legislative_initiatives(id) ON DELETE CASCADE;


--
-- Name: legis_init_delegates legis_init_delegates_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_init_delegates
    ADD CONSTRAINT legis_init_delegates_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: legis_init_delegates legis_init_delegates_legis_init_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_init_delegates
    ADD CONSTRAINT legis_init_delegates_legis_init_id_fkey FOREIGN KEY (legis_init_id) REFERENCES public.legislative_initiatives(id);


--
-- Name: legis_init_was_updated legis_init_was_updated_legis_init_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_init_was_updated
    ADD CONSTRAINT legis_init_was_updated_legis_init_id_fkey FOREIGN KEY (legis_init_id) REFERENCES public.legislative_initiatives(id);


--
-- Name: legis_inits_refs legis_inits_refs_origin_legis_init_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legis_inits_refs
    ADD CONSTRAINT legis_inits_refs_origin_legis_init_id_fkey FOREIGN KEY (origin_legis_init_id) REFERENCES public.legislative_initiatives(id);


--
-- Name: legislative_documents legislative_documents_legislative_initiatives_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legislative_documents
    ADD CONSTRAINT legislative_documents_legislative_initiatives_id_fkey FOREIGN KEY (legislative_initiatives_id) REFERENCES public.legislative_initiatives(id);


--
-- Name: legislative_initiative_summaries legislative_initiative_summaries_legis_init_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legislative_initiative_summaries
    ADD CONSTRAINT legislative_initiative_summaries_legis_init_id_fkey FOREIGN KEY (legis_init_id) REFERENCES public.legislative_initiatives(id) ON DELETE CASCADE;


--
-- Name: legislative_initiative_summaries legislative_initiative_summaries_summary_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legislative_initiative_summaries
    ADD CONSTRAINT legislative_initiative_summaries_summary_id_fkey FOREIGN KEY (summary_id) REFERENCES public.summaries(id) ON DELETE CASCADE;


--
-- Name: legislative_initiatives legislative_initiatives_plenary_session_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.legislative_initiatives
    ADD CONSTRAINT legislative_initiatives_plenary_session_id_fkey FOREIGN KEY (plenary_session_id) REFERENCES public.plenar_infos(id);


--
-- Name: mandates mandates_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.mandates
    ADD CONSTRAINT mandates_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: ministerial_proposal_summaries ministerial_proposal_summaries_ministerial_proposal_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministerial_proposal_summaries
    ADD CONSTRAINT ministerial_proposal_summaries_ministerial_proposal_id_fkey FOREIGN KEY (ministerial_proposal_id) REFERENCES public.ministrial_proposals(id) ON DELETE CASCADE;


--
-- Name: ministerial_proposal_summaries ministerial_proposal_summaries_summary_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministerial_proposal_summaries
    ADD CONSTRAINT ministerial_proposal_summaries_summary_id_fkey FOREIGN KEY (summary_id) REFERENCES public.summaries(id) ON DELETE CASCADE;


--
-- Name: ministrial_decrees_documents ministrial_decrees_documents_ministrial_decree_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_decrees_documents
    ADD CONSTRAINT ministrial_decrees_documents_ministrial_decree_id_fkey FOREIGN KEY (ministrial_decree_id) REFERENCES public.ministrial_decrees(id);


--
-- Name: ministrial_decrees ministrial_decrees_gov_official_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_decrees
    ADD CONSTRAINT ministrial_decrees_gov_official_id_fkey FOREIGN KEY (gov_official_id) REFERENCES public.delegates(id);


--
-- Name: ministrial_email_info ministrial_email_info_already_sent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_email_info
    ADD CONSTRAINT ministrial_email_info_already_sent_id_fkey FOREIGN KEY (already_sent_id) REFERENCES public.ministrial_proposals(id) ON DELETE CASCADE;


--
-- Name: ministrial_issuer ministrial_issuer_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_issuer
    ADD CONSTRAINT ministrial_issuer_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: ministrial_issuer ministrial_issuer_ministrial_proposal_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_issuer
    ADD CONSTRAINT ministrial_issuer_ministrial_proposal_id_fkey FOREIGN KEY (ministrial_proposal_id) REFERENCES public.ministrial_proposals(id);


--
-- Name: ministrial_proposals_documents ministrial_proposals_documents_ministrial_proposal_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.ministrial_proposals_documents
    ADD CONSTRAINT ministrial_proposals_documents_ministrial_proposal_id_fkey FOREIGN KEY (ministrial_proposal_id) REFERENCES public.ministrial_proposals(id);


--
-- Name: named_vote_info named_vote_info_legis_init_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.named_vote_info
    ADD CONSTRAINT named_vote_info_legis_init_id_fkey FOREIGN KEY (legis_init_id) REFERENCES public.legislative_initiatives(id);


--
-- Name: named_votes named_votes_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.named_votes
    ADD CONSTRAINT named_votes_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: named_votes named_votes_named_vote_info_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.named_votes
    ADD CONSTRAINT named_votes_named_vote_info_id_fkey FOREIGN KEY (named_vote_info_id) REFERENCES public.named_vote_info(id);


--
-- Name: other_keyword_topics_legis_init other_keyword_topics_legis_init_legislative_initiatives_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.other_keyword_topics_legis_init
    ADD CONSTRAINT other_keyword_topics_legis_init_legislative_initiatives_id_fkey FOREIGN KEY (legislative_initiatives_id) REFERENCES public.legislative_initiatives(id);


--
-- Name: other_keyword_topics_ministrial_proposals other_keyword_topics_ministrial_pro_ministrial_proposal_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.other_keyword_topics_ministrial_proposals
    ADD CONSTRAINT other_keyword_topics_ministrial_pro_ministrial_proposal_id_fkey FOREIGN KEY (ministrial_proposal_id) REFERENCES public.ministrial_proposals(id);


--
-- Name: other_keyword_topics_proposals other_keyword_topics_proposals_proposals_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.other_keyword_topics_proposals
    ADD CONSTRAINT other_keyword_topics_proposals_proposals_id_fkey FOREIGN KEY (proposals_id) REFERENCES public.proposals(id);


--
-- Name: plenar_speech_legis_inits plenar_speech_legis_inits_legis_init_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_speech_legis_inits
    ADD CONSTRAINT plenar_speech_legis_inits_legis_init_id_fkey FOREIGN KEY (legis_init_id) REFERENCES public.legislative_initiatives(id);


--
-- Name: plenar_speech_legis_inits plenar_speech_legis_inits_speech_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_speech_legis_inits
    ADD CONSTRAINT plenar_speech_legis_inits_speech_id_fkey FOREIGN KEY (speech_id) REFERENCES public.plenar_speeches(id);


--
-- Name: plenar_speeches plenar_speeches_debate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_speeches
    ADD CONSTRAINT plenar_speeches_debate_id_fkey FOREIGN KEY (debate_id) REFERENCES public.debates(id);


--
-- Name: plenar_speeches plenar_speeches_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.plenar_speeches
    ADD CONSTRAINT plenar_speeches_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: political_answers political_answers_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_answers
    ADD CONSTRAINT political_answers_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: political_answers political_answers_question_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_answers
    ADD CONSTRAINT political_answers_question_id_fkey FOREIGN KEY (question_id) REFERENCES public.political_questions(id);


--
-- Name: political_opinions political_opinions_answer_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_opinions
    ADD CONSTRAINT political_opinions_answer_id_fkey FOREIGN KEY (answer_id) REFERENCES public.political_answers(id);


--
-- Name: political_opinions political_opinions_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_opinions
    ADD CONSTRAINT political_opinions_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: political_opinions political_opinions_question_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_opinions
    ADD CONSTRAINT political_opinions_question_id_fkey FOREIGN KEY (question_id) REFERENCES public.political_questions(id);


--
-- Name: political_positions political_positions_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_positions
    ADD CONSTRAINT political_positions_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: political_questions_detailed_topics_influence political_questions_detailed_topics_influence_question_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_detailed_topics_influence
    ADD CONSTRAINT political_questions_detailed_topics_influence_question_id_fkey FOREIGN KEY (question_id) REFERENCES public.political_questions(id);


--
-- Name: political_questions_detailed_topics political_questions_detailed_topics_question_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_detailed_topics
    ADD CONSTRAINT political_questions_detailed_topics_question_id_fkey FOREIGN KEY (question_id) REFERENCES public.political_questions(id);


--
-- Name: political_questions_topics_influence political_questions_topics_influence_question_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_topics_influence
    ADD CONSTRAINT political_questions_topics_influence_question_id_fkey FOREIGN KEY (question_id) REFERENCES public.political_questions(id);


--
-- Name: political_questions_topics political_questions_topics_question_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.political_questions_topics
    ADD CONSTRAINT political_questions_topics_question_id_fkey FOREIGN KEY (question_id) REFERENCES public.political_questions(id);


--
-- Name: proposal_delegates proposal_delegates_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.proposal_delegates
    ADD CONSTRAINT proposal_delegates_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: proposal_delegates proposal_delegates_proposal_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.proposal_delegates
    ADD CONSTRAINT proposal_delegates_proposal_id_fkey FOREIGN KEY (proposal_id) REFERENCES public.proposals(id);


--
-- Name: proposal_documents proposal_documents_proposal_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.proposal_documents
    ADD CONSTRAINT proposal_documents_proposal_id_fkey FOREIGN KEY (proposal_id) REFERENCES public.proposals(id);


--
-- Name: proposal_email_info proposal_email_info_already_sent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.proposal_email_info
    ADD CONSTRAINT proposal_email_info_already_sent_id_fkey FOREIGN KEY (already_sent_id) REFERENCES public.legislative_initiatives(id) ON DELETE CASCADE;


--
-- Name: questions questions_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.questions
    ADD CONSTRAINT questions_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: questions questions_issuer_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.questions
    ADD CONSTRAINT questions_issuer_id_fkey FOREIGN KEY (issuer_id) REFERENCES public.users(id);


--
-- Name: quiz_questions quiz_questions_quiz_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.quiz_questions
    ADD CONSTRAINT quiz_questions_quiz_id_fkey FOREIGN KEY (quiz_id) REFERENCES public.quiz(id);


--
-- Name: seat_history seat_history_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.seat_history
    ADD CONSTRAINT seat_history_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: speech_complexity speech_complexity_speech_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speech_complexity
    ADD CONSTRAINT speech_complexity_speech_id_fkey FOREIGN KEY (speech_id) REFERENCES public.plenar_speeches(id);


--
-- Name: speech_summaries speech_summaries_speech_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speech_summaries
    ADD CONSTRAINT speech_summaries_speech_id_fkey FOREIGN KEY (speech_id) REFERENCES public.plenar_speeches(id) ON DELETE CASCADE;


--
-- Name: speech_summaries speech_summaries_summary_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speech_summaries
    ADD CONSTRAINT speech_summaries_summary_id_fkey FOREIGN KEY (summary_id) REFERENCES public.summaries(id) ON DELETE CASCADE;


--
-- Name: speeches speeches_delegate_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speeches
    ADD CONSTRAINT speeches_delegate_id_fkey FOREIGN KEY (delegate_id) REFERENCES public.delegates(id);


--
-- Name: speeches_html_urls speeches_html_urls_speech_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speeches_html_urls
    ADD CONSTRAINT speeches_html_urls_speech_id_fkey FOREIGN KEY (speech_id) REFERENCES public.speeches(id);


--
-- Name: speeches speeches_legislative_initiatives_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.speeches
    ADD CONSTRAINT speeches_legislative_initiatives_id_fkey FOREIGN KEY (legislative_initiatives_id) REFERENCES public.legislative_initiatives(id);


--
-- Name: stance_citations stance_citations_answer_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.stance_citations
    ADD CONSTRAINT stance_citations_answer_id_fkey FOREIGN KEY (answer_id) REFERENCES public.political_answers(id);


--
-- Name: stance_citations stance_citations_speech_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.stance_citations
    ADD CONSTRAINT stance_citations_speech_id_fkey FOREIGN KEY (speech_id) REFERENCES public.plenar_speeches(id);


--
-- Name: topics_legis_init topics_legis_init_legislative_initiatives_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.topics_legis_init
    ADD CONSTRAINT topics_legis_init_legislative_initiatives_id_fkey FOREIGN KEY (legislative_initiatives_id) REFERENCES public.legislative_initiatives(id);


--
-- Name: topics_ministrial_proposals topics_ministrial_proposals_ministrial_proposal_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.topics_ministrial_proposals
    ADD CONSTRAINT topics_ministrial_proposals_ministrial_proposal_id_fkey FOREIGN KEY (ministrial_proposal_id) REFERENCES public.ministrial_proposals(id);


--
-- Name: topics_proposals topics_proposals_proposals_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.topics_proposals
    ADD CONSTRAINT topics_proposals_proposals_id_fkey FOREIGN KEY (proposals_id) REFERENCES public.proposals(id);


--
-- Name: transcriptionsqa transcriptionsqa_transcription_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.transcriptionsqa
    ADD CONSTRAINT transcriptionsqa_transcription_id_fkey FOREIGN KEY (transcription_id) REFERENCES public.introduction_transcriptions(id);


--
-- Name: user_topics user_topics_topic_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.user_topics
    ADD CONSTRAINT user_topics_topic_id_fkey FOREIGN KEY (topic_id) REFERENCES public.unique_eurovoc_topics(id);


--
-- Name: user_topics user_topics_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.user_topics
    ADD CONSTRAINT user_topics_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.somes_user(id);


--
-- Name: votes votes_legislative_initiatives_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: fabi
--

ALTER TABLE ONLY public.votes
    ADD CONSTRAINT votes_legislative_initiatives_id_fkey FOREIGN KEY (legislative_initiatives_id) REFERENCES public.legislative_initiatives(id);


--
-- Name: SCHEMA public; Type: ACL; Schema: -; Owner: pg_database_owner
--

REVOKE USAGE ON SCHEMA public FROM PUBLIC;
GRANT ALL ON SCHEMA public TO PUBLIC;


--
-- PostgreSQL database dump complete
--

\unrestrict 22Xa7GDLzHnPbwlhDfvaSkmFFg85IsJ210jIPtz7IyzxYDFdyw1gH7p3TKP82LA

