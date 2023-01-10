CREATE TABLE snapshots (
    id BIGINT NOT NULL DEFAULT nextval('ids'),
    taken_at TIMESTAMP WITH TIME ZONE NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),

    PRIMARY KEY (id)
)