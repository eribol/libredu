--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1
-- Dumped by pg_dump version 15.1

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
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO postgres;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: postgres
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


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.__diesel_schema_migrations OWNER TO postgres;

--
-- Name: activities; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.activities (
    id integer NOT NULL,
    subject integer,
    hour smallint,
    classes integer[] DEFAULT '{}'::integer[] NOT NULL,
    teachers integer[] DEFAULT ARRAY[]::integer[] NOT NULL,
    partner_activity integer
);


ALTER TABLE public.activities OWNER TO postgres;

--
-- Name: activites_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.activites_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.activites_id_seq OWNER TO postgres;

--
-- Name: activites_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.activites_id_seq OWNED BY public.activities.id;


--
-- Name: auth; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.auth (
    id integer NOT NULL,
    title character varying(50)
);


ALTER TABLE public.auth OWNER TO postgres;

--
-- Name: auth_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.auth_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.auth_id_seq OWNER TO postgres;

--
-- Name: auth_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.auth_id_seq OWNED BY public.auth.id;


--
-- Name: books; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.books (
    id integer NOT NULL,
    library integer NOT NULL,
    name character varying(250) NOT NULL,
    barkod integer NOT NULL,
    piece integer DEFAULT 1 NOT NULL,
    writer character varying(100)
);


ALTER TABLE public.books OWNER TO postgres;

--
-- Name: books_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.books_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.books_id_seq OWNER TO postgres;

--
-- Name: books_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.books_id_seq OWNED BY public.books.id;


--
-- Name: city; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.city (
    pk integer NOT NULL,
    name character varying(150) NOT NULL
);


ALTER TABLE public.city OWNER TO postgres;

--
-- Name: city_pk_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.city_pk_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.city_pk_seq OWNER TO postgres;

--
-- Name: city_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.city_pk_seq OWNED BY public.city.pk;


--
-- Name: class_auths; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.class_auths (
    group_id integer NOT NULL,
    user_id integer,
    class_menu integer NOT NULL,
    rw smallint
);


ALTER TABLE public.class_auths OWNER TO postgres;

--
-- Name: class_available; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.class_available (
    class_id integer NOT NULL,
    day integer NOT NULL,
    hours boolean[] NOT NULL
);


ALTER TABLE public.class_available OWNER TO postgres;

--
-- Name: class_groups; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.class_groups (
    id integer NOT NULL,
    school integer,
    name character varying,
    prime boolean DEFAULT false NOT NULL,
    hour integer DEFAULT 8 NOT NULL
);


ALTER TABLE public.class_groups OWNER TO postgres;

--
-- Name: class_groups_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.class_groups_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.class_groups_id_seq OWNER TO postgres;

--
-- Name: class_groups_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.class_groups_id_seq OWNED BY public.class_groups.id;


--
-- Name: class_menus; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.class_menus (
    id integer NOT NULL,
    title character varying(25),
    link character varying(25)
);


ALTER TABLE public.class_menus OWNER TO postgres;

--
-- Name: class_menus_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.class_menus_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.class_menus_id_seq OWNER TO postgres;

--
-- Name: class_menus_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.class_menus_id_seq OWNED BY public.class_menus.id;


--
-- Name: class_rooms; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.class_rooms (
    id integer NOT NULL,
    name character varying(25) NOT NULL,
    school integer NOT NULL,
    rw smallint NOT NULL,
    cl smallint NOT NULL,
    width smallint NOT NULL
);


ALTER TABLE public.class_rooms OWNER TO postgres;

--
-- Name: class_rooms_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.class_rooms_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.class_rooms_id_seq OWNER TO postgres;

--
-- Name: class_rooms_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.class_rooms_id_seq OWNED BY public.class_rooms.id;


--
-- Name: class_student; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.class_student (
    student integer NOT NULL,
    class_id integer NOT NULL,
    group_id integer NOT NULL
);


ALTER TABLE public.class_student OWNER TO postgres;

--
-- Name: class_timetable; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.class_timetable (
    id integer NOT NULL,
    day_id integer NOT NULL,
    hour smallint NOT NULL,
    activity integer NOT NULL,
    locked boolean DEFAULT false NOT NULL
);


ALTER TABLE public.class_timetable OWNER TO postgres;

--
-- Name: class_timetable_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.class_timetable_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.class_timetable_id_seq OWNER TO postgres;

