ALTER TABLE items
ADD CONSTRAINT cons_fk_items_posted_by
FOREIGN KEY (posted_by)
REFERENCES users (id)