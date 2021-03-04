-- Your SQL goes here
-- 테이블 재정의시 diesel migration redo를 수행할것

CREATE TABLE "tb_user" (
	"id"	serial8		NOT NULL,
	"email"	varchar(200)		NOT NULL,
	"salt"	varchar(100)		NOT NULL,
	"password"	text		NOT NULL,
	"user_type"	varchar(10)	DEFAULT 'USER'	NOT NULL,
	"nickname"	varchar(30)		NOT NULL,
	"use_yn"	boolean	DEFAULT true	NOT NULL,
	"reg_utc"	int8	DEFAULT floor(date_part('epoch'::text, now()))::bigint	NOT NULL
);

COMMENT ON COLUMN "tb_user"."email" IS '이메일.  unique';

COMMENT ON COLUMN "tb_user"."password" IS '패스워드';

COMMENT ON COLUMN "tb_user"."user_type" IS 'USER=일반사용자. ADMIN=어드민';

COMMENT ON COLUMN "tb_user"."use_yn" IS '사용여부';

COMMENT ON COLUMN "tb_user"."reg_utc" IS '등록시간';

CREATE TABLE "tb_document_history" (
	"id"	serial8		NOT NULL,
	"writer_id"	int8 	NOT NULL,
	"document_id"	int8		NOT NULL,
	"filepath"	text		NOT NULL,
	"increase"	int8		NOT NULL,
	"reg_date"	int8	DEFAULT floor(date_part('epoch'::text, now()))::bigint	NOT NULL
);

CREATE TABLE "tb_document" (
	"id"	serial8		NOT NULL,
	"title"	text		NOT NULL,
	"reg_utc"	int8	DEFAULT floor(date_part('epoch'::text, now()))::bigint	NOT NULL
);

COMMENT ON COLUMN "tb_document"."title" IS '문서 제목';

CREATE TABLE "tb_image" (
	"id"	serial		NOT NULL,
	"uploader_id"	int8		NOT NULL,
	"domain"	text		NULL,
	"path"	text		NOT NULL,
	"use_yn"	bool	DEFAULT true	NOT NULL,
	"reg_utc"	int8	DEFAULT floor(date_part('epoch'::text, now()))::bigint	NOT NULL,
	"Field"	VARCHAR(255)		NULL
);

CREATE TABLE "tb_refresh_token" (
	"token_value"	text		NOT NULL,
	"user_id"	int8		NOT NULL,
	"reg_utc"	int8	DEFAULT floor(date_part('epoch'::text, now()))::bigint	NOT NULL,
	"dead_yn"	bool	DEFAULT false	NOT NULL,
	"dead_utc"	int8		NULL
);

COMMENT ON COLUMN "tb_refresh_token"."user_id" IS '사용자 식별자';

COMMENT ON COLUMN "tb_refresh_token"."dead_yn" IS '삭제여부';

COMMENT ON COLUMN "tb_refresh_token"."dead_utc" IS '삭제일자';

ALTER TABLE "tb_user" ADD CONSTRAINT "PK_TB_USER" PRIMARY KEY (
	"id"
);

ALTER TABLE "tb_document_history" ADD CONSTRAINT "PK_TB_DOCUMENT_HISTORY" PRIMARY KEY (
	"id",
	"writer_id",
	"document_id"
);

ALTER TABLE "tb_document" ADD CONSTRAINT "PK_TB_DOCUMENT" PRIMARY KEY (
	"id"
);

ALTER TABLE "tb_image" ADD CONSTRAINT "PK_TB_IMAGE" PRIMARY KEY (
	"id",
	"uploader_id"
);

ALTER TABLE "tb_refresh_token" ADD CONSTRAINT "PK_TB_REFRESH_TOKEN" PRIMARY KEY (
	"token_value",
	"user_id"
);


