use axum::{extract::State, Json};
    use sqlx::SqlitePool;
        use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Student {
    pub id: Option<i32>,
        pub name: String,
}

pub async fn add_student(
    State(pool): State<SqlitePool>,
        Json(payload): Json<Student>,
) -> Json<Student> {
    let res = sqlx::query_as::<_, Student>(
        "INSERT INTO students (name) VALUES (?) RETURNING *"
    )
    .bind(payload.name)
        .fetch_one(&pool)
            .await
             .unwrap();
                  Json(res)
}