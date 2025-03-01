// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateReadinessCheck`](crate::operation::update_readiness_check::builders::UpdateReadinessCheckFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`readiness_check_name(impl ::std::convert::Into<String>)`](crate::operation::update_readiness_check::builders::UpdateReadinessCheckFluentBuilder::readiness_check_name) / [`set_readiness_check_name(Option<String>)`](crate::operation::update_readiness_check::builders::UpdateReadinessCheckFluentBuilder::set_readiness_check_name): <p>Name of a readiness check.</p>
    ///   - [`resource_set_name(impl ::std::convert::Into<String>)`](crate::operation::update_readiness_check::builders::UpdateReadinessCheckFluentBuilder::resource_set_name) / [`set_resource_set_name(Option<String>)`](crate::operation::update_readiness_check::builders::UpdateReadinessCheckFluentBuilder::set_resource_set_name): <p>The name of the resource set to be checked.</p>
    /// - On success, responds with [`UpdateReadinessCheckOutput`](crate::operation::update_readiness_check::UpdateReadinessCheckOutput) with field(s):
    ///   - [`readiness_check_arn(Option<String>)`](crate::operation::update_readiness_check::UpdateReadinessCheckOutput::readiness_check_arn): <p>The Amazon Resource Name (ARN) associated with a readiness check.</p>
    ///   - [`readiness_check_name(Option<String>)`](crate::operation::update_readiness_check::UpdateReadinessCheckOutput::readiness_check_name): <p>Name of a readiness check.</p>
    ///   - [`resource_set(Option<String>)`](crate::operation::update_readiness_check::UpdateReadinessCheckOutput::resource_set): <p>Name of the resource set to be checked.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::update_readiness_check::UpdateReadinessCheckOutput::tags): <p>A collection of tags associated with a resource.</p>
    /// - On failure, responds with [`SdkError<UpdateReadinessCheckError>`](crate::operation::update_readiness_check::UpdateReadinessCheckError)
    pub fn update_readiness_check(
        &self,
    ) -> crate::operation::update_readiness_check::builders::UpdateReadinessCheckFluentBuilder {
        crate::operation::update_readiness_check::builders::UpdateReadinessCheckFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
