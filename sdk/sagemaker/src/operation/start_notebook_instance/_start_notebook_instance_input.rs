// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartNotebookInstanceInput {
    /// <p>The name of the notebook instance to start.</p>
    #[doc(hidden)]
    pub notebook_instance_name: ::std::option::Option<::std::string::String>,
}
impl StartNotebookInstanceInput {
    /// <p>The name of the notebook instance to start.</p>
    pub fn notebook_instance_name(&self) -> ::std::option::Option<&str> {
        self.notebook_instance_name.as_deref()
    }
}
impl StartNotebookInstanceInput {
    /// Creates a new builder-style object to manufacture [`StartNotebookInstanceInput`](crate::operation::start_notebook_instance::StartNotebookInstanceInput).
    pub fn builder(
    ) -> crate::operation::start_notebook_instance::builders::StartNotebookInstanceInputBuilder
    {
        crate::operation::start_notebook_instance::builders::StartNotebookInstanceInputBuilder::default()
    }
}

/// A builder for [`StartNotebookInstanceInput`](crate::operation::start_notebook_instance::StartNotebookInstanceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartNotebookInstanceInputBuilder {
    pub(crate) notebook_instance_name: ::std::option::Option<::std::string::String>,
}
impl StartNotebookInstanceInputBuilder {
    /// <p>The name of the notebook instance to start.</p>
    pub fn notebook_instance_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.notebook_instance_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the notebook instance to start.</p>
    pub fn set_notebook_instance_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.notebook_instance_name = input;
        self
    }
    /// Consumes the builder and constructs a [`StartNotebookInstanceInput`](crate::operation::start_notebook_instance::StartNotebookInstanceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_notebook_instance::StartNotebookInstanceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::start_notebook_instance::StartNotebookInstanceInput {
                notebook_instance_name: self.notebook_instance_name,
            },
        )
    }
}