--
-- Name: class_timetable_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.class_timetable_id_seq OWNED BY public.class_timetable.id;


--
-- Name: classes; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.classes (
    id integer NOT NULL,
    kademe character varying(10) NOT NULL,
    sube character varying(5) NOT NULL,
    school integer NOT NULL,
    teacher integer,
    group_id integer NOT NULL
);


ALTER TABLE public.classes OWNER TO postgres;

--
-- Name: classes_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.classes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.classes_id_seq OWNER TO postgres;

--
-- Name: classes_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.classes_id_seq OWNED BY public.classes.id;


--
-- Name: content_type; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.content_type (
    id integer NOT NULL,
    app_label character varying(100) NOT NULL,
    model character varying(100) NOT NULL
);


ALTER TABLE public.content_type OWNER TO postgres;

--
-- Name: content_type_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.content_type_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.content_type_id_seq OWNER TO postgres;

--
-- Name: content_type_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.content_type_id_seq OWNED BY public.content_type.id;


--
-- Name: days; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.days (
    id integer NOT NULL,
    name character varying(100) NOT NULL
);


ALTER TABLE public.days OWNER TO postgres;

--
-- Name: days_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.days_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.days_id_seq OWNER TO postgres;

--
-- Name: days_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.days_id_seq OWNED BY public.days.id;


--
-- Name: group_auths; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.group_auths (
    group_id integer NOT NULL,
    user_id integer,
    group_menu integer NOT NULL,
    rw smallint
);


ALTER TABLE public.group_auths OWNER TO postgres;

--
-- Name: group_menus; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.group_menus (
    id integer NOT NULL,
    name character varying(25),
    link character varying(25)
);


ALTER TABLE public.group_menus OWNER TO postgres;

--
-- Name: group_menus_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.group_menus_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.group_menus_id_seq OWNER TO postgres;

--
-- Name: group_menus_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.group_menus_id_seq OWNED BY public.group_menus.id;


--
-- Name: group_schedules; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.group_schedules (
    group_id integer NOT NULL,
    hour integer NOT NULL,
    start_time time without time zone NOT NULL,
    end_time time without time zone NOT NULL
);


ALTER TABLE public.group_schedules OWNER TO postgres;

--
-- Name: hours; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.hours (
    id integer NOT NULL
);


ALTER TABLE public.hours OWNER TO postgres;

--
-- Name: hours_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.hours_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.hours_id_seq OWNER TO postgres;

--
-- Name: hours_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.hours_id_seq OWNED BY public.hours.id;


--
-- Name: libraries; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.libraries (
    id integer NOT NULL,
    school integer NOT NULL,
    manager integer NOT NULL,
    student integer NOT NULL,
    barkod_max integer DEFAULT 10000 NOT NULL,
    barkod_min integer DEFAULT 1 NOT NULL
);


ALTER TABLE public.libraries OWNER TO postgres;

--
-- Name: libraries_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.libraries_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.libraries_id_seq OWNER TO postgres;

--
-- Name: libraries_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.libraries_id_seq OWNED BY public.libraries.id;


--
-- Name: post; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.post (
    id integer NOT NULL,
    title character varying(200),
    only_teacher boolean DEFAULT true,
    body text NOT NULL,
    pub_date timestamp with time zone,
    sender integer NOT NULL,
    school integer
);


ALTER TABLE public.post OWNER TO postgres;

--
-- Name: post_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.post_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.post_id_seq OWNER TO postgres;

--
-- Name: post_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.post_id_seq OWNED BY public.post.id;


--
-- Name: roles; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.roles (
    id integer NOT NULL,
    name character varying(100) NOT NULL
);


ALTER TABLE public.roles OWNER TO postgres;

--
-- Name: roles_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.roles_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.roles_id_seq OWNER TO postgres;

--
-- Name: roles_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.roles_id_seq OWNED BY public.roles.id;


--
-- Name: school; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.school (
    id integer NOT NULL,
    name character varying(150),
    pansiyon boolean,
    dersane boolean,
    is_active boolean,
    manager integer,
    town integer,
    school_type integer,
    tel character varying(13),
    location character varying(250)
);


ALTER TABLE public.school OWNER TO postgres;

--
-- Name: school_auths; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.school_auths (
    school_id integer NOT NULL,
    user_id integer,
    school_menu integer NOT NULL,
    rw smallint
);


