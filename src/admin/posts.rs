// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use askama::Template;
use axum::{response::IntoResponse, Extension};
use entity::{page, prelude::Page};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

use crate::{HtmlTemplate, ADMIN_URL_PREFIX};

#[derive(Template)]
#[template(path = "admin/posts.html")]
struct PostsTemplate<'a> {
    admin_url_prefix: &'a str,
    title: &'a str,
    posts: Vec<page::Model>,
}

pub(super) async fn get(
    Extension(ref database_connection): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    HtmlTemplate(PostsTemplate {
        admin_url_prefix: ADMIN_URL_PREFIX,
        title: "Posts",
        posts: Page::find()
            .filter(page::Column::IsPost.eq(true))
            .order_by_desc(page::Column::Time)
            .all(database_connection)
            .await
            .expect("unable to retrieve posts"),
    })
}
