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
                    .table(Page::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Page::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Page::Time).timestamp().not_null())
                    .col(ColumnDef::new(Page::Title).string().not_null())
                    .col(ColumnDef::new(Page::Url).string().not_null())
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
                    .if_not_exists()
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
