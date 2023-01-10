ALTER TABLE top_ranks
ADD CONSTRAINT cons_fk_ranks_item_id
FOREIGN KEY (item_id)
REFERENCES items (id)