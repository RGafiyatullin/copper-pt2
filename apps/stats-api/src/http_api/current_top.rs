use crate::{AnyError, PgPool};
use axum::extract::State;
use axum::response::IntoResponse;

pub async fn current_top(State(db_pool): State<PgPool>) -> impl IntoResponse {
    match do_current_top(db_pool).await {
        Ok(response) => response.into_response(),
        Err(reason) => {
            log::warn!("current-top: {:?}", reason);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Sorry, ISE").into_response()
        },
    }
}
async fn do_current_top(db_pool: PgPool) -> Result<impl axum::response::IntoResponse, AnyError> {
    let db_conn = db_pool.get().await?;

    let rows = db_conn
        .query(
            r#"
            SELECT
                T.rank,
                I.item_id,
                U.nickname
            FROM
                top_ranks T
                JOIN items I ON I.id = T.item_id
                JOIN users U ON U.id = I.posted_by
            WHERE
                T.snapshot_id = (
                    SELECT
                        S.id
                    FROM
                        snapshots S
                    ORDER BY
                        S.taken_at DESC
                    LIMIT 1)
            ORDER BY
                T.rank ASC
            LIMIT 1000
            "#,
            &[],
        )
        .await?;

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Item {
        rank: i32,
        item_id: i64,
        by: String,
    }
    let mut items: Vec<Item> = vec![];
    for row in rows {
        let item = Item {
            rank: row.try_get("rank")?,
            item_id: row.try_get("item_id")?,
            by: row.try_get("nickname")?,
        };
        items.push(item);
    }

    Ok((axum::http::StatusCode::OK, axum::response::Json(items)))
}
