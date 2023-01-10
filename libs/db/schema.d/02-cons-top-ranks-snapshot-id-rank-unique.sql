ALTER TABLE top_ranks
ADD CONSTRAINT cons_top_ranks_snapshot_id_rank_unique
UNIQUE (snapshot_id, rank)