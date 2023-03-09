use sea_orm_migration::prelude::*;

use super::m20230309_144142_create_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Text).string().not_null())
                    .col(ColumnDef::new(Post::UserId).integer().not_null())
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("post_user_id_fkey")
                            .to_tbl(User::Table)
                            .to_col(User::Id)
                            .from_tbl(Post::Table)
                            .from_col(Post::UserId)
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
    UserId
}
