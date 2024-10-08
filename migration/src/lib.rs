pub use sea_orm_migration::prelude::*;

mod m20240811_144718_create_question_table;
mod m20240811_145111_create_page_table;
mod m20240814_153908_create_survey_table;
mod m20240816_073728_create_answer_table;
mod m20240817_030711_add_required_to_question;
mod m20240817_035143_add_control_to_survey;
mod m20240817_100708_create_admin_table;
mod m20240817_154332_create_score_table;
mod m20240818_142432_add_answer_to_question;
mod m20240825_093046_add_completed_to_score;
mod m20240826_053333_add_previous_to_page;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240811_144718_create_question_table::Migration),
            Box::new(m20240811_145111_create_page_table::Migration),
            Box::new(m20240814_153908_create_survey_table::Migration),
            Box::new(m20240816_073728_create_answer_table::Migration),
            Box::new(m20240817_030711_add_required_to_question::Migration),
            Box::new(m20240817_035143_add_control_to_survey::Migration),
            Box::new(m20240817_100708_create_admin_table::Migration),
            Box::new(m20240817_154332_create_score_table::Migration),
            Box::new(m20240818_142432_add_answer_to_question::Migration),
            Box::new(m20240825_093046_add_completed_to_score::Migration),
            Box::new(m20240826_053333_add_previous_to_page::Migration),
        ]
    }
}
