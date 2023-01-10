ALTER TABLE top_ranks
ADD CONSTRAINT cons_fk_ranks_snapshot_id
FOREIGN KEY (snapshot_id)
REFERENCES snapshots (id)