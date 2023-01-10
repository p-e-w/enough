// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

mod markdown;
mod posts;
mod settings;

use axum::{
    routing::{get, post},
    Router,
};
use regex::Regex;

pub(super) fn router() -> Router {
    Router::new()
        .route("/posts", get(posts::get_posts))
        .route(
            "/posts/:post_id",
            get(posts::get_post).post(posts::post_save_post),
        )
        .route("/posts/:post_id/publish", post(posts::post_publish_post))
        .route(
            "/posts/:post_id/unpublish",
            post(posts::post_unpublish_post),
        )
        .route(
            "/posts/:post_id/delete",
            get(posts::get_delete_post).post(posts::post_delete_post),
        )
        .route(
            "/header",
            get(settings::get_header).post(settings::post_header),
        )
        .route(
            "/footer",
            get(settings::get_footer).post(settings::post_footer),
        )
        .route("/css", get(settings::get_css).post(settings::post_css))
        .route(
            "/javascript",
            get(settings::get_javascript).post(settings::post_javascript),
        )
        .route(
            "/settings",
            get(settings::get_settings).post(settings::post_settings),
        )
}

fn is_valid_url(url: &str) -> bool {
    Regex::new(r"^[a-zA-Z0-9-]+$").unwrap().is_match(url)
}

fn title_to_url(title: &str) -> String {
    let whitespace = Regex::new(r"\s+").unwrap();
    let disallowed_characters = Regex::new(r"[^a-zA-Z0-9-]+").unwrap();
    let hyphens = Regex::new(r"-+").unwrap();

    let title = whitespace.replace_all(title, "-");
    let title = disallowed_characters.replace_all(&title, "");
    let title = hyphens.replace_all(&title, "-");

    title.to_lowercase()
}
