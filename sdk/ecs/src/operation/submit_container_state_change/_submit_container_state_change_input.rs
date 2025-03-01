// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SubmitContainerStateChangeInput {
    /// <p>The short name or full ARN of the cluster that hosts the container.</p>
    #[doc(hidden)]
    pub cluster: ::std::option::Option<::std::string::String>,
    /// <p>The task ID or full Amazon Resource Name (ARN) of the task that hosts the container.</p>
    #[doc(hidden)]
    pub task: ::std::option::Option<::std::string::String>,
    /// <p>The name of the container.</p>
    #[doc(hidden)]
    pub container_name: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Docker container.</p>
    #[doc(hidden)]
    pub runtime_id: ::std::option::Option<::std::string::String>,
    /// <p>The status of the state change request.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<::std::string::String>,
    /// <p>The exit code that's returned for the state change request.</p>
    #[doc(hidden)]
    pub exit_code: ::std::option::Option<i32>,
    /// <p>The reason for the state change request.</p>
    #[doc(hidden)]
    pub reason: ::std::option::Option<::std::string::String>,
    /// <p>The network bindings of the container.</p>
    #[doc(hidden)]
    pub network_bindings: ::std::option::Option<::std::vec::Vec<crate::types::NetworkBinding>>,
}
impl SubmitContainerStateChangeInput {
    /// <p>The short name or full ARN of the cluster that hosts the container.</p>
    pub fn cluster(&self) -> ::std::option::Option<&str> {
        self.cluster.as_deref()
    }
    /// <p>The task ID or full Amazon Resource Name (ARN) of the task that hosts the container.</p>
    pub fn task(&self) -> ::std::option::Option<&str> {
        self.task.as_deref()
    }
    /// <p>The name of the container.</p>
    pub fn container_name(&self) -> ::std::option::Option<&str> {
        self.container_name.as_deref()
    }
    /// <p>The ID of the Docker container.</p>
    pub fn runtime_id(&self) -> ::std::option::Option<&str> {
        self.runtime_id.as_deref()
    }
    /// <p>The status of the state change request.</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>The exit code that's returned for the state change request.</p>
    pub fn exit_code(&self) -> ::std::option::Option<i32> {
        self.exit_code
    }
    /// <p>The reason for the state change request.</p>
    pub fn reason(&self) -> ::std::option::Option<&str> {
        self.reason.as_deref()
    }
    /// <p>The network bindings of the container.</p>
    pub fn network_bindings(&self) -> ::std::option::Option<&[crate::types::NetworkBinding]> {
        self.network_bindings.as_deref()
    }
}
impl SubmitContainerStateChangeInput {
    /// Creates a new builder-style object to manufacture [`SubmitContainerStateChangeInput`](crate::operation::submit_container_state_change::SubmitContainerStateChangeInput).
    pub fn builder() -> crate::operation::submit_container_state_change::builders::SubmitContainerStateChangeInputBuilder{
        crate::operation::submit_container_state_change::builders::SubmitContainerStateChangeInputBuilder::default()
    }
}

/// A builder for [`SubmitContainerStateChangeInput`](crate::operation::submit_container_state_change::SubmitContainerStateChangeInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SubmitContainerStateChangeInputBuilder {
    pub(crate) cluster: ::std::option::Option<::std::string::String>,
    pub(crate) task: ::std::option::Option<::std::string::String>,
    pub(crate) container_name: ::std::option::Option<::std::string::String>,
    pub(crate) runtime_id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<::std::string::String>,
    pub(crate) exit_code: ::std::option::Option<i32>,
    pub(crate) reason: ::std::option::Option<::std::string::String>,
    pub(crate) network_bindings:
        ::std::option::Option<::std::vec::Vec<crate::types::NetworkBinding>>,
}
impl SubmitContainerStateChangeInputBuilder {
    /// <p>The short name or full ARN of the cluster that hosts the container.</p>
    pub fn cluster(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cluster = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The short name or full ARN of the cluster that hosts the container.</p>
    pub fn set_cluster(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cluster = input;
        self
    }
    /// <p>The task ID or full Amazon Resource Name (ARN) of the task that hosts the container.</p>
    pub fn task(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.task = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The task ID or full Amazon Resource Name (ARN) of the task that hosts the container.</p>
    pub fn set_task(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.task = input;
        self
    }
    /// <p>The name of the container.</p>
    pub fn container_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.container_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the container.</p>
    pub fn set_container_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.container_name = input;
        self
    }
    /// <p>The ID of the Docker container.</p>
    pub fn runtime_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.runtime_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Docker container.</p>
    pub fn set_runtime_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.runtime_id = input;
        self
    }
    /// <p>The status of the state change request.</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The status of the state change request.</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p>The exit code that's returned for the state change request.</p>
    pub fn exit_code(mut self, input: i32) -> Self {
        self.exit_code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The exit code that's returned for the state change request.</p>
    pub fn set_exit_code(mut self, input: ::std::option::Option<i32>) -> Self {
        self.exit_code = input;
        self
    }
    /// <p>The reason for the state change request.</p>
    pub fn reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The reason for the state change request.</p>
    pub fn set_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.reason = input;
        self
    }
    /// Appends an item to `network_bindings`.
    ///
    /// To override the contents of this collection use [`set_network_bindings`](Self::set_network_bindings).
    ///
    /// <p>The network bindings of the container.</p>
    pub fn network_bindings(mut self, input: crate::types::NetworkBinding) -> Self {
        let mut v = self.network_bindings.unwrap_or_default();
        v.push(input);
        self.network_bindings = ::std::option::Option::Some(v);
        self
    }
    /// <p>The network bindings of the container.</p>
    pub fn set_network_bindings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::NetworkBinding>>,
    ) -> Self {
        self.network_bindings = input;
        self
    }
    /// Consumes the builder and constructs a [`SubmitContainerStateChangeInput`](crate::operation::submit_container_state_change::SubmitContainerStateChangeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::submit_container_state_change::SubmitContainerStateChangeInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::submit_container_state_change::SubmitContainerStateChangeInput {
                cluster: self.cluster,
                task: self.task,
                container_name: self.container_name,
                runtime_id: self.runtime_id,
                status: self.status,
                exit_code: self.exit_code,
                reason: self.reason,
                network_bindings: self.network_bindings,
            },
        )
    }
}
