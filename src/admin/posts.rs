// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension, Form,
};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use entity::{page, prelude::Page};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::Deserialize;

use crate::{
    admin::{is_valid_url, title_to_url},
    ErrorResponse, HtmlTemplate, ADMIN_URL_PREFIX,
};

async fn post_by_id(
    connection: &DatabaseConnection,
    id: i32,
) -> Result<page::Model, ErrorResponse> {
    Page::find_by_id(id)
        .filter(page::Column::IsPost.eq(true))
        .one(connection)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "unable to retrieve post"))?
        .ok_or((StatusCode::NOT_FOUND, "post not found"))
}

#[derive(Template)]
#[template(path = "admin/posts.html")]
struct PostsTemplate<'a> {
    admin_url_prefix: &'a str,
    title: &'a str,
    posts: Vec<page::Model>,
}

pub(super) async fn get_posts(
    Extension(ref database_connection): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ErrorResponse> {
    Ok(HtmlTemplate(PostsTemplate {
        admin_url_prefix: ADMIN_URL_PREFIX,
        title: "Posts",
        posts: Page::find()
            .filter(page::Column::IsPost.eq(true))
            .order_by_desc(page::Column::Time)
            .all(database_connection)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "unable to retrieve posts",
                )
            })?,
    }))
}

#[derive(Template)]
#[template(path = "admin/post.html")]
struct PostTemplate<'a> {
    admin_url_prefix: &'a str,
    title: &'a str,
    post: page::Model,
    is_new: bool,
}

pub(super) async fn get_post(
    Extension(ref database_connection): Extension<DatabaseConnection>,
    Path(post_id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let is_new = post_id == "new";

    let post = if is_new {
        page::Model {
            id: 0,
            time: NaiveDateTime::MIN,
            title: String::new(),
            url: String::new(),
            content_markdown: String::new(),
            content_html: String::new(),
            is_post: true,
            is_published: false,
        }
    } else {
        post_by_id(
            database_connection,
            post_id
                .parse()
                .map_err(|_| (StatusCode::BAD_REQUEST, "invalid post ID"))?,
        )
        .await?
    };

    Ok(HtmlTemplate(PostTemplate {
        admin_url_prefix: ADMIN_URL_PREFIX,
        title: if is_new { "New post" } else { "Edit post" },
        post,
        is_new,
    }))
}

#[derive(Debug, Deserialize)]
pub(super) struct PostInput {
    title: String,
    url: String,
    date: String,
    content: String,
}

async fn save_post(
    database_connection: &DatabaseConnection,
    post_id: String,
    post_input: &PostInput,
    set_is_published: Option<bool>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let is_new = post_id == "new";

    let mut post = if is_new {
        page::ActiveModel {
            is_post: Set(true),
            is_published: Set(false),
            ..Default::default()
        }
    } else {
        post_by_id(
            database_connection,
            post_id
                .parse()
                .map_err(|_| (StatusCode::BAD_REQUEST, "invalid post ID"))?,
        )
        .await?
        .into()
    };

    post.title = Set(post_input.title.clone());

    post.url = Set(if post_input.url.is_empty() {
        title_to_url(&post_input.title)
    } else if is_valid_url(&post_input.url) {
        post_input.url.clone()
    } else {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            "invalid URL, must contain only letters (a-z, A-Z), digits (0-9), and hyphens (-)",
        ));
    });

    post.time = Set(if post_input.date.is_empty() {
        Utc::now().naive_utc()
    } else {
        NaiveDate::parse_from_str(&post_input.date, "%Y-%m-%d")
            .map_err(|_| {
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "invalid date, must be in format YYYY-MM-DD",
                )
            })?
            .and_hms_opt(12, 0, 0)
            .unwrap()
    });

    post.content_markdown = Set(post_input.content.clone());

    // TODO: Generate HTML!
    post.content_html = Set(String::new());

    if let Some(is_published) = set_is_published {
        post.is_published = Set(is_published);
    }

    let post = if is_new {
        post.insert(database_connection)
    } else {
        post.update(database_connection)
    }
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "unable to save post"))?;

    Ok(Redirect::to(&format!(
        "{}/posts/{}",
        ADMIN_URL_PREFIX, post.id,
    )))
}

pub(super) async fn post_save_post(
    Extension(ref database_connection): Extension<DatabaseConnection>,
    Path(post_id): Path<String>,
    Form(ref post_input): Form<PostInput>,
) -> Result<impl IntoResponse, ErrorResponse> {
    save_post(database_connection, post_id, post_input, None).await
}

pub(super) async fn post_publish_post(
    Extension(ref database_connection): Extension<DatabaseConnection>,
    Path(post_id): Path<String>,
    Form(ref post_input): Form<PostInput>,
) -> Result<impl IntoResponse, ErrorResponse> {
    save_post(database_connection, post_id, post_input, Some(true)).await
}

pub(super) async fn post_unpublish_post(
    Extension(ref database_connection): Extension<DatabaseConnection>,
    Path(post_id): Path<String>,
    Form(ref post_input): Form<PostInput>,
) -> Result<impl IntoResponse, ErrorResponse> {
    save_post(database_connection, post_id, post_input, Some(false)).await
}

#[derive(Template)]
#[template(path = "admin/delete_post.html")]
struct DeletePostTemplate<'a> {
    admin_url_prefix: &'a str,
    title: &'a str,
    post: page::Model,
}

pub(super) async fn get_delete_post(
    Extension(ref database_connection): Extension<DatabaseConnection>,
    Path(post_id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let post = post_by_id(
        database_connection,
        post_id
            .parse()
            .map_err(|_| (StatusCode::BAD_REQUEST, "invalid post ID"))?,
    )
    .await?;

    Ok(HtmlTemplate(DeletePostTemplate {
        admin_url_prefix: ADMIN_URL_PREFIX,
        title: "Delete post",
        post,
    }))
}

pub(super) async fn post_delete_post(
    Extension(ref database_connection): Extension<DatabaseConnection>,
    Path(post_id): Path<String>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let post = post_by_id(
        database_connection,
        post_id
            .parse()
            .map_err(|_| (StatusCode::BAD_REQUEST, "invalid post ID"))?,
    )
    .await?;

    post.delete(database_connection)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "unable to delete post"))?;

    Ok(Redirect::to(&format!("{}/posts", ADMIN_URL_PREFIX)))
}
