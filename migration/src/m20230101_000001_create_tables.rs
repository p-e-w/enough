// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2022  Philipp Emanuel Weidmann <pew@worldwidemann.com>

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Settings::Table)
                    .col(
                        ColumnDef::new(Settings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Settings::HeaderMarkdown).text().not_null())
                    .col(ColumnDef::new(Settings::HeaderHtml).text().not_null())
                    .col(ColumnDef::new(Settings::FooterMarkdown).text().not_null())
                    .col(ColumnDef::new(Settings::FooterHtml).text().not_null())
                    .col(ColumnDef::new(Settings::Css).text().not_null())
                    .col(ColumnDef::new(Settings::Javascript).text().not_null())
                    .col(ColumnDef::new(Settings::PostsPerPage).integer().not_null())
                    .to_owned(),
            )
            .await?;

        // Default settings.
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Settings::Table)
                    .columns([
                        Settings::HeaderMarkdown,
                        Settings::HeaderHtml,
                        Settings::FooterMarkdown,
                        Settings::FooterHtml,
                        Settings::Css,
                        Settings::Javascript,
                        Settings::PostsPerPage,
                    ])
                    .values_panic([
                        "".into(),
                        "".into(),
                        "".into(),
                        "".into(),
                        "".into(),
                        "".into(),
                        5.into(),
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Page::Table)
                    .col(
                        ColumnDef::new(Page::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Page::Time).timestamp().not_null())
                    .col(ColumnDef::new(Page::Title).text().not_null())
                    .col(ColumnDef::new(Page::Url).text().not_null())
                    .col(ColumnDef::new(Page::ContentMarkdown).text().not_null())
                    .col(ColumnDef::new(Page::ContentHtml).text().not_null())
                    .col(ColumnDef::new(Page::IsPost).boolean().not_null())
                    .col(ColumnDef::new(Page::IsPublished).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-page-time")
                    .table(Page::Table)
                    .col(Page::Time)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        unimplemented!();
    }
}

#[derive(Iden)]
enum Settings {
    Table,
    Id,
    HeaderMarkdown,
    HeaderHtml,
    FooterMarkdown,
    FooterHtml,
    Css,
    Javascript,
    PostsPerPage,
}

#[derive(Iden)]
enum Page {
    Table,
    Id,
    Time,
    Title,
    Url,
    ContentMarkdown,
    ContentHtml,
    IsPost,
    IsPublished,
}
