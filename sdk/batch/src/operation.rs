// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CancelJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`cancel_job`](crate::client::Client::cancel_job).
///
/// See [`crate::client::fluent_builders::CancelJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CancelJob {
    _private: (),
}
impl CancelJob {
    /// Creates a new builder-style object to manufacture [`CancelJobInput`](crate::input::CancelJobInput).
    pub fn builder() -> crate::input::cancel_job_input::Builder {
        crate::input::cancel_job_input::Builder::default()
    }
    /// Creates a new `CancelJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelJob {
    type Output = std::result::Result<crate::output::CancelJobOutput, crate::error::CancelJobError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_cancel_job_error(response)
        } else {
            crate::operation_deser::parse_cancel_job_response(response)
        }
    }
}

/// Operation shape for `CreateComputeEnvironment`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_compute_environment`](crate::client::Client::create_compute_environment).
///
/// See [`crate::client::fluent_builders::CreateComputeEnvironment`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateComputeEnvironment {
    _private: (),
}
impl CreateComputeEnvironment {
    /// Creates a new builder-style object to manufacture [`CreateComputeEnvironmentInput`](crate::input::CreateComputeEnvironmentInput).
    pub fn builder() -> crate::input::create_compute_environment_input::Builder {
        crate::input::create_compute_environment_input::Builder::default()
    }
    /// Creates a new `CreateComputeEnvironment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateComputeEnvironment {
    type Output = std::result::Result<
        crate::output::CreateComputeEnvironmentOutput,
        crate::error::CreateComputeEnvironmentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_compute_environment_error(response)
        } else {
            crate::operation_deser::parse_create_compute_environment_response(response)
        }
    }
}

/// Operation shape for `CreateJobQueue`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_job_queue`](crate::client::Client::create_job_queue).
///
/// See [`crate::client::fluent_builders::CreateJobQueue`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateJobQueue {
    _private: (),
}
impl CreateJobQueue {
    /// Creates a new builder-style object to manufacture [`CreateJobQueueInput`](crate::input::CreateJobQueueInput).
    pub fn builder() -> crate::input::create_job_queue_input::Builder {
        crate::input::create_job_queue_input::Builder::default()
    }
    /// Creates a new `CreateJobQueue` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateJobQueue {
    type Output =
        std::result::Result<crate::output::CreateJobQueueOutput, crate::error::CreateJobQueueError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_job_queue_error(response)
        } else {
            crate::operation_deser::parse_create_job_queue_response(response)
        }
    }
}

/// Operation shape for `CreateSchedulingPolicy`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_scheduling_policy`](crate::client::Client::create_scheduling_policy).
///
/// See [`crate::client::fluent_builders::CreateSchedulingPolicy`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateSchedulingPolicy {
    _private: (),
}
impl CreateSchedulingPolicy {
    /// Creates a new builder-style object to manufacture [`CreateSchedulingPolicyInput`](crate::input::CreateSchedulingPolicyInput).
    pub fn builder() -> crate::input::create_scheduling_policy_input::Builder {
        crate::input::create_scheduling_policy_input::Builder::default()
    }
    /// Creates a new `CreateSchedulingPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateSchedulingPolicy {
    type Output = std::result::Result<
        crate::output::CreateSchedulingPolicyOutput,
        crate::error::CreateSchedulingPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_scheduling_policy_error(response)
        } else {
            crate::operation_deser::parse_create_scheduling_policy_response(response)
        }
    }
}

/// Operation shape for `DeleteComputeEnvironment`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_compute_environment`](crate::client::Client::delete_compute_environment).
///
/// See [`crate::client::fluent_builders::DeleteComputeEnvironment`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteComputeEnvironment {
    _private: (),
}
impl DeleteComputeEnvironment {
    /// Creates a new builder-style object to manufacture [`DeleteComputeEnvironmentInput`](crate::input::DeleteComputeEnvironmentInput).
    pub fn builder() -> crate::input::delete_compute_environment_input::Builder {
        crate::input::delete_compute_environment_input::Builder::default()
    }
    /// Creates a new `DeleteComputeEnvironment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteComputeEnvironment {
    type Output = std::result::Result<
        crate::output::DeleteComputeEnvironmentOutput,
        crate::error::DeleteComputeEnvironmentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_compute_environment_error(response)
        } else {
            crate::operation_deser::parse_delete_compute_environment_response(response)
        }
    }
}