ALTER TABLE public.school_auths OWNER TO postgres;

--
-- Name: school_code_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.school_code_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.school_code_seq OWNER TO postgres;

--
-- Name: school_code_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.school_code_seq OWNED BY public.school.id;


--
-- Name: school_grades; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.school_grades (
    school_type_id integer,
    name character varying(100)
);


ALTER TABLE public.school_grades OWNER TO postgres;

--
-- Name: school_menus; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.school_menus (
    id integer NOT NULL,
    name character varying(25) NOT NULL,
    link character varying(25),
    school_type integer NOT NULL
);


ALTER TABLE public.school_menus OWNER TO postgres;

--
-- Name: school_menus_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.school_menus_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.school_menus_id_seq OWNER TO postgres;

--
-- Name: school_menus_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.school_menus_id_seq OWNED BY public.school_menus.id;


--
-- Name: school_type; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.school_type (
    name character varying(150) NOT NULL,
    id integer NOT NULL
);


ALTER TABLE public.school_type OWNER TO postgres;

--
-- Name: school_type_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.school_type_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.school_type_id_seq OWNER TO postgres;

--
-- Name: school_type_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.school_type_id_seq OWNED BY public.school_type.id;


--
-- Name: school_users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.school_users (
    user_id integer NOT NULL,
    school_id integer NOT NULL,
    auth integer,
    role integer DEFAULT 3 NOT NULL,
    write boolean DEFAULT false,
    read boolean DEFAULT false
);


ALTER TABLE public.school_users OWNER TO postgres;

--
-- Name: session; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.session (
    user_id integer NOT NULL,
    key character varying NOT NULL
);


ALTER TABLE public.session OWNER TO postgres;

--
-- Name: students; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.students (
    id integer NOT NULL,
    first_name character varying(250) NOT NULL,
    last_name character varying(50) NOT NULL,
    father integer,
    mother integer,
    school integer,
    school_number integer,
    user_id integer NOT NULL
);


ALTER TABLE public.students OWNER TO postgres;

--
-- Name: students_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.students_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.students_id_seq OWNER TO postgres;

--
-- Name: students_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.students_id_seq OWNED BY public.students.id;


--
-- Name: subject_group; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.subject_group (
    id integer NOT NULL,
    name character varying(25)
);


ALTER TABLE public.subject_group OWNER TO postgres;

--
-- Name: subject_group_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.subject_group_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.subject_group_id_seq OWNER TO postgres;

--
-- Name: subject_group_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.subject_group_id_seq OWNED BY public.subject_group.id;


--
-- Name: subjects; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.subjects (
    id integer NOT NULL,
    name character varying(50),
    kademe character varying(10),
    optional boolean DEFAULT false,
    school integer DEFAULT 720917 NOT NULL,
    school_type integer DEFAULT 1 NOT NULL,
    short_name character varying(10) DEFAULT ''::character varying NOT NULL,
    group_id integer
);


ALTER TABLE public.subjects OWNER TO postgres;

--
-- Name: subjects_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.subjects_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.subjects_id_seq OWNER TO postgres;

--
-- Name: subjects_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.subjects_id_seq OWNED BY public.subjects.id;


--
-- Name: teacher_auths; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.teacher_auths (
    school_id integer NOT NULL,
    user_id integer,
    teacher_menu integer NOT NULL,
    rw smallint
);


ALTER TABLE public.teacher_auths OWNER TO postgres;

--
-- Name: teacher_available; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.teacher_available (
    user_id integer NOT NULL,
    school_id integer NOT NULL,
    day integer NOT NULL,
    hours boolean[] NOT NULL,
    group_id integer
);


ALTER TABLE public.teacher_available OWNER TO postgres;

--
-- Name: teacher_menus; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.teacher_menus (
    id integer NOT NULL,
    title character varying(25),
    link character varying(25)
);


ALTER TABLE public.teacher_menus OWNER TO postgres;

--
-- Name: teacher_menus_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.teacher_menus_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.teacher_menus_id_seq OWNER TO postgres;

--
-- Name: teacher_menus_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.teacher_menus_id_seq OWNED BY public.teacher_menus.id;


--
-- Name: teachers_menu; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.teachers_menu (
    id integer NOT NULL,
    title character varying(25),
    link character varying(25)
);


ALTER TABLE public.teachers_menu OWNER TO postgres;

