// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetTrafficPolicy`](crate::operation::get_traffic_policy::builders::GetTrafficPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::get_traffic_policy::builders::GetTrafficPolicyFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::get_traffic_policy::builders::GetTrafficPolicyFluentBuilder::set_id): <p>The ID of the traffic policy that you want to get information about.</p>
    ///   - [`version(i32)`](crate::operation::get_traffic_policy::builders::GetTrafficPolicyFluentBuilder::version) / [`set_version(Option<i32>)`](crate::operation::get_traffic_policy::builders::GetTrafficPolicyFluentBuilder::set_version): <p>The version number of the traffic policy that you want to get information about.</p>
    /// - On success, responds with [`GetTrafficPolicyOutput`](crate::operation::get_traffic_policy::GetTrafficPolicyOutput) with field(s):
    ///   - [`traffic_policy(Option<TrafficPolicy>)`](crate::operation::get_traffic_policy::GetTrafficPolicyOutput::traffic_policy): <p>A complex type that contains settings for the specified traffic policy.</p>
    /// - On failure, responds with [`SdkError<GetTrafficPolicyError>`](crate::operation::get_traffic_policy::GetTrafficPolicyError)
    pub fn get_traffic_policy(
        &self,
    ) -> crate::operation::get_traffic_policy::builders::GetTrafficPolicyFluentBuilder {
        crate::operation::get_traffic_policy::builders::GetTrafficPolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
