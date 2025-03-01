// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Properties that specify the code generator for a generated code job.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Generator {
    /// <p>The platform that will be used to run the generated code.</p>
    #[doc(hidden)]
    pub target_platform: ::std::option::Option<::std::string::String>,
    /// <p>The programming language for the generated code.</p>
    /// <p> Not all languages are supported for each platform. For cases where multiple languages are supported, this parameter specifies the language to be used. If this value is omitted, the default language for the target platform will be used. </p>
    #[doc(hidden)]
    pub language: ::std::option::Option<::std::string::String>,
    /// <p>The target version of the GameSparks Game SDK.</p>
    #[doc(hidden)]
    pub game_sdk_version: ::std::option::Option<::std::string::String>,
}
impl Generator {
    /// <p>The platform that will be used to run the generated code.</p>
    pub fn target_platform(&self) -> ::std::option::Option<&str> {
        self.target_platform.as_deref()
    }
    /// <p>The programming language for the generated code.</p>
    /// <p> Not all languages are supported for each platform. For cases where multiple languages are supported, this parameter specifies the language to be used. If this value is omitted, the default language for the target platform will be used. </p>
    pub fn language(&self) -> ::std::option::Option<&str> {
        self.language.as_deref()
    }
    /// <p>The target version of the GameSparks Game SDK.</p>
    pub fn game_sdk_version(&self) -> ::std::option::Option<&str> {
        self.game_sdk_version.as_deref()
    }
}
impl Generator {
    /// Creates a new builder-style object to manufacture [`Generator`](crate::types::Generator).
    pub fn builder() -> crate::types::builders::GeneratorBuilder {
        crate::types::builders::GeneratorBuilder::default()
    }
}

/// A builder for [`Generator`](crate::types::Generator).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GeneratorBuilder {
    pub(crate) target_platform: ::std::option::Option<::std::string::String>,
    pub(crate) language: ::std::option::Option<::std::string::String>,
    pub(crate) game_sdk_version: ::std::option::Option<::std::string::String>,
}
impl GeneratorBuilder {
    /// <p>The platform that will be used to run the generated code.</p>
    pub fn target_platform(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.target_platform = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The platform that will be used to run the generated code.</p>
    pub fn set_target_platform(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.target_platform = input;
        self
    }
    /// <p>The programming language for the generated code.</p>
    /// <p> Not all languages are supported for each platform. For cases where multiple languages are supported, this parameter specifies the language to be used. If this value is omitted, the default language for the target platform will be used. </p>
    pub fn language(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.language = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The programming language for the generated code.</p>
    /// <p> Not all languages are supported for each platform. For cases where multiple languages are supported, this parameter specifies the language to be used. If this value is omitted, the default language for the target platform will be used. </p>
    pub fn set_language(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.language = input;
        self
    }
    /// <p>The target version of the GameSparks Game SDK.</p>
    pub fn game_sdk_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.game_sdk_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The target version of the GameSparks Game SDK.</p>
    pub fn set_game_sdk_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.game_sdk_version = input;
        self
    }
    /// Consumes the builder and constructs a [`Generator`](crate::types::Generator).
    pub fn build(self) -> crate::types::Generator {
        crate::types::Generator {
            target_platform: self.target_platform,
            language: self.language,
            game_sdk_version: self.game_sdk_version,
        }
    }
}
