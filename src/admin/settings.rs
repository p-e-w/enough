// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use askama::Template;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension, Form,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::Deserialize;

use crate::{
    admin::markdown::markdown_to_html, settings, ErrorResponse, HtmlTemplate, ADMIN_URL_PREFIX,
};

#[derive(Template)]
#[template(path = "admin/header.html")]
struct HeaderTemplate<'a> {
    admin_url_prefix: &'a str,
    title: &'a str,
    header: String,
}

pub(super) async fn get_header(
    Extension(ref database_connection): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ErrorResponse> {
    Ok(HtmlTemplate(HeaderTemplate {
        admin_url_prefix: ADMIN_URL_PREFIX,
        title: "Header",
        header: settings(database_connection).await?.header_markdown,
    }))
}

#[derive(Debug, Deserialize)]
pub(super) struct HeaderInput {
    header: String,
}

pub(super) async fn post_header(
    Extension(ref database_connection): Extension<DatabaseConnection>,
    Form(ref header_input): Form<HeaderInput>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let mut settings: settings::ActiveModel = settings(database_connection).await?.into();

    settings.header_markdown = Set(header_input.header.clone());

    settings.header_html = Set(markdown_to_html(&header_input.header));

    settings
        .update(database_connection)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "unable to save header"))?;

    Ok(Redirect::to(&format!("{}/header", ADMIN_URL_PREFIX)))
}

#[derive(Template)]
#[template(path = "admin/footer.html")]
struct FooterTemplate<'a> {
    admin_url_prefix: &'a str,
    title: &'a str,
    footer: String,
}

pub(super) async fn get_footer(
    Extension(ref database_connection): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ErrorResponse> {
    Ok(HtmlTemplate(FooterTemplate {
        admin_url_prefix: ADMIN_URL_PREFIX,
        title: "Footer",
        footer: settings(database_connection).await?.footer_markdown,
    }))
}

#[derive(Debug, Deserialize)]
pub(super) struct FooterInput {
    footer: String,
}

pub(super) async fn post_footer(
    Extension(ref database_connection): Extension<DatabaseConnection>,
    Form(ref footer_input): Form<FooterInput>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let mut settings: settings::ActiveModel = settings(database_connection).await?.into();

    settings.footer_markdown = Set(footer_input.footer.clone());

    settings.footer_html = Set(markdown_to_html(&footer_input.footer));

    settings
        .update(database_connection)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "unable to save footer"))?;

    Ok(Redirect::to(&format!("{}/footer", ADMIN_URL_PREFIX)))
}

#[derive(Template)]
#[template(path = "admin/css.html")]
struct CssTemplate<'a> {
    admin_url_prefix: &'a str,
    title: &'a str,
    css: String,
}

pub(super) async fn get_css(
    Extension(ref database_connection): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ErrorResponse> {
    Ok(HtmlTemplate(CssTemplate {
        admin_url_prefix: ADMIN_URL_PREFIX,
        title: "CSS",
        css: settings(database_connection).await?.css,
    }))
}

#[derive(Debug, Deserialize)]
pub(super) struct CssInput {
    css: String,
}

pub(super) async fn post_css(
    Extension(ref database_connection): Extension<DatabaseConnection>,
    Form(ref css_input): Form<CssInput>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let mut settings: settings::ActiveModel = settings(database_connection).await?.into();

    settings.css = Set(css_input.css.clone());

    settings
        .update(database_connection)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "unable to save CSS"))?;

    Ok(Redirect::to(&format!("{}/css", ADMIN_URL_PREFIX)))
}

#[derive(Template)]
#[template(path = "admin/javascript.html")]
struct JavascriptTemplate<'a> {
    admin_url_prefix: &'a str,
    title: &'a str,
    javascript: String,
}

pub(super) async fn get_javascript(
    Extension(ref database_connection): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ErrorResponse> {
    Ok(HtmlTemplate(JavascriptTemplate {
        admin_url_prefix: ADMIN_URL_PREFIX,
        title: "JavaScript",
        javascript: settings(database_connection).await?.javascript,
    }))
}

#[derive(Debug, Deserialize)]
pub(super) struct JavascriptInput {
    javascript: String,
}

pub(super) async fn post_javascript(
    Extension(ref database_connection): Extension<DatabaseConnection>,
    Form(ref javascript_input): Form<JavascriptInput>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let mut settings: settings::ActiveModel = settings(database_connection).await?.into();

    settings.javascript = Set(javascript_input.javascript.clone());

    settings.update(database_connection).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "unable to save JavaScript",
        )
    })?;

    Ok(Redirect::to(&format!("{}/javascript", ADMIN_URL_PREFIX)))
}

#[derive(Template)]
#[template(path = "admin/settings.html")]
struct SettingsTemplate<'a> {
    admin_url_prefix: &'a str,
    title: &'a str,
    settings: settings::Model,
}

pub(super) async fn get_settings(
    Extension(ref database_connection): Extension<DatabaseConnection>,
) -> Result<impl IntoResponse, ErrorResponse> {
    Ok(HtmlTemplate(SettingsTemplate {
        admin_url_prefix: ADMIN_URL_PREFIX,
        title: "Settings",
        settings: settings(database_connection).await?,
    }))
}

#[derive(Debug, Deserialize)]
pub(super) struct SettingsInput {
    posts_per_page: String,
}

pub(super) async fn post_settings(
    Extension(ref database_connection): Extension<DatabaseConnection>,
    Form(ref settings_input): Form<SettingsInput>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let mut settings: settings::ActiveModel = settings(database_connection).await?.into();

    settings.posts_per_page = Set(match settings_input.posts_per_page.parse() {
        Ok(posts_per_page) if posts_per_page > 0 => posts_per_page,
        _ => {
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                "invalid 'posts per page' value, must be a positive integer",
            ));
        }
    });

    settings
        .update(database_connection)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "unable to save settings"))?;

    Ok(Redirect::to(&format!("{}/settings", ADMIN_URL_PREFIX)))
}
