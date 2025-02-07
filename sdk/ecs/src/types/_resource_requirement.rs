// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The type and amount of a resource to assign to a container. The supported resource types are GPUs and Elastic Inference accelerators. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-gpu.html">Working with GPUs on Amazon ECS</a> or <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-inference.html">Working with Amazon Elastic Inference on Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i> </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResourceRequirement {
    /// <p>The value for the specified resource type.</p>
    /// <p>If the <code>GPU</code> type is used, the value is the number of physical <code>GPUs</code> the Amazon ECS container agent reserves for the container. The number of GPUs that's reserved for all containers in a task can't exceed the number of available GPUs on the container instance that the task is launched on.</p>
    /// <p>If the <code>InferenceAccelerator</code> type is used, the <code>value</code> matches the <code>deviceName</code> for an <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_InferenceAccelerator.html">InferenceAccelerator</a> specified in a task definition.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<::std::string::String>,
    /// <p>The type of resource to assign to a container. The supported values are <code>GPU</code> or <code>InferenceAccelerator</code>.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<crate::types::ResourceType>,
}
impl ResourceRequirement {
    /// <p>The value for the specified resource type.</p>
    /// <p>If the <code>GPU</code> type is used, the value is the number of physical <code>GPUs</code> the Amazon ECS container agent reserves for the container. The number of GPUs that's reserved for all containers in a task can't exceed the number of available GPUs on the container instance that the task is launched on.</p>
    /// <p>If the <code>InferenceAccelerator</code> type is used, the <code>value</code> matches the <code>deviceName</code> for an <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_InferenceAccelerator.html">InferenceAccelerator</a> specified in a task definition.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
    /// <p>The type of resource to assign to a container. The supported values are <code>GPU</code> or <code>InferenceAccelerator</code>.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::ResourceType> {
        self.r#type.as_ref()
    }
}
impl ResourceRequirement {
    /// Creates a new builder-style object to manufacture [`ResourceRequirement`](crate::types::ResourceRequirement).
    pub fn builder() -> crate::types::builders::ResourceRequirementBuilder {
        crate::types::builders::ResourceRequirementBuilder::default()
    }
}

/// A builder for [`ResourceRequirement`](crate::types::ResourceRequirement).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ResourceRequirementBuilder {
    pub(crate) value: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::ResourceType>,
}
impl ResourceRequirementBuilder {
    /// <p>The value for the specified resource type.</p>
    /// <p>If the <code>GPU</code> type is used, the value is the number of physical <code>GPUs</code> the Amazon ECS container agent reserves for the container. The number of GPUs that's reserved for all containers in a task can't exceed the number of available GPUs on the container instance that the task is launched on.</p>
    /// <p>If the <code>InferenceAccelerator</code> type is used, the <code>value</code> matches the <code>deviceName</code> for an <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_InferenceAccelerator.html">InferenceAccelerator</a> specified in a task definition.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value for the specified resource type.</p>
    /// <p>If the <code>GPU</code> type is used, the value is the number of physical <code>GPUs</code> the Amazon ECS container agent reserves for the container. The number of GPUs that's reserved for all containers in a task can't exceed the number of available GPUs on the container instance that the task is launched on.</p>
    /// <p>If the <code>InferenceAccelerator</code> type is used, the <code>value</code> matches the <code>deviceName</code> for an <a href="https://docs.aws.amazon.com/AmazonECS/latest/APIReference/API_InferenceAccelerator.html">InferenceAccelerator</a> specified in a task definition.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// <p>The type of resource to assign to a container. The supported values are <code>GPU</code> or <code>InferenceAccelerator</code>.</p>
    pub fn r#type(mut self, input: crate::types::ResourceType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of resource to assign to a container. The supported values are <code>GPU</code> or <code>InferenceAccelerator</code>.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::ResourceType>) -> Self {
        self.r#type = input;
        self
    }
    /// Consumes the builder and constructs a [`ResourceRequirement`](crate::types::ResourceRequirement).
    pub fn build(self) -> crate::types::ResourceRequirement {
        crate::types::ResourceRequirement {
            value: self.value,
            r#type: self.r#type,
        }
    }
}