--
-- Name: teachers_menu_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.teachers_menu_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.teachers_menu_id_seq OWNER TO postgres;

--
-- Name: teachers_menu_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.teachers_menu_id_seq OWNED BY public.teachers_menu.id;


--
-- Name: timetables; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.timetables (
    group_id integer,
    pub_date date DEFAULT now() NOT NULL,
    tables json NOT NULL
);


ALTER TABLE public.timetables OWNER TO postgres;

--
-- Name: token_books; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.token_books (
    student integer NOT NULL,
    book integer NOT NULL,
    library integer NOT NULL,
    take_date date DEFAULT CURRENT_DATE NOT NULL,
    id integer NOT NULL
);


ALTER TABLE public.token_books OWNER TO postgres;

--
-- Name: token_books_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.token_books_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.token_books_id_seq OWNER TO postgres;

--
-- Name: token_books_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.token_books_id_seq OWNED BY public.token_books.id;


--
-- Name: town; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.town (
    pk integer NOT NULL,
    name character varying(150) NOT NULL,
    city integer
);


ALTER TABLE public.town OWNER TO postgres;

--
-- Name: town_pk_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.town_pk_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.town_pk_seq OWNER TO postgres;

--
-- Name: town_pk_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.town_pk_seq OWNED BY public.town.pk;


--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.users (
    id integer NOT NULL,
    first_name character varying(150) NOT NULL,
    last_name character varying(150) NOT NULL,
    username character varying(70),
    email character varying(100),
    password character varying(128),
    date_join timestamp without time zone DEFAULT now(),
    last_login timestamp without time zone,
    is_active boolean DEFAULT false,
    is_staff boolean DEFAULT false,
    is_admin boolean DEFAULT false,
    tel character varying(100),
    gender character varying(10),
    img character varying(250),
    key text,
    short_name character varying(10) DEFAULT ''::character varying NOT NULL
);


ALTER TABLE public.users OWNER TO postgres;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.users_id_seq OWNER TO postgres;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: activities id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.activities ALTER COLUMN id SET DEFAULT nextval('public.activites_id_seq'::regclass);


--
-- Name: auth id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.auth ALTER COLUMN id SET DEFAULT nextval('public.auth_id_seq'::regclass);


--
-- Name: books id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.books ALTER COLUMN id SET DEFAULT nextval('public.books_id_seq'::regclass);


--
-- Name: city pk; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.city ALTER COLUMN pk SET DEFAULT nextval('public.city_pk_seq'::regclass);


--
-- Name: class_groups id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_groups ALTER COLUMN id SET DEFAULT nextval('public.class_groups_id_seq'::regclass);


--
-- Name: class_menus id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_menus ALTER COLUMN id SET DEFAULT nextval('public.class_menus_id_seq'::regclass);


--
-- Name: class_rooms id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_rooms ALTER COLUMN id SET DEFAULT nextval('public.class_rooms_id_seq'::regclass);


--
-- Name: class_timetable id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_timetable ALTER COLUMN id SET DEFAULT nextval('public.class_timetable_id_seq'::regclass);


--
-- Name: classes id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.classes ALTER COLUMN id SET DEFAULT nextval('public.classes_id_seq'::regclass);


--
-- Name: content_type id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.content_type ALTER COLUMN id SET DEFAULT nextval('public.content_type_id_seq'::regclass);


--
-- Name: days id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.days ALTER COLUMN id SET DEFAULT nextval('public.days_id_seq'::regclass);


--
-- Name: group_menus id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.group_menus ALTER COLUMN id SET DEFAULT nextval('public.group_menus_id_seq'::regclass);


--
-- Name: hours id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.hours ALTER COLUMN id SET DEFAULT nextval('public.hours_id_seq'::regclass);


--
-- Name: libraries id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.libraries ALTER COLUMN id SET DEFAULT nextval('public.libraries_id_seq'::regclass);


--
-- Name: post id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.post ALTER COLUMN id SET DEFAULT nextval('public.post_id_seq'::regclass);


--
-- Name: roles id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.roles ALTER COLUMN id SET DEFAULT nextval('public.roles_id_seq'::regclass);


--
-- Name: school id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school ALTER COLUMN id SET DEFAULT nextval('public.school_code_seq'::regclass);


