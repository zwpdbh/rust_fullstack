use sea_orm::ForeignKeyAction;
use sea_orm_migration::prelude::*;

use crate::m20240612_072840_acstor_key_feature_table::KeyFeature;
use crate::m20240612_080823_acstor_workload_table::Workload;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorkloadKeyfeature::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(WorkloadKeyfeature::WorkloadId).integer())
                    .col(ColumnDef::new(WorkloadKeyfeature::KeyfeatureId).integer())
                    .primary_key(
                        Index::create()
                            .name("pk_workload_key_feature")
                            .col(WorkloadKeyfeature::WorkloadId)
                            .col(WorkloadKeyfeature::KeyfeatureId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workload_key_feature_workload_id")
                            .from(WorkloadKeyfeature::Table, WorkloadKeyfeature::WorkloadId)
                            .to(Workload::Table, Workload::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workload_key_feature_key_feature_id")
                            .from(WorkloadKeyfeature::Table, WorkloadKeyfeature::KeyfeatureId)
                            .to(KeyFeature::Table, KeyFeature::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WorkloadKeyfeature::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum WorkloadKeyfeature {
    Table,
    WorkloadId,
    KeyfeatureId,
}
