// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the configuration information to connect to Jira as your data source.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct JiraConfiguration {
    /// <p>The URL of the Jira account. For example, <i>company.atlassian.net</i>.</p>
    #[doc(hidden)]
    pub jira_account_url: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of a secret in Secrets Manager contains the key-value pairs required to connect to your Jira data source. The secret must contain a JSON structure with the following keys:</p>
    /// <ul>
    /// <li> <p>jiraId—The Jira user name or email.</p> </li>
    /// <li> <p>jiraCredentials—The Jira API token. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/data-source-jira.html">Using a Jira data source</a>.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub secret_arn: ::std::option::Option<::std::string::String>,
    /// <p> <code>TRUE</code> to use the Jira change log to determine which documents require updating in the index. Depending on the change log's size, it may take longer for Amazon Kendra to use the change log than to scan all of your documents in Jira.</p>
    #[doc(hidden)]
    pub use_change_log: bool,
    /// <p>Specify which projects to crawl in your Jira data source. You can specify one or more Jira project IDs.</p>
    #[doc(hidden)]
    pub project: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Specify which issue types to crawl in your Jira data source. You can specify one or more of these options to crawl.</p>
    #[doc(hidden)]
    pub issue_type: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Specify which statuses to crawl in your Jira data source. You can specify one or more of these options to crawl.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Specify whether to crawl comments, attachments, and work logs. You can specify one or more of these options.</p>
    #[doc(hidden)]
    pub issue_sub_entity_filter:
        ::std::option::Option<::std::vec::Vec<crate::types::IssueSubEntity>>,
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira attachments to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    #[doc(hidden)]
    pub attachment_field_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira comments to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    #[doc(hidden)]
    pub comment_field_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira issues to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    #[doc(hidden)]
    pub issue_field_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira projects to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    #[doc(hidden)]
    pub project_field_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira work logs to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    #[doc(hidden)]
    pub work_log_field_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    /// <p>A list of regular expression patterns to include certain file paths, file names, and file types in your Jira data source. Files that match the patterns are included in the index. Files that don't match the patterns are excluded from the index. If a file matches both an inclusion pattern and an exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    #[doc(hidden)]
    pub inclusion_patterns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A list of regular expression patterns to exclude certain file paths, file names, and file types in your Jira data source. Files that match the patterns are excluded from the index. Files that don’t match the patterns are included in the index. If a file matches both an inclusion pattern and an exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    #[doc(hidden)]
    pub exclusion_patterns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Configuration information for an Amazon Virtual Private Cloud to connect to your Jira. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/vpc-configuration.html">Configuring a VPC</a>.</p>
    #[doc(hidden)]
    pub vpc_configuration: ::std::option::Option<crate::types::DataSourceVpcConfiguration>,
}
impl JiraConfiguration {
    /// <p>The URL of the Jira account. For example, <i>company.atlassian.net</i>.</p>
    pub fn jira_account_url(&self) -> ::std::option::Option<&str> {
        self.jira_account_url.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of a secret in Secrets Manager contains the key-value pairs required to connect to your Jira data source. The secret must contain a JSON structure with the following keys:</p>
    /// <ul>
    /// <li> <p>jiraId—The Jira user name or email.</p> </li>
    /// <li> <p>jiraCredentials—The Jira API token. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/data-source-jira.html">Using a Jira data source</a>.</p> </li>
    /// </ul>
    pub fn secret_arn(&self) -> ::std::option::Option<&str> {
        self.secret_arn.as_deref()
    }
    /// <p> <code>TRUE</code> to use the Jira change log to determine which documents require updating in the index. Depending on the change log's size, it may take longer for Amazon Kendra to use the change log than to scan all of your documents in Jira.</p>
    pub fn use_change_log(&self) -> bool {
        self.use_change_log
    }
    /// <p>Specify which projects to crawl in your Jira data source. You can specify one or more Jira project IDs.</p>
    pub fn project(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.project.as_deref()
    }
    /// <p>Specify which issue types to crawl in your Jira data source. You can specify one or more of these options to crawl.</p>
    pub fn issue_type(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.issue_type.as_deref()
    }
    /// <p>Specify which statuses to crawl in your Jira data source. You can specify one or more of these options to crawl.</p>
    pub fn status(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.status.as_deref()
    }
    /// <p>Specify whether to crawl comments, attachments, and work logs. You can specify one or more of these options.</p>
    pub fn issue_sub_entity_filter(
        &self,
    ) -> ::std::option::Option<&[crate::types::IssueSubEntity]> {
        self.issue_sub_entity_filter.as_deref()
    }
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira attachments to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn attachment_field_mappings(
        &self,
    ) -> ::std::option::Option<&[crate::types::DataSourceToIndexFieldMapping]> {
        self.attachment_field_mappings.as_deref()
    }
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira comments to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn comment_field_mappings(
        &self,
    ) -> ::std::option::Option<&[crate::types::DataSourceToIndexFieldMapping]> {
        self.comment_field_mappings.as_deref()
    }
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira issues to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn issue_field_mappings(
        &self,
    ) -> ::std::option::Option<&[crate::types::DataSourceToIndexFieldMapping]> {
        self.issue_field_mappings.as_deref()
    }
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira projects to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn project_field_mappings(
        &self,
    ) -> ::std::option::Option<&[crate::types::DataSourceToIndexFieldMapping]> {
        self.project_field_mappings.as_deref()
    }
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira work logs to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn work_log_field_mappings(
        &self,
    ) -> ::std::option::Option<&[crate::types::DataSourceToIndexFieldMapping]> {
        self.work_log_field_mappings.as_deref()
    }
    /// <p>A list of regular expression patterns to include certain file paths, file names, and file types in your Jira data source. Files that match the patterns are included in the index. Files that don't match the patterns are excluded from the index. If a file matches both an inclusion pattern and an exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    pub fn inclusion_patterns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.inclusion_patterns.as_deref()
    }
    /// <p>A list of regular expression patterns to exclude certain file paths, file names, and file types in your Jira data source. Files that match the patterns are excluded from the index. Files that don’t match the patterns are included in the index. If a file matches both an inclusion pattern and an exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    pub fn exclusion_patterns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.exclusion_patterns.as_deref()
    }
    /// <p>Configuration information for an Amazon Virtual Private Cloud to connect to your Jira. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/vpc-configuration.html">Configuring a VPC</a>.</p>
    pub fn vpc_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::DataSourceVpcConfiguration> {
        self.vpc_configuration.as_ref()
    }
}
impl JiraConfiguration {
    /// Creates a new builder-style object to manufacture [`JiraConfiguration`](crate::types::JiraConfiguration).
    pub fn builder() -> crate::types::builders::JiraConfigurationBuilder {
        crate::types::builders::JiraConfigurationBuilder::default()
    }
}

