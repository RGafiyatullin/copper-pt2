use crate::{AnyError, PgPool};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use db::DateTime;

pub async fn users_reaching_top(
    State(db_pool): State<PgPool>,
    Path(rank_below): Path<usize>,
) -> impl IntoResponse {
    match do_users_reaching_top(db_pool, rank_below).await {
        Ok(response) => response.into_response(),
        Err(reason) => {
            log::warn!("posts-by-user: {:?}", reason);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Sorry, ISE").into_response()
        },
    }
}

async fn do_users_reaching_top(
    db_pool: PgPool,
    rank_below: usize,
) -> Result<impl axum::response::IntoResponse, AnyError> {
    let db_conn = db_pool.get().await?;

    let rows = db_conn
        .query(
            r#"
            SELECT
                *
            FROM (
                SELECT
                    FIRST_VALUE(T.rank) OVER W best_rank,
                        I.item_id,
                        U.nickname,
                        FIRST_VALUE(S.taken_at) OVER W best_at,
                            I.posted_at posted_at
                        FROM
                            top_ranks T
                            JOIN snapshots S ON T.snapshot_id = S.id
                            JOIN items I ON T.item_id = I.id
                            JOIN users U ON I.posted_by = U.id
                        WHERE
                            T.rank < $1 WINDOW W AS (PARTITION BY I.item_id, U.nickname ORDER BY T.rank ASC, S.taken_at ASC)
                        ORDER BY
                            U.nickname, I.item_id, T.rank) R
            GROUP BY
                R.best_rank,
                R.item_id,
                R.nickname,
                R.best_at,
                R.posted_at
            ORDER BY
                R.best_rank
            "#,
            &[&(rank_below as i32)],
        )
        .await?;

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Item {
        item_id: i64,
        best_rank: i32,
        user_name: String,

        #[serde(with = "crate::utils::serde_date_time_fixed_offset")]
        best_at: DateTime,
        #[serde(with = "crate::utils::serde_date_time_fixed_offset")]
        posted_at: DateTime,
    }
    let mut items: Vec<Item> = vec![];
    for row in rows {
        let item = Item {
            item_id: row.try_get("item_id")?,
            user_name: row.try_get("nickname")?,
            posted_at: row.try_get("posted_at")?,
            best_rank: row.try_get("best_rank")?,
            best_at: row.try_get("best_at")?,
        };
        items.push(item);
    }

    Ok((axum::http::StatusCode::OK, axum::response::Json(items)))
}
