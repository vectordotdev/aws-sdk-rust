// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_service_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_service::UpdateServiceInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cluster {
        object.key("cluster").string(var_1.as_str());
    }
    if let Some(var_2) = &input.service {
        object.key("service").string(var_2.as_str());
    }
    if let Some(var_3) = &input.desired_count {
        object.key("desiredCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.task_definition {
        object.key("taskDefinition").string(var_4.as_str());
    }
    if let Some(var_5) = &input.capacity_provider_strategy {
        let mut array_6 = object.key("capacityProviderStrategy").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_capacity_provider_strategy_item::ser_capacity_provider_strategy_item(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.deployment_configuration {
        #[allow(unused_mut)]
        let mut object_10 = object.key("deploymentConfiguration").start_object();
        crate::protocol_serde::shape_deployment_configuration::ser_deployment_configuration(
            &mut object_10,
            var_9,
        )?;
        object_10.finish();
    }
    if let Some(var_11) = &input.network_configuration {
        #[allow(unused_mut)]
        let mut object_12 = object.key("networkConfiguration").start_object();
        crate::protocol_serde::shape_network_configuration::ser_network_configuration(
            &mut object_12,
            var_11,
        )?;
        object_12.finish();
    }
    if let Some(var_13) = &input.placement_constraints {
        let mut array_14 = object.key("placementConstraints").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_placement_constraint::ser_placement_constraint(
                    &mut object_16,
                    item_15,
                )?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.placement_strategy {
        let mut array_18 = object.key("placementStrategy").start_array();
        for item_19 in var_17 {
            {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::protocol_serde::shape_placement_strategy::ser_placement_strategy(
                    &mut object_20,
                    item_19,
                )?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    if let Some(var_21) = &input.platform_version {
        object.key("platformVersion").string(var_21.as_str());
    }
    if let Some(var_22) = &input.force_new_deployment {
        object.key("forceNewDeployment").boolean(*var_22);
    }
    if let Some(var_23) = &input.health_check_grace_period_seconds {
        object.key("healthCheckGracePeriodSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_23).into()),
        );
    }
    if let Some(var_24) = &input.enable_execute_command {
        object.key("enableExecuteCommand").boolean(*var_24);
    }
    if let Some(var_25) = &input.enable_ecs_managed_tags {
        object.key("enableECSManagedTags").boolean(*var_25);
    }
    if let Some(var_26) = &input.load_balancers {
        let mut array_27 = object.key("loadBalancers").start_array();
        for item_28 in var_26 {
            {
                #[allow(unused_mut)]
                let mut object_29 = array_27.value().start_object();
                crate::protocol_serde::shape_load_balancer::ser_load_balancer(
                    &mut object_29,
                    item_28,
                )?;
                object_29.finish();
            }
        }
        array_27.finish();
    }
    if let Some(var_30) = &input.propagate_tags {
        object.key("propagateTags").string(var_30.as_str());
    }
    if let Some(var_31) = &input.service_registries {
        let mut array_32 = object.key("serviceRegistries").start_array();
        for item_33 in var_31 {
            {
                #[allow(unused_mut)]
                let mut object_34 = array_32.value().start_object();
                crate::protocol_serde::shape_service_registry::ser_service_registry(
                    &mut object_34,
                    item_33,
                )?;
                object_34.finish();
            }
        }
        array_32.finish();
    }
    if let Some(var_35) = &input.service_connect_configuration {
        #[allow(unused_mut)]
        let mut object_36 = object.key("serviceConnectConfiguration").start_object();
        crate::protocol_serde::shape_service_connect_configuration::ser_service_connect_configuration(&mut object_36, var_35)?;
        object_36.finish();
    }
    Ok(())
}
