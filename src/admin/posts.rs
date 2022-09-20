// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use askama::Template;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Form};
use chrono::{NaiveDate, Utc};
use entity::{page, prelude::Page};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
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
}

pub(super) async fn get_post(
    Extension(ref database_connection): Extension<DatabaseConnection>,
    Path(post_id): Path<i32>,
) -> Result<impl IntoResponse, ErrorResponse> {
    Ok(HtmlTemplate(PostTemplate {
        admin_url_prefix: ADMIN_URL_PREFIX,
        title: "Edit post",
        post: post_by_id(database_connection, post_id).await?,
    }))
}

#[derive(Debug, Deserialize)]
pub(super) struct PostInput {
    title: String,
    url: String,
    date: String,
    content: String,
}

pub(super) async fn post_post(
    Extension(ref database_connection): Extension<DatabaseConnection>,
    Path(post_id): Path<i32>,
    Form(ref post_input): Form<PostInput>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let mut post: page::ActiveModel = post_by_id(database_connection, post_id).await?.into();

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
            .and_hms(12, 0, 0)
    });

    // TODO: Generate HTML!
    post.content_markdown = Set(post_input.content.clone());

    Ok(HtmlTemplate(PostTemplate {
        admin_url_prefix: ADMIN_URL_PREFIX,
        title: "Edit post",
        post: post
            .update(database_connection)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "unable to save post"))?,
    }))
}
