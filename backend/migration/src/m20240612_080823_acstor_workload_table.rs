use sea_orm::ForeignKeyAction;
use sea_orm_migration::prelude::*;

use crate::m20240612_075342_acstor_storage_type_table::StorageType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Workload::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Workload::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Workload::Name).string().not_null())
                    .col(ColumnDef::new(Workload::StorageTypeId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_workload_storage_type_id")
                            .from(Workload::Table, Workload::StorageTypeId)
                            .to(StorageType::Table, StorageType::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Workload::Status).string().not_null())
                    .col(ColumnDef::new(Workload::ReleaseVersion).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Workload::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Workload {
    Table,
    Id,
    Name,
    StorageTypeId,
    Status,
    ReleaseVersion,
}
