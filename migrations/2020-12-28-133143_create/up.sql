-- Your SQL goes here

CREATE TABLE "tb_user" (
	"id"	serial8		NOT NULL,
	"email"	varchar(200)		NOT NULL,
	"salt" varchar(100) NOT NULL, 
	"password"	text		NOT NULL,
	"user_type"	varchar(1)	DEFAULT 'U'	NOT NULL,
	"nickname"	varchar(30)		NOT NULL,
	"use_yn"	boolean	DEFAULT true	NOT NULL,
	"reg_utc"	int8	DEFAULT floor(date_part('epoch'::text, now()))::bigint	NOT NULL
);

ALTER TABLE "tb_user" ADD CONSTRAINT "PK_USER" PRIMARY KEY (
	"id"
);