/// Operation shape for `DeleteJobQueue`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_job_queue`](crate::client::Client::delete_job_queue).
///
/// See [`crate::client::fluent_builders::DeleteJobQueue`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteJobQueue {
    _private: (),
}
impl DeleteJobQueue {
    /// Creates a new builder-style object to manufacture [`DeleteJobQueueInput`](crate::input::DeleteJobQueueInput).
    pub fn builder() -> crate::input::delete_job_queue_input::Builder {
        crate::input::delete_job_queue_input::Builder::default()
    }
    /// Creates a new `DeleteJobQueue` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteJobQueue {
    type Output =
        std::result::Result<crate::output::DeleteJobQueueOutput, crate::error::DeleteJobQueueError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_job_queue_error(response)
        } else {
            crate::operation_deser::parse_delete_job_queue_response(response)
        }
    }
}

/// Operation shape for `DeleteSchedulingPolicy`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_scheduling_policy`](crate::client::Client::delete_scheduling_policy).
///
/// See [`crate::client::fluent_builders::DeleteSchedulingPolicy`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSchedulingPolicy {
    _private: (),
}
impl DeleteSchedulingPolicy {
    /// Creates a new builder-style object to manufacture [`DeleteSchedulingPolicyInput`](crate::input::DeleteSchedulingPolicyInput).
    pub fn builder() -> crate::input::delete_scheduling_policy_input::Builder {
        crate::input::delete_scheduling_policy_input::Builder::default()
    }
    /// Creates a new `DeleteSchedulingPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteSchedulingPolicy {
    type Output = std::result::Result<
        crate::output::DeleteSchedulingPolicyOutput,
        crate::error::DeleteSchedulingPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_scheduling_policy_error(response)
        } else {
            crate::operation_deser::parse_delete_scheduling_policy_response(response)
        }
    }
}

/// Operation shape for `DeregisterJobDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`deregister_job_definition`](crate::client::Client::deregister_job_definition).
///
/// See [`crate::client::fluent_builders::DeregisterJobDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeregisterJobDefinition {
    _private: (),
}
impl DeregisterJobDefinition {
    /// Creates a new builder-style object to manufacture [`DeregisterJobDefinitionInput`](crate::input::DeregisterJobDefinitionInput).
    pub fn builder() -> crate::input::deregister_job_definition_input::Builder {
        crate::input::deregister_job_definition_input::Builder::default()
    }
    /// Creates a new `DeregisterJobDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeregisterJobDefinition {
    type Output = std::result::Result<
        crate::output::DeregisterJobDefinitionOutput,
        crate::error::DeregisterJobDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_deregister_job_definition_error(response)
        } else {
            crate::operation_deser::parse_deregister_job_definition_response(response)
        }
    }
}

