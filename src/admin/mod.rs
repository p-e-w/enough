// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

mod posts;

use axum::{routing::get, Router};

pub(super) fn router() -> Router {
    Router::new().route("/posts", get(posts::get))
}
