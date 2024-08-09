pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240612_072520_acstor_milestone_table;
mod m20240612_072840_acstor_key_feature_table;
mod m20240612_073251_acstor_add_milestone_key_feature_table;
mod m20240612_075342_acstor_storage_type_table;
mod m20240612_075543_acstor_add_storage_type_key_feature_table;
mod m20240612_080823_acstor_workload_table;
mod m20240612_082213_add_acstor_workload_key_feature_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240612_072520_acstor_milestone_table::Migration),
            Box::new(m20240612_072840_acstor_key_feature_table::Migration),
            Box::new(m20240612_073251_acstor_add_milestone_key_feature_table::Migration),
            Box::new(m20240612_075342_acstor_storage_type_table::Migration),
            Box::new(m20240612_075543_acstor_add_storage_type_key_feature_table::Migration),
            Box::new(m20240612_080823_acstor_workload_table::Migration),
            Box::new(m20240612_082213_add_acstor_workload_key_feature_table::Migration),
        ]
    }
}