/// Operation shape for `DescribeComputeEnvironments`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_compute_environments`](crate::client::Client::describe_compute_environments).
///
/// See [`crate::client::fluent_builders::DescribeComputeEnvironments`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeComputeEnvironments {
    _private: (),
}
impl DescribeComputeEnvironments {
    /// Creates a new builder-style object to manufacture [`DescribeComputeEnvironmentsInput`](crate::input::DescribeComputeEnvironmentsInput).
    pub fn builder() -> crate::input::describe_compute_environments_input::Builder {
        crate::input::describe_compute_environments_input::Builder::default()
    }
    /// Creates a new `DescribeComputeEnvironments` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeComputeEnvironments {
    type Output = std::result::Result<
        crate::output::DescribeComputeEnvironmentsOutput,
        crate::error::DescribeComputeEnvironmentsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_compute_environments_error(response)
        } else {
            crate::operation_deser::parse_describe_compute_environments_response(response)
        }
    }
}
#[cfg(test)]
#[allow(unreachable_code, unused_variables)]
mod describe_compute_environments_request_test {
    /// This test case validates a bug where unboxed primitives were incorrectly marked as required
    /// Test ID: DeserializeDescribeCompute
    #[tokio::test]
    async fn deserialize_describe_compute_response() {
        let expected_output = crate::output::DescribeComputeEnvironmentsOutput::builder()
            .set_compute_environments(Some(vec![crate::model::ComputeEnvironmentDetail::builder(
            )
            .set_compute_environment_name(Some("test-batch-compute".to_owned()))
            .set_compute_environment_arn(Some("arn".to_owned()))
            .set_ecs_cluster_arn(Some("clusteran".to_owned()))
            .set_tags(Some({
                let mut ret = std::collections::HashMap::new();
                ret.insert("foo".to_owned(), "bar".to_owned());
                ret
            }))
            .set_type(Some(crate::model::CeType::from("MANAGED")))
            .set_state(Some(crate::model::CeState::from("ENABLED")))
            .set_status(Some(crate::model::CeStatus::from("VALID")))
            .set_status_reason(Some("ComputeEnvironment Healthy".to_owned()))
            .set_compute_resources(Some(
                crate::model::ComputeResource::builder()
                    .set_type(Some(crate::model::CrType::from("EC2")))
                    .set_minv_cpus(Some(0))
                    .set_maxv_cpus(Some(256))
                    .set_desiredv_cpus(Some(0))
                    .set_instance_types(Some(vec!["optimal".to_owned()]))
                    .set_subnets(Some(vec![
                        "subnet-c745b79c".to_owned(),
                        "subnet-d4e24fe8".to_owned(),
                    ]))
                    .set_security_group_ids(Some(vec!["sg-06a55e7b".to_owned()]))
                    .set_instance_role(Some("instancerole".to_owned()))
                    .set_tags(Some({
                        let mut ret = std::collections::HashMap::new();
                        ret.insert("Name".to_owned(), "batch-compute".to_owned());
                        ret
                    }))
                    .set_ec2_configuration(Some(vec![crate::model::Ec2Configuration::builder()
                        .set_image_type(Some("ECS_AL1".to_owned()))
                        .build()]))
                    .build(),
            ))
            .set_service_role(Some(
                "arn:aws:iam::432762038596:role/service-role/AWSBatchServiceRole".to_owned(),
            ))
            .build()]))
            .build();
        let http_response = http::response::Builder::new()
        .status(200)
                    .body(aws_smithy_http::body::SdkBody::from("    {\n        \"computeEnvironments\":[{\n            \"computeEnvironmentName\":\"test-batch-compute\",\n            \"computeEnvironmentArn\":\"arn\",\n            \"ecsClusterArn\":\"clusteran\",\n            \"tags\":{\"foo\": \"bar\"},\n            \"type\":\"MANAGED\",\n            \"state\":\"ENABLED\",\n            \"status\":\"VALID\",\n            \"statusReason\":\"ComputeEnvironment Healthy\",\n            \"computeResources\":{\n                \"type\":\"EC2\",\n                \"minvCpus\":0,\n                \"maxvCpus\":256,\n                \"desiredvCpus\":0,\n                \"instanceTypes\":[\"optimal\"],\n                \"subnets\":[\"subnet-c745b79c\",\"subnet-d4e24fe8\"],\n                \"securityGroupIds\":[\"sg-06a55e7b\"],\n                \"instanceRole\":\"instancerole\",\n                \"tags\":{\"Name\":\"batch-compute\"},\n                \"ec2Configuration\":[{\"imageType\":\"ECS_AL1\"}]\n            },\n            \"serviceRole\":\"arn:aws:iam::432762038596:role/service-role/AWSBatchServiceRole\"\n        }]\n    }\n"))
                    .unwrap();
        let mut op_response = aws_smithy_http::operation::Response::new(http_response);
        use aws_smithy_http::response::ParseHttpResponse;
        let parser = crate::operation::DescribeComputeEnvironments::new();
        let parsed = parser.parse_unloaded(&mut op_response);
        let parsed = parsed.unwrap_or_else(|| {
                        let (http_response, _) = op_response.into_parts();
                        let http_response = http_response.map(|body|bytes::Bytes::copy_from_slice(body.bytes().unwrap()));
                        <crate::operation::DescribeComputeEnvironments as aws_smithy_http::response::ParseHttpResponse>::parse_loaded(&parser, &http_response)
                    });
        let parsed = parsed.unwrap();
        pretty_assertions::assert_eq!(
            parsed.compute_environments,
            expected_output.compute_environments,
            "Unexpected value for `compute_environments`"
        );
        pretty_assertions::assert_eq!(
            parsed.next_token,
            expected_output.next_token,
            "Unexpected value for `next_token`"
        );
    }
}

/// Operation shape for `DescribeJobDefinitions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_job_definitions`](crate::client::Client::describe_job_definitions).
///
/// See [`crate::client::fluent_builders::DescribeJobDefinitions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeJobDefinitions {
    _private: (),
}
impl DescribeJobDefinitions {
    /// Creates a new builder-style object to manufacture [`DescribeJobDefinitionsInput`](crate::input::DescribeJobDefinitionsInput).
    pub fn builder() -> crate::input::describe_job_definitions_input::Builder {
        crate::input::describe_job_definitions_input::Builder::default()
    }
    /// Creates a new `DescribeJobDefinitions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeJobDefinitions {
    type Output = std::result::Result<
        crate::output::DescribeJobDefinitionsOutput,
        crate::error::DescribeJobDefinitionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_job_definitions_error(response)
        } else {
            crate::operation_deser::parse_describe_job_definitions_response(response)
        }
    }
}

