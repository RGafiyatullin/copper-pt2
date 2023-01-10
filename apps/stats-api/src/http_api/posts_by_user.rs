use crate::{AnyError, PgPool};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use db::DateTime;

pub async fn posts_by_user(
    State(db_pool): State<PgPool>,
    Path(user_name): Path<String>,
) -> impl IntoResponse {
    match do_posts_by_user(db_pool, &user_name).await {
        Ok(response) => response.into_response(),
        Err(reason) => {
            log::warn!("posts-by-user: {:?}", reason);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Sorry, ISE").into_response()
        },
    }
}

async fn do_posts_by_user(
    db_pool: PgPool,
    user_name: &str,
) -> Result<impl axum::response::IntoResponse, AnyError> {
    let db_conn = db_pool.get().await?;

    let rows = db_conn
        .query(
            r#"
            SELECT R.* FROM (
                SELECT
                    I.item_id,
                    U.nickname,
                    FIRST_VALUE(T.rank) OVER W best_rank,
                    FIRST_VALUE(S.taken_at) OVER W best_at,
                    I.posted_at
                FROM
                    users U
                    JOIN items I ON U.id = I.posted_by
                    JOIN top_ranks T ON I.id = T.item_id
                    JOIN snapshots S ON T.snapshot_id = S.id
                WHERE
                    U.nickname = $1
                WINDOW
                    W AS (PARTITION BY (I.item_id, U.nickname) ORDER BY T.rank ASC, S.taken_at ASC)
                ) R
                GROUP BY
                    R.item_id,
                    R.nickname,
                    R.posted_at,
                    R.best_rank,
                    R.best_at
                
                ORDER BY
                    R.nickname,
                    R.best_rank ASC,
                    R.item_id
                    
            "#,
            &[&user_name],
        )
        .await?;

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Item {
        item_id: i64,
        best_rank: i32,
        #[serde(with = "crate::utils::serde_date_time_fixed_offset")]
        best_at: DateTime,
        #[serde(with = "crate::utils::serde_date_time_fixed_offset")]
        posted_at: DateTime,
    }
    let mut items: Vec<Item> = vec![];
    for row in rows {
        let item = Item {
            item_id: row.try_get("item_id")?,
            posted_at: row.try_get("posted_at")?,
            best_rank: row.try_get("best_rank")?,
            best_at: row.try_get("best_at")?,
        };
        items.push(item);
    }

    Ok((axum::http::StatusCode::OK, axum::response::Json(items)))
}
