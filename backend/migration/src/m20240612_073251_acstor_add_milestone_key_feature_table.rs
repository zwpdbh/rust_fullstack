use sea_orm::ForeignKeyAction;
use sea_orm_migration::prelude::*;

use crate::{
    m20240612_072520_acstor_milestone_table::Milestone,
    m20240612_072840_acstor_key_feature_table::KeyFeature,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MilestoneKeyfeature::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MilestoneKeyfeature::MilestoneId).integer())
                    .col(ColumnDef::new(MilestoneKeyfeature::KeyFeatureId).integer())
                    .primary_key(
                        Index::create()
                            .name("pk_milestone_key_feature")
                            .col(MilestoneKeyfeature::MilestoneId)
                            .col(MilestoneKeyfeature::KeyFeatureId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_milestone_key_feature_milestone_id")
                            .from(MilestoneKeyfeature::Table, MilestoneKeyfeature::MilestoneId)
                            .to(Milestone::Table, Milestone::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_milestone_key_feature_key_feature_id")
                            .from(
                                MilestoneKeyfeature::Table,
                                MilestoneKeyfeature::KeyFeatureId,
                            )
                            .to(KeyFeature::Table, KeyFeature::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MilestoneKeyfeature::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MilestoneKeyfeature {
    Table,
    MilestoneId,
    KeyFeatureId,
}
