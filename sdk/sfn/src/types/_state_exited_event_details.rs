// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains details about an exit from a state during an execution.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct StateExitedEventDetails {
    /// <p>The name of the state.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li> <p>white space</p> </li>
    /// <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li>
    /// <li> <p>wildcard characters <code>? *</code> </p> </li>
    /// <li> <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li>
    /// <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The JSON output data of the state. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.</p>
    #[doc(hidden)]
    pub output: ::std::option::Option<::std::string::String>,
    /// <p>Contains details about the output of an execution history event.</p>
    #[doc(hidden)]
    pub output_details: ::std::option::Option<crate::types::HistoryEventExecutionDataDetails>,
}
impl StateExitedEventDetails {
    /// <p>The name of the state.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li> <p>white space</p> </li>
    /// <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li>
    /// <li> <p>wildcard characters <code>? *</code> </p> </li>
    /// <li> <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li>
    /// <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The JSON output data of the state. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.</p>
    pub fn output(&self) -> ::std::option::Option<&str> {
        self.output.as_deref()
    }
    /// <p>Contains details about the output of an execution history event.</p>
    pub fn output_details(
        &self,
    ) -> ::std::option::Option<&crate::types::HistoryEventExecutionDataDetails> {
        self.output_details.as_ref()
    }
}
impl ::std::fmt::Debug for StateExitedEventDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("StateExitedEventDetails");
        formatter.field("name", &self.name);
        formatter.field("output", &"*** Sensitive Data Redacted ***");
        formatter.field("output_details", &self.output_details);
        formatter.finish()
    }
}
impl StateExitedEventDetails {
    /// Creates a new builder-style object to manufacture [`StateExitedEventDetails`](crate::types::StateExitedEventDetails).
    pub fn builder() -> crate::types::builders::StateExitedEventDetailsBuilder {
        crate::types::builders::StateExitedEventDetailsBuilder::default()
    }
}

/// A builder for [`StateExitedEventDetails`](crate::types::StateExitedEventDetails).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct StateExitedEventDetailsBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) output: ::std::option::Option<::std::string::String>,
    pub(crate) output_details:
        ::std::option::Option<crate::types::HistoryEventExecutionDataDetails>,
}
impl StateExitedEventDetailsBuilder {
    /// <p>The name of the state.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li> <p>white space</p> </li>
    /// <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li>
    /// <li> <p>wildcard characters <code>? *</code> </p> </li>
    /// <li> <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li>
    /// <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the state.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li> <p>white space</p> </li>
    /// <li> <p>brackets <code>&lt; &gt; { } [ ]</code> </p> </li>
    /// <li> <p>wildcard characters <code>? *</code> </p> </li>
    /// <li> <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code> </p> </li>
    /// <li> <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>)</p> </li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The JSON output data of the state. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.</p>
    pub fn output(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.output = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The JSON output data of the state. Length constraints apply to the payload size, and are expressed as bytes in UTF-8 encoding.</p>
    pub fn set_output(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.output = input;
        self
    }
    /// <p>Contains details about the output of an execution history event.</p>
    pub fn output_details(mut self, input: crate::types::HistoryEventExecutionDataDetails) -> Self {
        self.output_details = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains details about the output of an execution history event.</p>
    pub fn set_output_details(
        mut self,
        input: ::std::option::Option<crate::types::HistoryEventExecutionDataDetails>,
    ) -> Self {
        self.output_details = input;
        self
    }
    /// Consumes the builder and constructs a [`StateExitedEventDetails`](crate::types::StateExitedEventDetails).
    pub fn build(self) -> crate::types::StateExitedEventDetails {
        crate::types::StateExitedEventDetails {
            name: self.name,
            output: self.output,
            output_details: self.output_details,
        }
    }
}
impl ::std::fmt::Debug for StateExitedEventDetailsBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("StateExitedEventDetailsBuilder");
        formatter.field("name", &self.name);
        formatter.field("output", &"*** Sensitive Data Redacted ***");
        formatter.field("output_details", &self.output_details);
        formatter.finish()
    }
}
