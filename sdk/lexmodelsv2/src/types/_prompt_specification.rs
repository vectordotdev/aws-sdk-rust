// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies a list of message groups that Amazon Lex sends to a user to elicit a response.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PromptSpecification {
    /// <p>A collection of messages that Amazon Lex can send to the user. Amazon Lex chooses the actual message to send at runtime.</p>
    #[doc(hidden)]
    pub message_groups: ::std::option::Option<::std::vec::Vec<crate::types::MessageGroup>>,
    /// <p>The maximum number of times the bot tries to elicit a response from the user using this prompt.</p>
    #[doc(hidden)]
    pub max_retries: ::std::option::Option<i32>,
    /// <p>Indicates whether the user can interrupt a speech prompt from the bot.</p>
    #[doc(hidden)]
    pub allow_interrupt: ::std::option::Option<bool>,
    /// <p>Indicates how a message is selected from a message group among retries.</p>
    #[doc(hidden)]
    pub message_selection_strategy: ::std::option::Option<crate::types::MessageSelectionStrategy>,
    /// <p>Specifies the advanced settings on each attempt of the prompt.</p>
    #[doc(hidden)]
    pub prompt_attempts_specification: ::std::option::Option<
        ::std::collections::HashMap<
            crate::types::PromptAttempt,
            crate::types::PromptAttemptSpecification,
        >,
    >,
}
impl PromptSpecification {
    /// <p>A collection of messages that Amazon Lex can send to the user. Amazon Lex chooses the actual message to send at runtime.</p>
    pub fn message_groups(&self) -> ::std::option::Option<&[crate::types::MessageGroup]> {
        self.message_groups.as_deref()
    }
    /// <p>The maximum number of times the bot tries to elicit a response from the user using this prompt.</p>
    pub fn max_retries(&self) -> ::std::option::Option<i32> {
        self.max_retries
    }
    /// <p>Indicates whether the user can interrupt a speech prompt from the bot.</p>
    pub fn allow_interrupt(&self) -> ::std::option::Option<bool> {
        self.allow_interrupt
    }
    /// <p>Indicates how a message is selected from a message group among retries.</p>
    pub fn message_selection_strategy(
        &self,
    ) -> ::std::option::Option<&crate::types::MessageSelectionStrategy> {
        self.message_selection_strategy.as_ref()
    }
    /// <p>Specifies the advanced settings on each attempt of the prompt.</p>
    pub fn prompt_attempts_specification(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<
            crate::types::PromptAttempt,
            crate::types::PromptAttemptSpecification,
        >,
    > {
        self.prompt_attempts_specification.as_ref()
    }
}
impl PromptSpecification {
    /// Creates a new builder-style object to manufacture [`PromptSpecification`](crate::types::PromptSpecification).
    pub fn builder() -> crate::types::builders::PromptSpecificationBuilder {
        crate::types::builders::PromptSpecificationBuilder::default()
    }
}

/// A builder for [`PromptSpecification`](crate::types::PromptSpecification).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PromptSpecificationBuilder {
    pub(crate) message_groups: ::std::option::Option<::std::vec::Vec<crate::types::MessageGroup>>,
    pub(crate) max_retries: ::std::option::Option<i32>,
    pub(crate) allow_interrupt: ::std::option::Option<bool>,
    pub(crate) message_selection_strategy:
        ::std::option::Option<crate::types::MessageSelectionStrategy>,
    pub(crate) prompt_attempts_specification: ::std::option::Option<
        ::std::collections::HashMap<
            crate::types::PromptAttempt,
            crate::types::PromptAttemptSpecification,
        >,
    >,
}
impl PromptSpecificationBuilder {
    /// Appends an item to `message_groups`.
    ///
    /// To override the contents of this collection use [`set_message_groups`](Self::set_message_groups).
    ///
    /// <p>A collection of messages that Amazon Lex can send to the user. Amazon Lex chooses the actual message to send at runtime.</p>
    pub fn message_groups(mut self, input: crate::types::MessageGroup) -> Self {
        let mut v = self.message_groups.unwrap_or_default();
        v.push(input);
        self.message_groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>A collection of messages that Amazon Lex can send to the user. Amazon Lex chooses the actual message to send at runtime.</p>
    pub fn set_message_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::MessageGroup>>,
    ) -> Self {
        self.message_groups = input;
        self
    }
    /// <p>The maximum number of times the bot tries to elicit a response from the user using this prompt.</p>
    pub fn max_retries(mut self, input: i32) -> Self {
        self.max_retries = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of times the bot tries to elicit a response from the user using this prompt.</p>
    pub fn set_max_retries(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_retries = input;
        self
    }
    /// <p>Indicates whether the user can interrupt a speech prompt from the bot.</p>
    pub fn allow_interrupt(mut self, input: bool) -> Self {
        self.allow_interrupt = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the user can interrupt a speech prompt from the bot.</p>
    pub fn set_allow_interrupt(mut self, input: ::std::option::Option<bool>) -> Self {
        self.allow_interrupt = input;
        self
    }
    /// <p>Indicates how a message is selected from a message group among retries.</p>
    pub fn message_selection_strategy(
        mut self,
        input: crate::types::MessageSelectionStrategy,
    ) -> Self {
        self.message_selection_strategy = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates how a message is selected from a message group among retries.</p>
    pub fn set_message_selection_strategy(
        mut self,
        input: ::std::option::Option<crate::types::MessageSelectionStrategy>,
    ) -> Self {
        self.message_selection_strategy = input;
        self
    }
    /// Adds a key-value pair to `prompt_attempts_specification`.
    ///
    /// To override the contents of this collection use [`set_prompt_attempts_specification`](Self::set_prompt_attempts_specification).
    ///
    /// <p>Specifies the advanced settings on each attempt of the prompt.</p>
    pub fn prompt_attempts_specification(
        mut self,
        k: crate::types::PromptAttempt,
        v: crate::types::PromptAttemptSpecification,
    ) -> Self {
        let mut hash_map = self.prompt_attempts_specification.unwrap_or_default();
        hash_map.insert(k, v);
        self.prompt_attempts_specification = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Specifies the advanced settings on each attempt of the prompt.</p>
    pub fn set_prompt_attempts_specification(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<
                crate::types::PromptAttempt,
                crate::types::PromptAttemptSpecification,
            >,
        >,
    ) -> Self {
        self.prompt_attempts_specification = input;
        self
    }
    /// Consumes the builder and constructs a [`PromptSpecification`](crate::types::PromptSpecification).
    pub fn build(self) -> crate::types::PromptSpecification {
        crate::types::PromptSpecification {
            message_groups: self.message_groups,
            max_retries: self.max_retries,
            allow_interrupt: self.allow_interrupt,
            message_selection_strategy: self.message_selection_strategy,
            prompt_attempts_specification: self.prompt_attempts_specification,
        }
    }
}