--
-- Name: school_menus id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school_menus ALTER COLUMN id SET DEFAULT nextval('public.school_menus_id_seq'::regclass);


--
-- Name: school_type id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school_type ALTER COLUMN id SET DEFAULT nextval('public.school_type_id_seq'::regclass);


--
-- Name: students id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.students ALTER COLUMN id SET DEFAULT nextval('public.students_id_seq'::regclass);


--
-- Name: subject_group id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subject_group ALTER COLUMN id SET DEFAULT nextval('public.subject_group_id_seq'::regclass);


--
-- Name: subjects id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subjects ALTER COLUMN id SET DEFAULT nextval('public.subjects_id_seq'::regclass);


--
-- Name: teacher_menus id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.teacher_menus ALTER COLUMN id SET DEFAULT nextval('public.teacher_menus_id_seq'::regclass);


--
-- Name: teachers_menu id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.teachers_menu ALTER COLUMN id SET DEFAULT nextval('public.teachers_menu_id_seq'::regclass);


--
-- Name: token_books id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.token_books ALTER COLUMN id SET DEFAULT nextval('public.token_books_id_seq'::regclass);


--
-- Name: town pk; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.town ALTER COLUMN pk SET DEFAULT nextval('public.town_pk_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Name: __diesel_schema_migrations __diesel_schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.__diesel_schema_migrations
    ADD CONSTRAINT __diesel_schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: activities activites_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.activities
    ADD CONSTRAINT activites_pkey PRIMARY KEY (id);


--
-- Name: auth auth_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.auth
    ADD CONSTRAINT auth_pkey PRIMARY KEY (id);


--
-- Name: books books_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.books
    ADD CONSTRAINT books_pkey PRIMARY KEY (id);


--
-- Name: city city_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.city
    ADD CONSTRAINT city_pkey PRIMARY KEY (pk);


--
-- Name: class_auths class_auths_class_menu_user_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_auths
    ADD CONSTRAINT class_auths_class_menu_user_id_key UNIQUE (class_menu, user_id);


--
-- Name: class_available class_available_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_available
    ADD CONSTRAINT class_available_pkey PRIMARY KEY (class_id, day);


--
-- Name: class_groups class_groups_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_groups
    ADD CONSTRAINT class_groups_pkey PRIMARY KEY (id);


--
-- Name: class_menus class_menus_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_menus
    ADD CONSTRAINT class_menus_pkey PRIMARY KEY (id);


--
-- Name: class_rooms class_rooms_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_rooms
    ADD CONSTRAINT class_rooms_pkey PRIMARY KEY (id);


--
-- Name: class_student class_student_student_class_id_group_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_student
    ADD CONSTRAINT class_student_student_class_id_group_id_key UNIQUE (student, class_id, group_id);


--
-- Name: class_timetable class_timetable_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_timetable
    ADD CONSTRAINT class_timetable_pkey PRIMARY KEY (id);


--
-- Name: classes classes_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.classes
    ADD CONSTRAINT classes_pkey PRIMARY KEY (id);


--
-- Name: classes classes_school_sube_kademe_group_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.classes
    ADD CONSTRAINT classes_school_sube_kademe_group_id_key UNIQUE (school, sube, kademe, group_id);


--
-- Name: content_type content_type_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.content_type
    ADD CONSTRAINT content_type_pkey PRIMARY KEY (id);


--
-- Name: days days_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.days
    ADD CONSTRAINT days_pkey PRIMARY KEY (id);


--
-- Name: group_auths group_auths_group_id_group_menu_user_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.group_auths
    ADD CONSTRAINT group_auths_group_id_group_menu_user_id_key UNIQUE (group_id, group_menu, user_id);


--
-- Name: group_menus group_menus_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.group_menus
    ADD CONSTRAINT group_menus_pkey PRIMARY KEY (id);


--
-- Name: group_schedules group_schedules_group_id_hour_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.group_schedules
    ADD CONSTRAINT group_schedules_group_id_hour_key UNIQUE (group_id, hour);


--
-- Name: hours hours_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.hours
    ADD CONSTRAINT hours_pkey PRIMARY KEY (id);


--
-- Name: school_users id; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school_users
    ADD CONSTRAINT id PRIMARY KEY (school_id, user_id);


--
-- Name: libraries libraries_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.libraries
    ADD CONSTRAINT libraries_pkey PRIMARY KEY (id);


--
-- Name: post post_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.post
    ADD CONSTRAINT post_pkey PRIMARY KEY (id);


--
-- Name: roles roles_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.roles
    ADD CONSTRAINT roles_pkey PRIMARY KEY (id);


--
-- Name: school_auths school_auths_school_id_school_menu_user_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school_auths
    ADD CONSTRAINT school_auths_school_id_school_menu_user_id_key UNIQUE (school_id, school_menu, user_id);


--
-- Name: libraries school_fkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.libraries
    ADD CONSTRAINT school_fkey UNIQUE (school);


--
-- Name: school_menus school_menus_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school_menus
    ADD CONSTRAINT school_menus_pkey PRIMARY KEY (id);


--
-- Name: school school_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school
    ADD CONSTRAINT school_pkey PRIMARY KEY (id);


--
-- Name: school_type school_type_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school_type
    ADD CONSTRAINT school_type_pkey PRIMARY KEY (id);


--
-- Name: session session_user_id_key_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.session
    ADD CONSTRAINT session_user_id_key_key UNIQUE (user_id, key);


--
-- Name: students students_id_father_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.students
    ADD CONSTRAINT students_id_father_key UNIQUE (id, father);


--
-- Name: students students_id_mother_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.students
    ADD CONSTRAINT students_id_mother_key UNIQUE (id, mother);


--
-- Name: students students_id_school_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.students
    ADD CONSTRAINT students_id_school_key UNIQUE (id, school);


--
-- Name: students students_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.students
    ADD CONSTRAINT students_pkey PRIMARY KEY (id);


--
-- Name: students students_school_school_number_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.students
    ADD CONSTRAINT students_school_school_number_key UNIQUE (school, school_number);


--
-- Name: subject_group subject_group_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subject_group
    ADD CONSTRAINT subject_group_pkey PRIMARY KEY (id);


--
-- Name: subjects subjects_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subjects
    ADD CONSTRAINT subjects_pkey PRIMARY KEY (id);


--
-- Name: teacher_auths teacher_auths_school_id_teacher_menu_user_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.teacher_auths
    ADD CONSTRAINT teacher_auths_school_id_teacher_menu_user_id_key UNIQUE (school_id, teacher_menu, user_id);


--
-- Name: teacher_menus teacher_menus_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.teacher_menus
    ADD CONSTRAINT teacher_menus_pkey PRIMARY KEY (id);


--
-- Name: teachers_menu teachers_menu_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.teachers_menu
    ADD CONSTRAINT teachers_menu_pkey PRIMARY KEY (id);


--
-- Name: token_books token_books_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.token_books
    ADD CONSTRAINT token_books_pkey PRIMARY KEY (id);


--
-- Name: town town_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.town
    ADD CONSTRAINT town_pkey PRIMARY KEY (pk);


--
-- Name: teacher_available user_id_school_id_group_id_day_id; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.teacher_available
    ADD CONSTRAINT user_id_school_id_group_id_day_id UNIQUE (user_id, school_id, group_id, day);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_key_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_key_key UNIQUE (key);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users users_tel_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_tel_key UNIQUE (tel);


--
-- Name: users users_username_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_username_key UNIQUE (username);


--
-- Name: activities activities_partner_activity_fk; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.activities
    ADD CONSTRAINT activities_partner_activity_fk FOREIGN KEY (partner_activity) REFERENCES public.activities(id);


--
-- Name: activities activities_subject_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.activities
    ADD CONSTRAINT activities_subject_fkey FOREIGN KEY (subject) REFERENCES public.subjects(id) ON DELETE CASCADE;


--
-- Name: books books_library_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.books
    ADD CONSTRAINT books_library_fkey FOREIGN KEY (library) REFERENCES public.libraries(id) ON DELETE CASCADE;


--
-- Name: class_auths class_auths_class_menu_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_auths
    ADD CONSTRAINT class_auths_class_menu_fkey FOREIGN KEY (class_menu) REFERENCES public.class_menus(id);


--
-- Name: class_auths class_auths_group_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_auths
    ADD CONSTRAINT class_auths_group_id_fkey FOREIGN KEY (group_id) REFERENCES public.class_groups(id) ON DELETE CASCADE;


--
-- Name: class_auths class_auths_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_auths
    ADD CONSTRAINT class_auths_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: class_available class_available_class_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_available
    ADD CONSTRAINT class_available_class_id_fkey FOREIGN KEY (class_id) REFERENCES public.classes(id) ON DELETE CASCADE;


--
-- Name: class_available class_available_day_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_available
    ADD CONSTRAINT class_available_day_fkey FOREIGN KEY (day) REFERENCES public.days(id);


--
-- Name: class_groups class_groups_school_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_groups
    ADD CONSTRAINT class_groups_school_fkey FOREIGN KEY (school) REFERENCES public.school(id) ON DELETE CASCADE;


--
-- Name: class_student class_student_class_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_student
    ADD CONSTRAINT class_student_class_id_fkey FOREIGN KEY (class_id) REFERENCES public.classes(id) ON DELETE CASCADE;


--
-- Name: class_student class_student_group_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_student
    ADD CONSTRAINT class_student_group_id_fkey FOREIGN KEY (group_id) REFERENCES public.class_groups(id) ON DELETE CASCADE;


--
-- Name: class_student class_student_student_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_student
    ADD CONSTRAINT class_student_student_fkey FOREIGN KEY (student) REFERENCES public.students(id) ON DELETE CASCADE;


--
-- Name: class_timetable class_timetable_activities_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_timetable
    ADD CONSTRAINT class_timetable_activities_fkey FOREIGN KEY (activity) REFERENCES public.activities(id) ON DELETE CASCADE;


--
-- Name: class_timetable class_timetable_day_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.class_timetable
    ADD CONSTRAINT class_timetable_day_id_fkey FOREIGN KEY (day_id) REFERENCES public.days(id);


--
-- Name: classes classes_school_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.classes
    ADD CONSTRAINT classes_school_fkey FOREIGN KEY (school) REFERENCES public.school(id) ON DELETE CASCADE;


--
-- Name: classes fk_group_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.classes
    ADD CONSTRAINT fk_group_id FOREIGN KEY (group_id) REFERENCES public.class_groups(id) ON DELETE CASCADE;


--
-- Name: timetables fk_group_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.timetables
    ADD CONSTRAINT fk_group_id FOREIGN KEY (group_id) REFERENCES public.class_groups(id) ON DELETE CASCADE;


--
-- Name: group_auths group_auths_group_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.group_auths
    ADD CONSTRAINT group_auths_group_id_fkey FOREIGN KEY (group_id) REFERENCES public.class_groups(id) ON DELETE CASCADE;


--
-- Name: group_auths group_auths_group_menu_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.group_auths
    ADD CONSTRAINT group_auths_group_menu_fkey FOREIGN KEY (group_menu) REFERENCES public.group_menus(id);


--
-- Name: group_auths group_auths_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.group_auths
    ADD CONSTRAINT group_auths_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: teacher_available group_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.teacher_available
    ADD CONSTRAINT group_id_fkey FOREIGN KEY (group_id) REFERENCES public.class_groups(id) ON DELETE CASCADE;


--
-- Name: group_schedules group_schedules_group_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.group_schedules
    ADD CONSTRAINT group_schedules_group_id_fkey FOREIGN KEY (group_id) REFERENCES public.class_groups(id) ON DELETE CASCADE;


--
-- Name: libraries libraries_manager_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.libraries
    ADD CONSTRAINT libraries_manager_fkey FOREIGN KEY (manager) REFERENCES public.users(id);


--
-- Name: libraries libraries_school_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.libraries
    ADD CONSTRAINT libraries_school_fkey FOREIGN KEY (school) REFERENCES public.school(id) ON DELETE CASCADE;


--
-- Name: libraries libraries_student_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.libraries
    ADD CONSTRAINT libraries_student_fkey FOREIGN KEY (student) REFERENCES public.students(id);


--
-- Name: post post_school_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.post
    ADD CONSTRAINT post_school_fkey FOREIGN KEY (school) REFERENCES public.school(id) ON DELETE CASCADE;


--
-- Name: post post_sender_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.post
    ADD CONSTRAINT post_sender_fkey FOREIGN KEY (sender) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: school_auths school_auths_school_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school_auths
    ADD CONSTRAINT school_auths_school_id_fkey FOREIGN KEY (school_id) REFERENCES public.school(id);


--
-- Name: school_auths school_auths_school_menu_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school_auths
    ADD CONSTRAINT school_auths_school_menu_fkey FOREIGN KEY (school_menu) REFERENCES public.school_menus(id);


--
-- Name: school_auths school_auths_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school_auths
    ADD CONSTRAINT school_auths_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id);


