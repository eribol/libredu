use super::auth::POSTGRES;
use moon::tokio_stream::StreamExt;
use shared::DownMsg;
use sqlx::Row;

pub async fn get_school(manager: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut query = sqlx::query(r#"select id, name from school where manager = $1"#)
        .bind(&manager)
        .fetch(&*db);
    if let Ok(row) = query.try_next().await {
        if let Some(row2) = row {
            return DownMsg::GetSchool {
                id: row2.try_get("id").unwrap(),
                name: row2.try_get("name").unwrap(),
            };
        }
    }
    DownMsg::AuthError("Not auth for school".to_string())
}
