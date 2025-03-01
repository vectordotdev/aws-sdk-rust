// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetBuiltinIntentInput {
    /// <p>The unique identifier for a built-in intent. To find the signature for an intent, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents">Standard Built-in Intents</a> in the <i>Alexa Skills Kit</i>.</p>
    #[doc(hidden)]
    pub signature: ::std::option::Option<::std::string::String>,
}
impl GetBuiltinIntentInput {
    /// <p>The unique identifier for a built-in intent. To find the signature for an intent, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents">Standard Built-in Intents</a> in the <i>Alexa Skills Kit</i>.</p>
    pub fn signature(&self) -> ::std::option::Option<&str> {
        self.signature.as_deref()
    }
}
impl GetBuiltinIntentInput {
    /// Creates a new builder-style object to manufacture [`GetBuiltinIntentInput`](crate::operation::get_builtin_intent::GetBuiltinIntentInput).
    pub fn builder() -> crate::operation::get_builtin_intent::builders::GetBuiltinIntentInputBuilder
    {
        crate::operation::get_builtin_intent::builders::GetBuiltinIntentInputBuilder::default()
    }
}

/// A builder for [`GetBuiltinIntentInput`](crate::operation::get_builtin_intent::GetBuiltinIntentInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetBuiltinIntentInputBuilder {
    pub(crate) signature: ::std::option::Option<::std::string::String>,
}
impl GetBuiltinIntentInputBuilder {
    /// <p>The unique identifier for a built-in intent. To find the signature for an intent, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents">Standard Built-in Intents</a> in the <i>Alexa Skills Kit</i>.</p>
    pub fn signature(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.signature = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for a built-in intent. To find the signature for an intent, see <a href="https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/built-in-intent-ref/standard-intents">Standard Built-in Intents</a> in the <i>Alexa Skills Kit</i>.</p>
    pub fn set_signature(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.signature = input;
        self
    }
    /// Consumes the builder and constructs a [`GetBuiltinIntentInput`](crate::operation::get_builtin_intent::GetBuiltinIntentInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_builtin_intent::GetBuiltinIntentInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_builtin_intent::GetBuiltinIntentInput {
                signature: self.signature,
            },
        )
    }
}