--
-- Name: school school_manager_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school
    ADD CONSTRAINT school_manager_fkey FOREIGN KEY (manager) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: school_menus school_menus_school_type_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school_menus
    ADD CONSTRAINT school_menus_school_type_fkey FOREIGN KEY (school_type) REFERENCES public.school_type(id) ON DELETE CASCADE;


--
-- Name: school school_school_type_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school
    ADD CONSTRAINT school_school_type_fkey FOREIGN KEY (school_type) REFERENCES public.school_type(id);


--
-- Name: school school_town_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school
    ADD CONSTRAINT school_town_fkey FOREIGN KEY (town) REFERENCES public.town(pk) ON DELETE CASCADE;


--
-- Name: school_grades school_type_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school_grades
    ADD CONSTRAINT school_type_id_fkey FOREIGN KEY (school_type_id) REFERENCES public.school_type(id) ON DELETE CASCADE;


--
-- Name: school_users school_users_role_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school_users
    ADD CONSTRAINT school_users_role_fkey FOREIGN KEY (role) REFERENCES public.roles(id);


--
-- Name: school_users school_users_school_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school_users
    ADD CONSTRAINT school_users_school_id_fkey FOREIGN KEY (school_id) REFERENCES public.school(id) ON DELETE CASCADE;


--
-- Name: school_users school_users_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.school_users
    ADD CONSTRAINT school_users_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: students students_father_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.students
    ADD CONSTRAINT students_father_fkey FOREIGN KEY (father) REFERENCES public.users(id);