/// Operation shape for `DescribeJobQueues`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_job_queues`](crate::client::Client::describe_job_queues).
///
/// See [`crate::client::fluent_builders::DescribeJobQueues`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeJobQueues {
    _private: (),
}
impl DescribeJobQueues {
    /// Creates a new builder-style object to manufacture [`DescribeJobQueuesInput`](crate::input::DescribeJobQueuesInput).
    pub fn builder() -> crate::input::describe_job_queues_input::Builder {
        crate::input::describe_job_queues_input::Builder::default()
    }
    /// Creates a new `DescribeJobQueues` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeJobQueues {
    type Output = std::result::Result<
        crate::output::DescribeJobQueuesOutput,
        crate::error::DescribeJobQueuesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_job_queues_error(response)
        } else {
            crate::operation_deser::parse_describe_job_queues_response(response)
        }
    }
}

/// Operation shape for `DescribeJobs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_jobs`](crate::client::Client::describe_jobs).
///
/// See [`crate::client::fluent_builders::DescribeJobs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeJobs {
    _private: (),
}
impl DescribeJobs {
    /// Creates a new builder-style object to manufacture [`DescribeJobsInput`](crate::input::DescribeJobsInput).
    pub fn builder() -> crate::input::describe_jobs_input::Builder {
        crate::input::describe_jobs_input::Builder::default()
    }
    /// Creates a new `DescribeJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeJobs {
    type Output =
        std::result::Result<crate::output::DescribeJobsOutput, crate::error::DescribeJobsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_jobs_error(response)
        } else {
            crate::operation_deser::parse_describe_jobs_response(response)
        }
    }
}

/// Operation shape for `DescribeSchedulingPolicies`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_scheduling_policies`](crate::client::Client::describe_scheduling_policies).
///
/// See [`crate::client::fluent_builders::DescribeSchedulingPolicies`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeSchedulingPolicies {
    _private: (),
}
impl DescribeSchedulingPolicies {
    /// Creates a new builder-style object to manufacture [`DescribeSchedulingPoliciesInput`](crate::input::DescribeSchedulingPoliciesInput).
    pub fn builder() -> crate::input::describe_scheduling_policies_input::Builder {
        crate::input::describe_scheduling_policies_input::Builder::default()
    }
    /// Creates a new `DescribeSchedulingPolicies` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeSchedulingPolicies {
    type Output = std::result::Result<
        crate::output::DescribeSchedulingPoliciesOutput,
        crate::error::DescribeSchedulingPoliciesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_scheduling_policies_error(response)
        } else {
            crate::operation_deser::parse_describe_scheduling_policies_response(response)
        }
    }
}

/// Operation shape for `ListJobs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_jobs`](crate::client::Client::list_jobs).
///
/// See [`crate::client::fluent_builders::ListJobs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListJobs {
    _private: (),
}
impl ListJobs {
    /// Creates a new builder-style object to manufacture [`ListJobsInput`](crate::input::ListJobsInput).
    pub fn builder() -> crate::input::list_jobs_input::Builder {
        crate::input::list_jobs_input::Builder::default()
    }
    /// Creates a new `ListJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListJobs {
    type Output = std::result::Result<crate::output::ListJobsOutput, crate::error::ListJobsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_jobs_response(response)
        }
    }
}

/// Operation shape for `ListSchedulingPolicies`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_scheduling_policies`](crate::client::Client::list_scheduling_policies).
///
/// See [`crate::client::fluent_builders::ListSchedulingPolicies`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListSchedulingPolicies {
    _private: (),
}
impl ListSchedulingPolicies {
    /// Creates a new builder-style object to manufacture [`ListSchedulingPoliciesInput`](crate::input::ListSchedulingPoliciesInput).
    pub fn builder() -> crate::input::list_scheduling_policies_input::Builder {
        crate::input::list_scheduling_policies_input::Builder::default()
    }
    /// Creates a new `ListSchedulingPolicies` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSchedulingPolicies {
    type Output = std::result::Result<
        crate::output::ListSchedulingPoliciesOutput,
        crate::error::ListSchedulingPoliciesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_scheduling_policies_error(response)
        } else {
            crate::operation_deser::parse_list_scheduling_policies_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `RegisterJobDefinition`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`register_job_definition`](crate::client::Client::register_job_definition).
