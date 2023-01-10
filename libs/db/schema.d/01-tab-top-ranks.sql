CREATE TABLE top_ranks (
    id BIGINT NOT NULL DEFAULT nextval('ids'),

    snapshot_id BIGINT NOT NULL,
    rank INT NOT NULL,

    item_id BIGINT NOT NULL,
    
    -- created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),

    PRIMARY KEY (id)
)
