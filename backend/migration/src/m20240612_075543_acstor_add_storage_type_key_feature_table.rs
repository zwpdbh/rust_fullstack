use sea_orm::ForeignKeyAction;
use sea_orm_migration::prelude::*;

use crate::{
    m20240612_072840_acstor_key_feature_table::KeyFeature,
    m20240612_075342_acstor_storage_type_table::StorageType,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StorageTypeKeyfeature::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(StorageTypeKeyfeature::StorageTypeId).integer())
                    .col(ColumnDef::new(StorageTypeKeyfeature::KeyFeatureId).integer())
                    .primary_key(
                        Index::create()
                            .name("pk_storage_type_key_feature")
                            .col(StorageTypeKeyfeature::StorageTypeId)
                            .col(StorageTypeKeyfeature::KeyFeatureId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_storage_type_key_feature_storage_type_id")
                            .from(
                                StorageTypeKeyfeature::Table,
                                StorageTypeKeyfeature::StorageTypeId,
                            )
                            .to(StorageType::Table, StorageType::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_storage_type_key_feature_key_feature_id")
                            .from(
                                StorageTypeKeyfeature::Table,
                                StorageTypeKeyfeature::KeyFeatureId,
                            )
                            .to(KeyFeature::Table, KeyFeature::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(StorageTypeKeyfeature::Status)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StorageTypeKeyfeature::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StorageTypeKeyfeature {
    Table,
    StorageTypeId,
    KeyFeatureId,
    Status,
}
