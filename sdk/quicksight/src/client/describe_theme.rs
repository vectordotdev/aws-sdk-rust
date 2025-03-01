// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeTheme`](crate::operation::describe_theme::builders::DescribeThemeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::describe_theme::builders::DescribeThemeFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::describe_theme::builders::DescribeThemeFluentBuilder::set_aws_account_id): <p>The ID of the Amazon Web Services account that contains the theme that you're describing.</p>
    ///   - [`theme_id(impl ::std::convert::Into<String>)`](crate::operation::describe_theme::builders::DescribeThemeFluentBuilder::theme_id) / [`set_theme_id(Option<String>)`](crate::operation::describe_theme::builders::DescribeThemeFluentBuilder::set_theme_id): <p>The ID for the theme.</p>
    ///   - [`version_number(i64)`](crate::operation::describe_theme::builders::DescribeThemeFluentBuilder::version_number) / [`set_version_number(Option<i64>)`](crate::operation::describe_theme::builders::DescribeThemeFluentBuilder::set_version_number): <p>The version number for the version to describe. If a <code>VersionNumber</code> parameter value isn't provided, the latest version of the theme is described.</p>
    ///   - [`alias_name(impl ::std::convert::Into<String>)`](crate::operation::describe_theme::builders::DescribeThemeFluentBuilder::alias_name) / [`set_alias_name(Option<String>)`](crate::operation::describe_theme::builders::DescribeThemeFluentBuilder::set_alias_name): <p>The alias of the theme that you want to describe. If you name a specific alias, you describe the version that the alias points to. You can specify the latest version of the theme by providing the keyword <code>$LATEST</code> in the <code>AliasName</code> parameter. The keyword <code>$PUBLISHED</code> doesn't apply to themes.</p>
    /// - On success, responds with [`DescribeThemeOutput`](crate::operation::describe_theme::DescribeThemeOutput) with field(s):
    ///   - [`theme(Option<Theme>)`](crate::operation::describe_theme::DescribeThemeOutput::theme): <p>The information about the theme that you are describing.</p>
    ///   - [`status(i32)`](crate::operation::describe_theme::DescribeThemeOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::describe_theme::DescribeThemeOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    /// - On failure, responds with [`SdkError<DescribeThemeError>`](crate::operation::describe_theme::DescribeThemeError)
    pub fn describe_theme(
        &self,
    ) -> crate::operation::describe_theme::builders::DescribeThemeFluentBuilder {
        crate::operation::describe_theme::builders::DescribeThemeFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
