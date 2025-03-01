// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_engine_versions_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_engine_versions::DescribeEngineVersionsInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.engine_version {
        object.key("EngineVersion").string(var_1.as_str());
    }
    if let Some(var_2) = &input.parameter_group_family {
        object.key("ParameterGroupFamily").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.next_token {
        object.key("NextToken").string(var_4.as_str());
    }
    if input.default_only {
        object.key("DefaultOnly").boolean(input.default_only);
    }
    Ok(())
}
