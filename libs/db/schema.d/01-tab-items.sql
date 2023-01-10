CREATE TABLE items (
    id BIGINT NOT NULL DEFAULT nextval('ids'),
    item_id BIGINT NOT NULL,
    posted_by BIGINT NOT NULL,
    posted_at TIMESTAMP WITH TIME ZONE NOT NULL,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),

    PRIMARY KEY (id)
)