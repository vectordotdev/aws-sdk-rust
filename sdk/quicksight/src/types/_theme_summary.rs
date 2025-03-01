// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The theme summary.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ThemeSummary {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>the display name for the theme.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the theme. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    #[doc(hidden)]
    pub theme_id: ::std::option::Option<::std::string::String>,
    /// <p>The latest version number for the theme. </p>
    #[doc(hidden)]
    pub latest_version_number: ::std::option::Option<i64>,
    /// <p>The date and time that this theme was created.</p>
    #[doc(hidden)]
    pub created_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The last date and time that this theme was updated.</p>
    #[doc(hidden)]
    pub last_updated_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ThemeSummary {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>the display name for the theme.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The ID of the theme. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    pub fn theme_id(&self) -> ::std::option::Option<&str> {
        self.theme_id.as_deref()
    }
    /// <p>The latest version number for the theme. </p>
    pub fn latest_version_number(&self) -> ::std::option::Option<i64> {
        self.latest_version_number
    }
    /// <p>The date and time that this theme was created.</p>
    pub fn created_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_time.as_ref()
    }
    /// <p>The last date and time that this theme was updated.</p>
    pub fn last_updated_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_updated_time.as_ref()
    }
}
impl ThemeSummary {
    /// Creates a new builder-style object to manufacture [`ThemeSummary`](crate::types::ThemeSummary).
    pub fn builder() -> crate::types::builders::ThemeSummaryBuilder {
        crate::types::builders::ThemeSummaryBuilder::default()
    }
}

/// A builder for [`ThemeSummary`](crate::types::ThemeSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ThemeSummaryBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) theme_id: ::std::option::Option<::std::string::String>,
    pub(crate) latest_version_number: ::std::option::Option<i64>,
    pub(crate) created_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_updated_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ThemeSummaryBuilder {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>the display name for the theme.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>the display name for the theme.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The ID of the theme. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    pub fn theme_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.theme_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the theme. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    pub fn set_theme_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.theme_id = input;
        self
    }
    /// <p>The latest version number for the theme. </p>
    pub fn latest_version_number(mut self, input: i64) -> Self {
        self.latest_version_number = ::std::option::Option::Some(input);
        self
    }
    /// <p>The latest version number for the theme. </p>
    pub fn set_latest_version_number(mut self, input: ::std::option::Option<i64>) -> Self {
        self.latest_version_number = input;
        self
    }
    /// <p>The date and time that this theme was created.</p>
    pub fn created_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time that this theme was created.</p>
    pub fn set_created_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_time = input;
        self
    }
    /// <p>The last date and time that this theme was updated.</p>
    pub fn last_updated_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The last date and time that this theme was updated.</p>
    pub fn set_last_updated_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_updated_time = input;
        self
    }
    /// Consumes the builder and constructs a [`ThemeSummary`](crate::types::ThemeSummary).
    pub fn build(self) -> crate::types::ThemeSummary {
        crate::types::ThemeSummary {
            arn: self.arn,
            name: self.name,
            theme_id: self.theme_id,
            latest_version_number: self.latest_version_number,
            created_time: self.created_time,
            last_updated_time: self.last_updated_time,
        }
    }
}
