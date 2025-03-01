// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BundleInstance`](crate::operation::bundle_instance::builders::BundleInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::bundle_instance::builders::BundleInstanceFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::bundle_instance::builders::BundleInstanceFluentBuilder::set_instance_id): <p>The ID of the instance to bundle.</p>  <p>Type: String</p>  <p>Default: None</p>  <p>Required: Yes</p>
    ///   - [`storage(Storage)`](crate::operation::bundle_instance::builders::BundleInstanceFluentBuilder::storage) / [`set_storage(Option<Storage>)`](crate::operation::bundle_instance::builders::BundleInstanceFluentBuilder::set_storage): <p>The bucket in which to store the AMI. You can specify a bucket that you already own or a new bucket that Amazon EC2 creates on your behalf. If you specify a bucket that belongs to someone else, Amazon EC2 returns an error.</p>
    ///   - [`dry_run(bool)`](crate::operation::bundle_instance::builders::BundleInstanceFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::bundle_instance::builders::BundleInstanceFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`BundleInstanceOutput`](crate::operation::bundle_instance::BundleInstanceOutput) with field(s):
    ///   - [`bundle_task(Option<BundleTask>)`](crate::operation::bundle_instance::BundleInstanceOutput::bundle_task): <p>Information about the bundle task.</p>
    /// - On failure, responds with [`SdkError<BundleInstanceError>`](crate::operation::bundle_instance::BundleInstanceError)
    pub fn bundle_instance(
        &self,
    ) -> crate::operation::bundle_instance::builders::BundleInstanceFluentBuilder {
        crate::operation::bundle_instance::builders::BundleInstanceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
