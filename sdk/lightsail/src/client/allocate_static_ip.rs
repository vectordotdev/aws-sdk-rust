// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AllocateStaticIp`](crate::operation::allocate_static_ip::builders::AllocateStaticIpFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`static_ip_name(impl ::std::convert::Into<String>)`](crate::operation::allocate_static_ip::builders::AllocateStaticIpFluentBuilder::static_ip_name) / [`set_static_ip_name(Option<String>)`](crate::operation::allocate_static_ip::builders::AllocateStaticIpFluentBuilder::set_static_ip_name): <p>The name of the static IP address.</p>
    /// - On success, responds with [`AllocateStaticIpOutput`](crate::operation::allocate_static_ip::AllocateStaticIpOutput) with field(s):
    ///   - [`operations(Option<Vec<Operation>>)`](crate::operation::allocate_static_ip::AllocateStaticIpOutput::operations): <p>An array of objects that describe the result of the action, such as the status of the request, the timestamp of the request, and the resources affected by the request.</p>
    /// - On failure, responds with [`SdkError<AllocateStaticIpError>`](crate::operation::allocate_static_ip::AllocateStaticIpError)
    pub fn allocate_static_ip(
        &self,
    ) -> crate::operation::allocate_static_ip::builders::AllocateStaticIpFluentBuilder {
        crate::operation::allocate_static_ip::builders::AllocateStaticIpFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
