// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_member_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateMemberInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.invitation_id {
        object.key("InvitationId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.member_configuration {
        let mut object_4 = object.key("MemberConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_member_configuration(
            &mut object_4,
            var_3,
        )?;
        object_4.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_network_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateNetworkInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_5) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.description {
        object.key("Description").string(var_6.as_str());
    }
    if let Some(var_7) = &input.framework {
        object.key("Framework").string(var_7.as_str());
    }
    if let Some(var_8) = &input.framework_configuration {
        let mut object_9 = object.key("FrameworkConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_network_framework_configuration(
            &mut object_9,
            var_8,
        )?;
        object_9.finish();
    }
    if let Some(var_10) = &input.framework_version {
        object.key("FrameworkVersion").string(var_10.as_str());
    }
    if let Some(var_11) = &input.member_configuration {
        let mut object_12 = object.key("MemberConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_member_configuration(
            &mut object_12,
            var_11,
        )?;
        object_12.finish();
    }
    if let Some(var_13) = &input.name {
        object.key("Name").string(var_13.as_str());
    }
    if let Some(var_14) = &input.tags {
        let mut object_15 = object.key("Tags").start_object();
        for (key_16, value_17) in var_14 {
            {
                object_15.key(key_16.as_str()).string(value_17.as_str());
            }
        }
        object_15.finish();
    }
    if let Some(var_18) = &input.voting_policy {
        let mut object_19 = object.key("VotingPolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_voting_policy(&mut object_19, var_18)?;
        object_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_node_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateNodeInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_20) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_20.as_str());
    }
    if let Some(var_21) = &input.member_id {
        object.key("MemberId").string(var_21.as_str());
    }
    if let Some(var_22) = &input.node_configuration {
        let mut object_23 = object.key("NodeConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_node_configuration(
            &mut object_23,
            var_22,
        )?;
        object_23.finish();
    }
    if let Some(var_24) = &input.tags {
        let mut object_25 = object.key("Tags").start_object();
        for (key_26, value_27) in var_24 {
            {
                object_25.key(key_26.as_str()).string(value_27.as_str());
            }
        }
        object_25.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_proposal_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateProposalInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_28) = &input.actions {
        let mut object_29 = object.key("Actions").start_object();
        crate::json_ser::serialize_structure_crate_model_proposal_actions(&mut object_29, var_28)?;
        object_29.finish();
    }
    if let Some(var_30) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_30.as_str());
    }
    if let Some(var_31) = &input.description {
        object.key("Description").string(var_31.as_str());
    }
    if let Some(var_32) = &input.member_id {
        object.key("MemberId").string(var_32.as_str());
    }
    if let Some(var_33) = &input.tags {
        let mut object_34 = object.key("Tags").start_object();
        for (key_35, value_36) in var_33 {
            {
                object_34.key(key_35.as_str()).string(value_36.as_str());
            }
        }
        object_34.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_37) = &input.tags {
        let mut object_38 = object.key("Tags").start_object();
        for (key_39, value_40) in var_37 {
            {
                object_38.key(key_39.as_str()).string(value_40.as_str());
            }
        }
        object_38.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_member_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateMemberInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_41) = &input.log_publishing_configuration {
        let mut object_42 = object.key("LogPublishingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_member_log_publishing_configuration(
            &mut object_42,
            var_41,
        )?;
        object_42.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_node_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateNodeInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_43) = &input.log_publishing_configuration {
        let mut object_44 = object.key("LogPublishingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_node_log_publishing_configuration(
            &mut object_44,
            var_43,
        )?;
        object_44.finish();
    }
    if let Some(var_45) = &input.member_id {
        object.key("MemberId").string(var_45.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_vote_on_proposal_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::VoteOnProposalInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_46) = &input.vote {
        object.key("Vote").string(var_46.as_str());
    }
    if let Some(var_47) = &input.voter_member_id {
        object.key("VoterMemberId").string(var_47.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_member_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MemberConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_48) = &input.name {
        object.key("Name").string(var_48.as_str());
    }
    if let Some(var_49) = &input.description {
        object.key("Description").string(var_49.as_str());
    }
    if let Some(var_50) = &input.framework_configuration {
        let mut object_51 = object.key("FrameworkConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_member_framework_configuration(
            &mut object_51,
            var_50,
        )?;
        object_51.finish();
    }
    if let Some(var_52) = &input.log_publishing_configuration {
        let mut object_53 = object.key("LogPublishingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_member_log_publishing_configuration(
            &mut object_53,
            var_52,
        )?;
        object_53.finish();
    }
    if let Some(var_54) = &input.tags {
        let mut object_55 = object.key("Tags").start_object();
        for (key_56, value_57) in var_54 {
            {
                object_55.key(key_56.as_str()).string(value_57.as_str());
            }
        }
        object_55.finish();
    }
    if let Some(var_58) = &input.kms_key_arn {
        object.key("KmsKeyArn").string(var_58.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_network_framework_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NetworkFrameworkConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_59) = &input.fabric {
        let mut object_60 = object.key("Fabric").start_object();
        crate::json_ser::serialize_structure_crate_model_network_fabric_configuration(
            &mut object_60,
            var_59,
        )?;
        object_60.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_voting_policy(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VotingPolicy,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_61) = &input.approval_threshold_policy {
        let mut object_62 = object.key("ApprovalThresholdPolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_approval_threshold_policy(
            &mut object_62,
            var_61,
        )?;
        object_62.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_node_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NodeConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_63) = &input.instance_type {
        object.key("InstanceType").string(var_63.as_str());
    }
    if let Some(var_64) = &input.availability_zone {
        object.key("AvailabilityZone").string(var_64.as_str());
    }
    if let Some(var_65) = &input.log_publishing_configuration {
        let mut object_66 = object.key("LogPublishingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_node_log_publishing_configuration(
            &mut object_66,
            var_65,
        )?;
        object_66.finish();
    }
    if let Some(var_67) = &input.state_db {
        object.key("StateDB").string(var_67.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_proposal_actions(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ProposalActions,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_68) = &input.invitations {
        let mut array_69 = object.key("Invitations").start_array();
        for item_70 in var_68 {
            {
                let mut object_71 = array_69.value().start_object();
                crate::json_ser::serialize_structure_crate_model_invite_action(
                    &mut object_71,
                    item_70,
                )?;
                object_71.finish();
            }
        }
        array_69.finish();
    }
    if let Some(var_72) = &input.removals {
        let mut array_73 = object.key("Removals").start_array();
        for item_74 in var_72 {
            {
                let mut object_75 = array_73.value().start_object();
                crate::json_ser::serialize_structure_crate_model_remove_action(
                    &mut object_75,
                    item_74,
                )?;
                object_75.finish();
            }
        }
        array_73.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_member_log_publishing_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MemberLogPublishingConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_76) = &input.fabric {
        let mut object_77 = object.key("Fabric").start_object();
        crate::json_ser::serialize_structure_crate_model_member_fabric_log_publishing_configuration(&mut object_77, var_76)?;
        object_77.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_node_log_publishing_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NodeLogPublishingConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_78) = &input.fabric {
        let mut object_79 = object.key("Fabric").start_object();
        crate::json_ser::serialize_structure_crate_model_node_fabric_log_publishing_configuration(
            &mut object_79,
            var_78,
        )?;
        object_79.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_member_framework_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MemberFrameworkConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_80) = &input.fabric {
        let mut object_81 = object.key("Fabric").start_object();
        crate::json_ser::serialize_structure_crate_model_member_fabric_configuration(
            &mut object_81,
            var_80,
        )?;
        object_81.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_network_fabric_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NetworkFabricConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_82) = &input.edition {
        object.key("Edition").string(var_82.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_approval_threshold_policy(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ApprovalThresholdPolicy,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_83) = &input.threshold_percentage {
        object.key("ThresholdPercentage").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_83).into()),
        );
    }
    if let Some(var_84) = &input.proposal_duration_in_hours {
        object.key("ProposalDurationInHours").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_84).into()),
        );
    }
    if let Some(var_85) = &input.threshold_comparator {
        object.key("ThresholdComparator").string(var_85.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_invite_action(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::InviteAction,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_86) = &input.principal {
        object.key("Principal").string(var_86.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_remove_action(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RemoveAction,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_87) = &input.member_id {
        object.key("MemberId").string(var_87.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_member_fabric_log_publishing_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MemberFabricLogPublishingConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_88) = &input.ca_logs {
        let mut object_89 = object.key("CaLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_log_configurations(
            &mut object_89,
            var_88,
        )?;
        object_89.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_node_fabric_log_publishing_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NodeFabricLogPublishingConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_90) = &input.chaincode_logs {
        let mut object_91 = object.key("ChaincodeLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_log_configurations(
            &mut object_91,
            var_90,
        )?;
        object_91.finish();
    }
    if let Some(var_92) = &input.peer_logs {
        let mut object_93 = object.key("PeerLogs").start_object();
        crate::json_ser::serialize_structure_crate_model_log_configurations(
            &mut object_93,
            var_92,
        )?;
        object_93.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_member_fabric_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::MemberFabricConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_94) = &input.admin_username {
        object.key("AdminUsername").string(var_94.as_str());
    }
    if let Some(var_95) = &input.admin_password {
        object.key("AdminPassword").string(var_95.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_log_configurations(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LogConfigurations,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_96) = &input.cloudwatch {
        let mut object_97 = object.key("Cloudwatch").start_object();
        crate::json_ser::serialize_structure_crate_model_log_configuration(&mut object_97, var_96)?;
        object_97.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_log_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LogConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_98) = &input.enabled {
        object.key("Enabled").boolean(*var_98);
    }
    Ok(())
}
