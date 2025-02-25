use std::fmt::Result;

use super::model;
use crate::data::{DataError, DatabasePool};
use crate::ShortCode;
use sqlx::Row;

type DataResult<T> = std::result::Result<T, DataError>;

pub async fn get_clip<M: Into<model::GetClip>>(
    model: M,
    pool: &DatabasePool,
) -> DataResult<model::Clip> {
    //convert the model into a GetClip struct
    let model = model.into();
    let shortcode = model.shortcode.as_str();
    Ok(sqlx::query_as!(
        model::Clip,
        "SELECT * FROM clips WHERE shortcode = ?",
        shortcode,
    )
    .fetch_one(pool)
    .await?)
}

pub async fn new_clip<M: Into<model::NewClip>>(
    model: M,
    pool: &DatabasePool,
) -> DataResult<model::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"INSERT INTO clips(
        clip_id,
        shortcode,
        content,
        title,
        posted,
        expires,
        password,
        hits
        )
        VALUES(?, ?, ?, ?, ?, ?, ?, ?)"#,
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0
    )
    .execute(pool)
    .await?;
    get_clip(model.shortcode, pool).await
}

pub async fn update_clip<M: Into<model::UpdateClip>>(
    model: M,
    pool: &DatabasePool,
) -> DataResult<model::Clip> {
    let model = model.into();
    let _ = sqlx::query!(
        r#"UPDATE clips SET
        content = ?,
        expires = ?,
        password =?,
        title= ?
        WHERE shortcode = ?
        "#,
        model.content,
        model.expires,
        model.password,
        model.title,
        model.shortcode,
    )
    .execute(pool)
    .await?;
    get_clip(model.shortcode, pool).await
}