///
/// See [`crate::client::fluent_builders::RegisterJobDefinition`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RegisterJobDefinition {
    _private: (),
}
impl RegisterJobDefinition {
    /// Creates a new builder-style object to manufacture [`RegisterJobDefinitionInput`](crate::input::RegisterJobDefinitionInput).
    pub fn builder() -> crate::input::register_job_definition_input::Builder {
        crate::input::register_job_definition_input::Builder::default()
    }
    /// Creates a new `RegisterJobDefinition` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RegisterJobDefinition {
    type Output = std::result::Result<
        crate::output::RegisterJobDefinitionOutput,
        crate::error::RegisterJobDefinitionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_register_job_definition_error(response)
        } else {
            crate::operation_deser::parse_register_job_definition_response(response)
        }
    }
}

/// Operation shape for `SubmitJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`submit_job`](crate::client::Client::submit_job).
///
/// See [`crate::client::fluent_builders::SubmitJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SubmitJob {
    _private: (),
}
impl SubmitJob {
    /// Creates a new builder-style object to manufacture [`SubmitJobInput`](crate::input::SubmitJobInput).
    pub fn builder() -> crate::input::submit_job_input::Builder {
        crate::input::submit_job_input::Builder::default()
    }
    /// Creates a new `SubmitJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SubmitJob {
    type Output = std::result::Result<crate::output::SubmitJobOutput, crate::error::SubmitJobError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_submit_job_error(response)
        } else {
            crate::operation_deser::parse_submit_job_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `TerminateJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`terminate_job`](crate::client::Client::terminate_job).
///
/// See [`crate::client::fluent_builders::TerminateJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TerminateJob {
    _private: (),
}
impl TerminateJob {
    /// Creates a new builder-style object to manufacture [`TerminateJobInput`](crate::input::TerminateJobInput).
    pub fn builder() -> crate::input::terminate_job_input::Builder {
        crate::input::terminate_job_input::Builder::default()
    }
    /// Creates a new `TerminateJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TerminateJob {
    type Output =
        std::result::Result<crate::output::TerminateJobOutput, crate::error::TerminateJobError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_terminate_job_error(response)
        } else {
            crate::operation_deser::parse_terminate_job_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateComputeEnvironment`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_compute_environment`](crate::client::Client::update_compute_environment).
///
/// See [`crate::client::fluent_builders::UpdateComputeEnvironment`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateComputeEnvironment {
    _private: (),
}
impl UpdateComputeEnvironment {
    /// Creates a new builder-style object to manufacture [`UpdateComputeEnvironmentInput`](crate::input::UpdateComputeEnvironmentInput).
    pub fn builder() -> crate::input::update_compute_environment_input::Builder {
        crate::input::update_compute_environment_input::Builder::default()
    }
    /// Creates a new `UpdateComputeEnvironment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateComputeEnvironment {
    type Output = std::result::Result<
        crate::output::UpdateComputeEnvironmentOutput,
        crate::error::UpdateComputeEnvironmentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_compute_environment_error(response)
        } else {
            crate::operation_deser::parse_update_compute_environment_response(response)
        }
    }
}

/// Operation shape for `UpdateJobQueue`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_job_queue`](crate::client::Client::update_job_queue).
///
/// See [`crate::client::fluent_builders::UpdateJobQueue`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateJobQueue {
    _private: (),
}
impl UpdateJobQueue {
    /// Creates a new builder-style object to manufacture [`UpdateJobQueueInput`](crate::input::UpdateJobQueueInput).
    pub fn builder() -> crate::input::update_job_queue_input::Builder {
        crate::input::update_job_queue_input::Builder::default()
    }
    /// Creates a new `UpdateJobQueue` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateJobQueue {
    type Output =
        std::result::Result<crate::output::UpdateJobQueueOutput, crate::error::UpdateJobQueueError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_job_queue_error(response)
        } else {
            crate::operation_deser::parse_update_job_queue_response(response)
        }
    }
}

/// Operation shape for `UpdateSchedulingPolicy`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_scheduling_policy`](crate::client::Client::update_scheduling_policy).
///
/// See [`crate::client::fluent_builders::UpdateSchedulingPolicy`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateSchedulingPolicy {
    _private: (),
}
impl UpdateSchedulingPolicy {
    /// Creates a new builder-style object to manufacture [`UpdateSchedulingPolicyInput`](crate::input::UpdateSchedulingPolicyInput).
    pub fn builder() -> crate::input::update_scheduling_policy_input::Builder {
        crate::input::update_scheduling_policy_input::Builder::default()
    }
    /// Creates a new `UpdateSchedulingPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateSchedulingPolicy {
    type Output = std::result::Result<
        crate::output::UpdateSchedulingPolicyOutput,
        crate::error::UpdateSchedulingPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_scheduling_policy_error(response)
        } else {
            crate::operation_deser::parse_update_scheduling_policy_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
