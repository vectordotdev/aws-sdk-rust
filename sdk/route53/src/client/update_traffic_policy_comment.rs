// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateTrafficPolicyComment`](crate::operation::update_traffic_policy_comment::builders::UpdateTrafficPolicyCommentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::update_traffic_policy_comment::builders::UpdateTrafficPolicyCommentFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::update_traffic_policy_comment::builders::UpdateTrafficPolicyCommentFluentBuilder::set_id): <p>The value of <code>Id</code> for the traffic policy that you want to update the comment for.</p>
    ///   - [`version(i32)`](crate::operation::update_traffic_policy_comment::builders::UpdateTrafficPolicyCommentFluentBuilder::version) / [`set_version(Option<i32>)`](crate::operation::update_traffic_policy_comment::builders::UpdateTrafficPolicyCommentFluentBuilder::set_version): <p>The value of <code>Version</code> for the traffic policy that you want to update the comment for.</p>
    ///   - [`comment(impl ::std::convert::Into<String>)`](crate::operation::update_traffic_policy_comment::builders::UpdateTrafficPolicyCommentFluentBuilder::comment) / [`set_comment(Option<String>)`](crate::operation::update_traffic_policy_comment::builders::UpdateTrafficPolicyCommentFluentBuilder::set_comment): <p>The new comment for the specified traffic policy and version.</p>
    /// - On success, responds with [`UpdateTrafficPolicyCommentOutput`](crate::operation::update_traffic_policy_comment::UpdateTrafficPolicyCommentOutput) with field(s):
    ///   - [`traffic_policy(Option<TrafficPolicy>)`](crate::operation::update_traffic_policy_comment::UpdateTrafficPolicyCommentOutput::traffic_policy): <p>A complex type that contains settings for the specified traffic policy.</p>
    /// - On failure, responds with [`SdkError<UpdateTrafficPolicyCommentError>`](crate::operation::update_traffic_policy_comment::UpdateTrafficPolicyCommentError)
    pub fn update_traffic_policy_comment(&self) -> crate::operation::update_traffic_policy_comment::builders::UpdateTrafficPolicyCommentFluentBuilder{
        crate::operation::update_traffic_policy_comment::builders::UpdateTrafficPolicyCommentFluentBuilder::new(self.handle.clone())
    }
}
