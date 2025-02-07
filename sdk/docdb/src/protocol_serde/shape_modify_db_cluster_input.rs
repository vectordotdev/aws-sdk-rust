// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_db_cluster_input_input(
    input: &crate::operation::modify_db_cluster::ModifyDbClusterInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError>
{
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyDBCluster", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DBClusterIdentifier");
    if let Some(var_2) = &input.db_cluster_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("NewDBClusterIdentifier");
    if let Some(var_4) = &input.new_db_cluster_identifier {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("ApplyImmediately");
    if input.apply_immediately {
        scope_5.boolean(input.apply_immediately);
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("BackupRetentionPeriod");
    if let Some(var_7) = &input.backup_retention_period {
        scope_6.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("DBClusterParameterGroupName");
    if let Some(var_9) = &input.db_cluster_parameter_group_name {
        scope_8.string(var_9);
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("VpcSecurityGroupIds");
    if let Some(var_11) = &input.vpc_security_group_ids {
        let mut list_13 = scope_10.start_list(false, Some("VpcSecurityGroupId"));
        for item_12 in var_11 {
            #[allow(unused_mut)]
            let mut entry_14 = list_13.entry();
            entry_14.string(item_12);
        }
        list_13.finish();
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("Port");
    if let Some(var_16) = &input.port {
        scope_15.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_16).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("MasterUserPassword");
    if let Some(var_18) = &input.master_user_password {
        scope_17.string(var_18);
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("PreferredBackupWindow");
    if let Some(var_20) = &input.preferred_backup_window {
        scope_19.string(var_20);
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("PreferredMaintenanceWindow");
    if let Some(var_22) = &input.preferred_maintenance_window {
        scope_21.string(var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("CloudwatchLogsExportConfiguration");
    if let Some(var_24) = &input.cloudwatch_logs_export_configuration {
        crate::protocol_serde::shape_cloudwatch_logs_export_configuration::ser_cloudwatch_logs_export_configuration(scope_23, var_24)?;
    }
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("EngineVersion");
    if let Some(var_26) = &input.engine_version {
        scope_25.string(var_26);
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("DeletionProtection");
    if let Some(var_28) = &input.deletion_protection {
        scope_27.boolean(*var_28);
    }
    writer.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}
