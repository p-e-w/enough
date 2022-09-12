// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use askama::Template;
use axum::response::IntoResponse;

use crate::{HtmlTemplate, ADMIN_URL_PREFIX};

#[derive(Template)]
#[template(path = "admin/posts.html")]
struct PostsTemplate<'a> {
    admin_url_prefix: &'a str,
    title: &'a str,
}

pub(super) async fn get() -> impl IntoResponse {
    let template = PostsTemplate {
        admin_url_prefix: ADMIN_URL_PREFIX,
        title: "Posts",
    };

    HtmlTemplate(template)
}
