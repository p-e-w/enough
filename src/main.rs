// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

mod admin;

use std::env;

use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Extension, Router, Server,
};
use entity::{prelude::Settings, settings};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, EntityTrait};

const ADMIN_URL_PREFIX: &str = "/-";

type ErrorResponse = (StatusCode, &'static str);

// From https://github.com/tokio-rs/axum/blob/1fe45583626a4c9c890cc01131d38c57f8728686/examples/templates/src/main.rs
// TODO: Remove once `askama_axum` supports the latest axum version!
struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

async fn settings(connection: &DatabaseConnection) -> Result<settings::Model, ErrorResponse> {
    Settings::find()
        .one(connection)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "unable to retrieve settings",
            )
        })?
        .ok_or((
            StatusCode::INTERNAL_SERVER_ERROR,
            "settings not found; this means the database is corrupt",
        ))
}

#[tokio::main]
async fn main() {
    let database_url =
        env::var("DATABASE_URL").expect("required environment variable DATABASE_URL not set");

    let database_connection = Database::connect(database_url)
        .await
        .expect("unable to connect to database");

    Migrator::up(&database_connection, None)
        .await
        .expect("unable to apply database migrations");

    let router = Router::new()
        .nest(ADMIN_URL_PREFIX, admin::router())
        .layer(Extension(database_connection));

    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
