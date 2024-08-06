use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        assert_eq!(Posts::Table.to_string(), "posts");
        assert_eq!(Posts::Id.to_string(), "id");
        assert_eq!(Posts::Title.to_string(), "title");
        assert_eq!(Posts::Content.to_string(), "content");
        assert_eq!(Posts::Excerpt.to_string(), "excerpt");

        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(pk_auto(Posts::Id))
                    .col(text(Posts::Title))
                    .col(text(Posts::Content))
                    .col(text(Posts::Excerpt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
    Content,
    Title,
    Excerpt,
}
