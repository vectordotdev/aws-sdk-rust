// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAuditSuppression`](crate::operation::describe_audit_suppression::builders::DescribeAuditSuppressionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`check_name(impl ::std::convert::Into<String>)`](crate::operation::describe_audit_suppression::builders::DescribeAuditSuppressionFluentBuilder::check_name) / [`set_check_name(Option<String>)`](crate::operation::describe_audit_suppression::builders::DescribeAuditSuppressionFluentBuilder::set_check_name): <p>An audit check name. Checks must be enabled for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> to select which checks are enabled.)</p>
    ///   - [`resource_identifier(ResourceIdentifier)`](crate::operation::describe_audit_suppression::builders::DescribeAuditSuppressionFluentBuilder::resource_identifier) / [`set_resource_identifier(Option<ResourceIdentifier>)`](crate::operation::describe_audit_suppression::builders::DescribeAuditSuppressionFluentBuilder::set_resource_identifier): <p>Information that identifies the noncompliant resource.</p>
    /// - On success, responds with [`DescribeAuditSuppressionOutput`](crate::operation::describe_audit_suppression::DescribeAuditSuppressionOutput) with field(s):
    ///   - [`check_name(Option<String>)`](crate::operation::describe_audit_suppression::DescribeAuditSuppressionOutput::check_name): <p>An audit check name. Checks must be enabled for your account. (Use <code>DescribeAccountAuditConfiguration</code> to see the list of all checks, including those that are enabled or use <code>UpdateAccountAuditConfiguration</code> to select which checks are enabled.)</p>
    ///   - [`resource_identifier(Option<ResourceIdentifier>)`](crate::operation::describe_audit_suppression::DescribeAuditSuppressionOutput::resource_identifier): <p>Information that identifies the noncompliant resource.</p>
    ///   - [`expiration_date(Option<DateTime>)`](crate::operation::describe_audit_suppression::DescribeAuditSuppressionOutput::expiration_date): <p> The epoch timestamp in seconds at which this suppression expires. </p>
    ///   - [`suppress_indefinitely(Option<bool>)`](crate::operation::describe_audit_suppression::DescribeAuditSuppressionOutput::suppress_indefinitely): <p> Indicates whether a suppression should exist indefinitely or not. </p>
    ///   - [`description(Option<String>)`](crate::operation::describe_audit_suppression::DescribeAuditSuppressionOutput::description): <p> The description of the audit suppression. </p>
    /// - On failure, responds with [`SdkError<DescribeAuditSuppressionError>`](crate::operation::describe_audit_suppression::DescribeAuditSuppressionError)
    pub fn describe_audit_suppression(
        &self,
    ) -> crate::operation::describe_audit_suppression::builders::DescribeAuditSuppressionFluentBuilder
    {
        crate::operation::describe_audit_suppression::builders::DescribeAuditSuppressionFluentBuilder::new(self.handle.clone())
    }
}