/// A builder for [`JiraConfiguration`](crate::types::JiraConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct JiraConfigurationBuilder {
    pub(crate) jira_account_url: ::std::option::Option<::std::string::String>,
    pub(crate) secret_arn: ::std::option::Option<::std::string::String>,
    pub(crate) use_change_log: ::std::option::Option<bool>,
    pub(crate) project: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) issue_type: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) status: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) issue_sub_entity_filter:
        ::std::option::Option<::std::vec::Vec<crate::types::IssueSubEntity>>,
    pub(crate) attachment_field_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    pub(crate) comment_field_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    pub(crate) issue_field_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    pub(crate) project_field_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    pub(crate) work_log_field_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    pub(crate) inclusion_patterns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) exclusion_patterns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) vpc_configuration: ::std::option::Option<crate::types::DataSourceVpcConfiguration>,
}
impl JiraConfigurationBuilder {
    /// <p>The URL of the Jira account. For example, <i>company.atlassian.net</i>.</p>
    pub fn jira_account_url(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.jira_account_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The URL of the Jira account. For example, <i>company.atlassian.net</i>.</p>
    pub fn set_jira_account_url(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.jira_account_url = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a secret in Secrets Manager contains the key-value pairs required to connect to your Jira data source. The secret must contain a JSON structure with the following keys:</p>
    /// <ul>
    /// <li> <p>jiraId—The Jira user name or email.</p> </li>
    /// <li> <p>jiraCredentials—The Jira API token. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/data-source-jira.html">Using a Jira data source</a>.</p> </li>
    /// </ul>
    pub fn secret_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.secret_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a secret in Secrets Manager contains the key-value pairs required to connect to your Jira data source. The secret must contain a JSON structure with the following keys:</p>
    /// <ul>
    /// <li> <p>jiraId—The Jira user name or email.</p> </li>
    /// <li> <p>jiraCredentials—The Jira API token. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/data-source-jira.html">Using a Jira data source</a>.</p> </li>
    /// </ul>
    pub fn set_secret_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.secret_arn = input;
        self
    }
    /// <p> <code>TRUE</code> to use the Jira change log to determine which documents require updating in the index. Depending on the change log's size, it may take longer for Amazon Kendra to use the change log than to scan all of your documents in Jira.</p>
    pub fn use_change_log(mut self, input: bool) -> Self {
        self.use_change_log = ::std::option::Option::Some(input);
        self
    }
    /// <p> <code>TRUE</code> to use the Jira change log to determine which documents require updating in the index. Depending on the change log's size, it may take longer for Amazon Kendra to use the change log than to scan all of your documents in Jira.</p>
    pub fn set_use_change_log(mut self, input: ::std::option::Option<bool>) -> Self {
        self.use_change_log = input;
        self
    }
    /// Appends an item to `project`.
    ///
    /// To override the contents of this collection use [`set_project`](Self::set_project).
    ///
    /// <p>Specify which projects to crawl in your Jira data source. You can specify one or more Jira project IDs.</p>
    pub fn project(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.project.unwrap_or_default();
        v.push(input.into());
        self.project = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specify which projects to crawl in your Jira data source. You can specify one or more Jira project IDs.</p>
    pub fn set_project(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.project = input;
        self
    }
    /// Appends an item to `issue_type`.
    ///
    /// To override the contents of this collection use [`set_issue_type`](Self::set_issue_type).
    ///
    /// <p>Specify which issue types to crawl in your Jira data source. You can specify one or more of these options to crawl.</p>
    pub fn issue_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.issue_type.unwrap_or_default();
        v.push(input.into());
        self.issue_type = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specify which issue types to crawl in your Jira data source. You can specify one or more of these options to crawl.</p>
    pub fn set_issue_type(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.issue_type = input;
        self
    }
    /// Appends an item to `status`.
    ///
    /// To override the contents of this collection use [`set_status`](Self::set_status).
    ///
    /// <p>Specify which statuses to crawl in your Jira data source. You can specify one or more of these options to crawl.</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.status.unwrap_or_default();
        v.push(input.into());
        self.status = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specify which statuses to crawl in your Jira data source. You can specify one or more of these options to crawl.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.status = input;
        self
    }
    /// Appends an item to `issue_sub_entity_filter`.
    ///
    /// To override the contents of this collection use [`set_issue_sub_entity_filter`](Self::set_issue_sub_entity_filter).
    ///
    /// <p>Specify whether to crawl comments, attachments, and work logs. You can specify one or more of these options.</p>
    pub fn issue_sub_entity_filter(mut self, input: crate::types::IssueSubEntity) -> Self {
        let mut v = self.issue_sub_entity_filter.unwrap_or_default();
        v.push(input);
        self.issue_sub_entity_filter = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specify whether to crawl comments, attachments, and work logs. You can specify one or more of these options.</p>
    pub fn set_issue_sub_entity_filter(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::IssueSubEntity>>,
    ) -> Self {
        self.issue_sub_entity_filter = input;
        self
    }
    /// Appends an item to `attachment_field_mappings`.
    ///
    /// To override the contents of this collection use [`set_attachment_field_mappings`](Self::set_attachment_field_mappings).
    ///
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira attachments to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn attachment_field_mappings(
        mut self,
        input: crate::types::DataSourceToIndexFieldMapping,
    ) -> Self {
        let mut v = self.attachment_field_mappings.unwrap_or_default();
        v.push(input);
        self.attachment_field_mappings = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira attachments to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn set_attachment_field_mappings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    ) -> Self {
        self.attachment_field_mappings = input;
        self
    }
    /// Appends an item to `comment_field_mappings`.
    ///
    /// To override the contents of this collection use [`set_comment_field_mappings`](Self::set_comment_field_mappings).
    ///
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira comments to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn comment_field_mappings(
        mut self,
        input: crate::types::DataSourceToIndexFieldMapping,
    ) -> Self {
        let mut v = self.comment_field_mappings.unwrap_or_default();
        v.push(input);
        self.comment_field_mappings = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira comments to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn set_comment_field_mappings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    ) -> Self {
        self.comment_field_mappings = input;
        self
    }
    /// Appends an item to `issue_field_mappings`.
    ///
    /// To override the contents of this collection use [`set_issue_field_mappings`](Self::set_issue_field_mappings).
    ///
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira issues to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn issue_field_mappings(
        mut self,
        input: crate::types::DataSourceToIndexFieldMapping,
    ) -> Self {
        let mut v = self.issue_field_mappings.unwrap_or_default();
        v.push(input);
        self.issue_field_mappings = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira issues to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn set_issue_field_mappings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    ) -> Self {
        self.issue_field_mappings = input;
        self
    }
    /// Appends an item to `project_field_mappings`.
    ///
    /// To override the contents of this collection use [`set_project_field_mappings`](Self::set_project_field_mappings).
    ///
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira projects to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn project_field_mappings(
        mut self,
        input: crate::types::DataSourceToIndexFieldMapping,
    ) -> Self {
        let mut v = self.project_field_mappings.unwrap_or_default();
        v.push(input);
        self.project_field_mappings = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira projects to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn set_project_field_mappings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    ) -> Self {
        self.project_field_mappings = input;
        self
    }
    /// Appends an item to `work_log_field_mappings`.
    ///
    /// To override the contents of this collection use [`set_work_log_field_mappings`](Self::set_work_log_field_mappings).
    ///
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira work logs to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn work_log_field_mappings(
        mut self,
        input: crate::types::DataSourceToIndexFieldMapping,
    ) -> Self {
        let mut v = self.work_log_field_mappings.unwrap_or_default();
        v.push(input);
        self.work_log_field_mappings = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map attributes or field names of Jira work logs to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Jira fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html"> Mapping data source fields</a>. The Jira data source field names must exist in your Jira custom metadata.</p>
    pub fn set_work_log_field_mappings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    ) -> Self {
        self.work_log_field_mappings = input;
        self
    }
    /// Appends an item to `inclusion_patterns`.
    ///
    /// To override the contents of this collection use [`set_inclusion_patterns`](Self::set_inclusion_patterns).
    ///
    /// <p>A list of regular expression patterns to include certain file paths, file names, and file types in your Jira data source. Files that match the patterns are included in the index. Files that don't match the patterns are excluded from the index. If a file matches both an inclusion pattern and an exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    pub fn inclusion_patterns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.inclusion_patterns.unwrap_or_default();
        v.push(input.into());
        self.inclusion_patterns = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of regular expression patterns to include certain file paths, file names, and file types in your Jira data source. Files that match the patterns are included in the index. Files that don't match the patterns are excluded from the index. If a file matches both an inclusion pattern and an exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    pub fn set_inclusion_patterns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inclusion_patterns = input;
        self
    }
    /// Appends an item to `exclusion_patterns`.
    ///
    /// To override the contents of this collection use [`set_exclusion_patterns`](Self::set_exclusion_patterns).
    ///
    /// <p>A list of regular expression patterns to exclude certain file paths, file names, and file types in your Jira data source. Files that match the patterns are excluded from the index. Files that don’t match the patterns are included in the index. If a file matches both an inclusion pattern and an exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    pub fn exclusion_patterns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.exclusion_patterns.unwrap_or_default();
        v.push(input.into());
        self.exclusion_patterns = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of regular expression patterns to exclude certain file paths, file names, and file types in your Jira data source. Files that match the patterns are excluded from the index. Files that don’t match the patterns are included in the index. If a file matches both an inclusion pattern and an exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    pub fn set_exclusion_patterns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.exclusion_patterns = input;
        self
    }
    /// <p>Configuration information for an Amazon Virtual Private Cloud to connect to your Jira. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/vpc-configuration.html">Configuring a VPC</a>.</p>
    pub fn vpc_configuration(mut self, input: crate::types::DataSourceVpcConfiguration) -> Self {
        self.vpc_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Configuration information for an Amazon Virtual Private Cloud to connect to your Jira. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/vpc-configuration.html">Configuring a VPC</a>.</p>
    pub fn set_vpc_configuration(
        mut self,
        input: ::std::option::Option<crate::types::DataSourceVpcConfiguration>,
    ) -> Self {
        self.vpc_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`JiraConfiguration`](crate::types::JiraConfiguration).
    pub fn build(self) -> crate::types::JiraConfiguration {
        crate::types::JiraConfiguration {
            jira_account_url: self.jira_account_url,
            secret_arn: self.secret_arn,
            use_change_log: self.use_change_log.unwrap_or_default(),
            project: self.project,
            issue_type: self.issue_type,
            status: self.status,
            issue_sub_entity_filter: self.issue_sub_entity_filter,
            attachment_field_mappings: self.attachment_field_mappings,
            comment_field_mappings: self.comment_field_mappings,
            issue_field_mappings: self.issue_field_mappings,
            project_field_mappings: self.project_field_mappings,
            work_log_field_mappings: self.work_log_field_mappings,
            inclusion_patterns: self.inclusion_patterns,
            exclusion_patterns: self.exclusion_patterns,
            vpc_configuration: self.vpc_configuration,
        }
    }
}
