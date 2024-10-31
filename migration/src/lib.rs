pub use sea_orm_migration::prelude::*;

mod m20241024_095446_create_survey_table;
mod m20241024_100652_create_page_table;
mod m20241024_101119_create_question_table;
mod m20241024_142752_create_user_table;
mod m20241028_063603_create_score_table;
mod m20241031_045157_add_password_and_source;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241024_095446_create_survey_table::Migration),
            Box::new(m20241024_100652_create_page_table::Migration),
            Box::new(m20241024_101119_create_question_table::Migration),
            Box::new(m20241024_142752_create_user_table::Migration),
            Box::new(m20241028_063603_create_score_table::Migration),
            Box::new(m20241031_045157_add_password_and_source::Migration),
        ]
    }
}
