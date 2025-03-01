// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateQueueNameInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    #[doc(hidden)]
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier for the queue.</p>
    #[doc(hidden)]
    pub queue_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the queue.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The description of the queue.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
}
impl UpdateQueueNameInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The identifier for the queue.</p>
    pub fn queue_id(&self) -> ::std::option::Option<&str> {
        self.queue_id.as_deref()
    }
    /// <p>The name of the queue.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The description of the queue.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
}
impl UpdateQueueNameInput {
    /// Creates a new builder-style object to manufacture [`UpdateQueueNameInput`](crate::operation::update_queue_name::UpdateQueueNameInput).
    pub fn builder() -> crate::operation::update_queue_name::builders::UpdateQueueNameInputBuilder {
        crate::operation::update_queue_name::builders::UpdateQueueNameInputBuilder::default()
    }
}

/// A builder for [`UpdateQueueNameInput`](crate::operation::update_queue_name::UpdateQueueNameInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateQueueNameInputBuilder {
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) queue_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
}
impl UpdateQueueNameInputBuilder {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The identifier for the queue.</p>
    pub fn queue_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.queue_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the queue.</p>
    pub fn set_queue_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.queue_id = input;
        self
    }
    /// <p>The name of the queue.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the queue.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The description of the queue.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the queue.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateQueueNameInput`](crate::operation::update_queue_name::UpdateQueueNameInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_queue_name::UpdateQueueNameInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_queue_name::UpdateQueueNameInput {
            instance_id: self.instance_id,
            queue_id: self.queue_id,
            name: self.name,
            description: self.description,
        })
    }
}
