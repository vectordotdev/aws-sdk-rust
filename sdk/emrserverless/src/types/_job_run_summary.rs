// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The summary of attributes associated with a job run.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct JobRunSummary {
    /// <p>The ID of the application the job is running on.</p>
    #[doc(hidden)]
    pub application_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the job run.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The optional job run name. This doesn't have to be unique.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the job run.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The user who created the job run.</p>
    #[doc(hidden)]
    pub created_by: ::std::option::Option<::std::string::String>,
    /// <p>The date and time when the job run was created.</p>
    #[doc(hidden)]
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date and time when the job run was last updated.</p>
    #[doc(hidden)]
    pub updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The execution role ARN of the job run.</p>
    #[doc(hidden)]
    pub execution_role: ::std::option::Option<::std::string::String>,
    /// <p>The state of the job run.</p>
    #[doc(hidden)]
    pub state: ::std::option::Option<crate::types::JobRunState>,
    /// <p>The state details of the job run.</p>
    #[doc(hidden)]
    pub state_details: ::std::option::Option<::std::string::String>,
    /// <p>The EMR release associated with the application your job is running on.</p>
    #[doc(hidden)]
    pub release_label: ::std::option::Option<::std::string::String>,
    /// <p>The type of job run, such as Spark or Hive.</p>
    #[doc(hidden)]
    pub r#type: ::std::option::Option<::std::string::String>,
}
impl JobRunSummary {
    /// <p>The ID of the application the job is running on.</p>
    pub fn application_id(&self) -> ::std::option::Option<&str> {
        self.application_id.as_deref()
    }
    /// <p>The ID of the job run.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The optional job run name. This doesn't have to be unique.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The ARN of the job run.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The user who created the job run.</p>
    pub fn created_by(&self) -> ::std::option::Option<&str> {
        self.created_by.as_deref()
    }
    /// <p>The date and time when the job run was created.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>The date and time when the job run was last updated.</p>
    pub fn updated_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.updated_at.as_ref()
    }
    /// <p>The execution role ARN of the job run.</p>
    pub fn execution_role(&self) -> ::std::option::Option<&str> {
        self.execution_role.as_deref()
    }
    /// <p>The state of the job run.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::JobRunState> {
        self.state.as_ref()
    }
    /// <p>The state details of the job run.</p>
    pub fn state_details(&self) -> ::std::option::Option<&str> {
        self.state_details.as_deref()
    }
    /// <p>The EMR release associated with the application your job is running on.</p>
    pub fn release_label(&self) -> ::std::option::Option<&str> {
        self.release_label.as_deref()
    }
    /// <p>The type of job run, such as Spark or Hive.</p>
    pub fn r#type(&self) -> ::std::option::Option<&str> {
        self.r#type.as_deref()
    }
}
impl JobRunSummary {
    /// Creates a new builder-style object to manufacture [`JobRunSummary`](crate::types::JobRunSummary).
    pub fn builder() -> crate::types::builders::JobRunSummaryBuilder {
        crate::types::builders::JobRunSummaryBuilder::default()
    }
}

/// A builder for [`JobRunSummary`](crate::types::JobRunSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct JobRunSummaryBuilder {
    pub(crate) application_id: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) created_by: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) updated_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) execution_role: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::JobRunState>,
    pub(crate) state_details: ::std::option::Option<::std::string::String>,
    pub(crate) release_label: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<::std::string::String>,
}
impl JobRunSummaryBuilder {
    /// <p>The ID of the application the job is running on.</p>
    pub fn application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the application the job is running on.</p>
    pub fn set_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_id = input;
        self
    }
    /// <p>The ID of the job run.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the job run.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The optional job run name. This doesn't have to be unique.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The optional job run name. This doesn't have to be unique.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The ARN of the job run.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the job run.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The user who created the job run.</p>
    pub fn created_by(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.created_by = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user who created the job run.</p>
    pub fn set_created_by(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.created_by = input;
        self
    }
    /// <p>The date and time when the job run was created.</p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when the job run was created.</p>
    pub fn set_created_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_at = input;
        self
    }
    /// <p>The date and time when the job run was last updated.</p>
    pub fn updated_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.updated_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when the job run was last updated.</p>
    pub fn set_updated_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.updated_at = input;
        self
    }
    /// <p>The execution role ARN of the job run.</p>
    pub fn execution_role(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.execution_role = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The execution role ARN of the job run.</p>
    pub fn set_execution_role(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.execution_role = input;
        self
    }
    /// <p>The state of the job run.</p>
    pub fn state(mut self, input: crate::types::JobRunState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the job run.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::JobRunState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The state details of the job run.</p>
    pub fn state_details(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.state_details = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The state details of the job run.</p>
    pub fn set_state_details(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.state_details = input;
        self
    }
    /// <p>The EMR release associated with the application your job is running on.</p>
    pub fn release_label(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.release_label = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The EMR release associated with the application your job is running on.</p>
    pub fn set_release_label(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.release_label = input;
        self
    }
    /// <p>The type of job run, such as Spark or Hive.</p>
    pub fn r#type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.r#type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of job run, such as Spark or Hive.</p>
    pub fn set_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.r#type = input;
        self
    }
    /// Consumes the builder and constructs a [`JobRunSummary`](crate::types::JobRunSummary).
    pub fn build(self) -> crate::types::JobRunSummary {
        crate::types::JobRunSummary {
            application_id: self.application_id,
            id: self.id,
            name: self.name,
            arn: self.arn,
            created_by: self.created_by,
            created_at: self.created_at,
            updated_at: self.updated_at,
            execution_role: self.execution_role,
            state: self.state,
            state_details: self.state_details,
            release_label: self.release_label,
            r#type: self.r#type,
        }
    }
}