--
-- Name: students students_mother_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.students
    ADD CONSTRAINT students_mother_fkey FOREIGN KEY (mother) REFERENCES public.users(id);


--
-- Name: students students_school_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.students
    ADD CONSTRAINT students_school_fkey FOREIGN KEY (school) REFERENCES public.school(id);


--
-- Name: students students_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.students
    ADD CONSTRAINT students_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: subjects subject_subject_group_fk_group_id; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subjects
    ADD CONSTRAINT subject_subject_group_fk_group_id FOREIGN KEY (group_id) REFERENCES public.subject_group(id);


--
-- Name: subjects subjects_school_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subjects
    ADD CONSTRAINT subjects_school_fkey FOREIGN KEY (school) REFERENCES public.school(id) ON DELETE CASCADE;


--
-- Name: subjects subjects_school_type_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.subjects
    ADD CONSTRAINT subjects_school_type_fkey FOREIGN KEY (school_type) REFERENCES public.school_type(id);


--
-- Name: teacher_auths teacher_auths_school_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.teacher_auths
    ADD CONSTRAINT teacher_auths_school_id_fkey FOREIGN KEY (school_id) REFERENCES public.school(id) ON DELETE CASCADE;


--
-- Name: teacher_auths teacher_auths_teacher_menu_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.teacher_auths
    ADD CONSTRAINT teacher_auths_teacher_menu_fkey FOREIGN KEY (teacher_menu) REFERENCES public.teacher_menus(id);


--
-- Name: teacher_auths teacher_auths_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.teacher_auths
    ADD CONSTRAINT teacher_auths_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: teacher_available teacher_available_day_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.teacher_available
    ADD CONSTRAINT teacher_available_day_fkey FOREIGN KEY (day) REFERENCES public.days(id);


--
-- Name: token_books token_books_book_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.token_books
    ADD CONSTRAINT token_books_book_fkey FOREIGN KEY (book) REFERENCES public.books(id);


--
-- Name: token_books token_books_library_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.token_books
    ADD CONSTRAINT token_books_library_fkey FOREIGN KEY (library) REFERENCES public.libraries(id);


--
-- Name: token_books token_books_student_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.token_books
    ADD CONSTRAINT token_books_student_fkey FOREIGN KEY (student) REFERENCES public.users(id);


--
-- Name: town town_city_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.town
    ADD CONSTRAINT town_city_fkey FOREIGN KEY (city) REFERENCES public.city(pk) ON DELETE CASCADE;


--
-- Name: teacher_available user_id_school_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.teacher_available
    ADD CONSTRAINT user_id_school_id_fkey FOREIGN KEY (user_id, school_id) REFERENCES public.school_users(user_id, school_id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

