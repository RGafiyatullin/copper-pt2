CREATE TABLE users (
    id BIGINT NOT NULL DEFAULT nextval('ids'),
    nickname VARCHAR(128) NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),

    PRIMARY KEY (id)
)