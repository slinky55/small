pub use sea_orm_migration::prelude::*;

mod m20230309_144142_create_user_table;
mod m20230309_144155_create_posts_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230309_144142_create_user_table::Migration),
            Box::new(m20230309_144155_create_posts_table::Migration),
        ]
    }
}
