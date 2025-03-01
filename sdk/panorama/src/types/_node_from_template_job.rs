// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A job to create a camera stream node.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NodeFromTemplateJob {
    /// <p>The job's ID.</p>
    #[doc(hidden)]
    pub job_id: ::std::option::Option<::std::string::String>,
    /// <p>The job's template type.</p>
    #[doc(hidden)]
    pub template_type: ::std::option::Option<crate::types::TemplateType>,
    /// <p>The job's status.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::NodeFromTemplateJobStatus>,
    /// <p>The job's status message.</p>
    #[doc(hidden)]
    pub status_message: ::std::option::Option<::std::string::String>,
    /// <p>When the job was created.</p>
    #[doc(hidden)]
    pub created_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The node's name.</p>
    #[doc(hidden)]
    pub node_name: ::std::option::Option<::std::string::String>,
}
impl NodeFromTemplateJob {
    /// <p>The job's ID.</p>
    pub fn job_id(&self) -> ::std::option::Option<&str> {
        self.job_id.as_deref()
    }
    /// <p>The job's template type.</p>
    pub fn template_type(&self) -> ::std::option::Option<&crate::types::TemplateType> {
        self.template_type.as_ref()
    }
    /// <p>The job's status.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::NodeFromTemplateJobStatus> {
        self.status.as_ref()
    }
    /// <p>The job's status message.</p>
    pub fn status_message(&self) -> ::std::option::Option<&str> {
        self.status_message.as_deref()
    }
    /// <p>When the job was created.</p>
    pub fn created_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_time.as_ref()
    }
    /// <p>The node's name.</p>
    pub fn node_name(&self) -> ::std::option::Option<&str> {
        self.node_name.as_deref()
    }
}
impl NodeFromTemplateJob {
    /// Creates a new builder-style object to manufacture [`NodeFromTemplateJob`](crate::types::NodeFromTemplateJob).
    pub fn builder() -> crate::types::builders::NodeFromTemplateJobBuilder {
        crate::types::builders::NodeFromTemplateJobBuilder::default()
    }
}

/// A builder for [`NodeFromTemplateJob`](crate::types::NodeFromTemplateJob).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct NodeFromTemplateJobBuilder {
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
    pub(crate) template_type: ::std::option::Option<crate::types::TemplateType>,
    pub(crate) status: ::std::option::Option<crate::types::NodeFromTemplateJobStatus>,
    pub(crate) status_message: ::std::option::Option<::std::string::String>,
    pub(crate) created_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) node_name: ::std::option::Option<::std::string::String>,
}
impl NodeFromTemplateJobBuilder {
    /// <p>The job's ID.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The job's ID.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// <p>The job's template type.</p>
    pub fn template_type(mut self, input: crate::types::TemplateType) -> Self {
        self.template_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The job's template type.</p>
    pub fn set_template_type(
        mut self,
        input: ::std::option::Option<crate::types::TemplateType>,
    ) -> Self {
        self.template_type = input;
        self
    }
    /// <p>The job's status.</p>
    pub fn status(mut self, input: crate::types::NodeFromTemplateJobStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The job's status.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::NodeFromTemplateJobStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>The job's status message.</p>
    pub fn status_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.status_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The job's status message.</p>
    pub fn set_status_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.status_message = input;
        self
    }
    /// <p>When the job was created.</p>
    pub fn created_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>When the job was created.</p>
    pub fn set_created_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_time = input;
        self
    }
    /// <p>The node's name.</p>
    pub fn node_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.node_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The node's name.</p>
    pub fn set_node_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.node_name = input;
        self
    }
    /// Consumes the builder and constructs a [`NodeFromTemplateJob`](crate::types::NodeFromTemplateJob).
    pub fn build(self) -> crate::types::NodeFromTemplateJob {
        crate::types::NodeFromTemplateJob {
            job_id: self.job_id,
            template_type: self.template_type,
            status: self.status,
            status_message: self.status_message,
            created_time: self.created_time,
            node_name: self.node_name,
        }
    }
}
