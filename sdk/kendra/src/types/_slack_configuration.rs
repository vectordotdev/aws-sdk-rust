// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the configuration information to connect to Slack as your data source.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SlackConfiguration {
    /// <p>The identifier of the team in the Slack workspace. For example, <i>T0123456789</i>.</p>
    /// <p>You can find your team ID in the URL of the main page of your Slack workspace. When you log in to Slack via a browser, you are directed to the URL of the main page. For example, <i>https://app.slack.com/client/<b>T0123456789</b>/...</i>.</p>
    #[doc(hidden)]
    pub team_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of an Secrets Manager secret that contains the key-value pairs required to connect to your Slack workspace team. The secret must contain a JSON structure with the following keys:</p>
    /// <ul>
    /// <li> <p>slackToken—The user or bot token created in Slack. For more information on creating a token in Slack, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/data-source-slack.html#slack-authentication">Authentication for a Slack data source</a>.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub secret_arn: ::std::option::Option<::std::string::String>,
    /// <p>Configuration information for an Amazon Virtual Private Cloud to connect to your Slack. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/vpc-configuration.html">Configuring a VPC</a>.</p>
    #[doc(hidden)]
    pub vpc_configuration: ::std::option::Option<crate::types::DataSourceVpcConfiguration>,
    /// <p>Specify whether to index public channels, private channels, group messages, and direct messages. You can specify one or more of these options.</p>
    #[doc(hidden)]
    pub slack_entity_list: ::std::option::Option<::std::vec::Vec<crate::types::SlackEntity>>,
    /// <p> <code>TRUE</code> to use the Slack change log to determine which documents require updating in the index. Depending on the Slack change log's size, it may take longer for Amazon Kendra to use the change log than to scan all of your documents in Slack.</p>
    #[doc(hidden)]
    pub use_change_log: bool,
    /// <p> <code>TRUE</code> to index bot messages from your Slack workspace team.</p>
    #[doc(hidden)]
    pub crawl_bot_message: bool,
    /// <p> <code>TRUE</code> to exclude archived messages to index from your Slack workspace team.</p>
    #[doc(hidden)]
    pub exclude_archived: bool,
    /// <p>The date to start crawling your data from your Slack workspace team. The date must follow this format: <code>yyyy-mm-dd</code>.</p>
    #[doc(hidden)]
    pub since_crawl_date: ::std::option::Option<::std::string::String>,
    /// <p>The number of hours for change log to look back from when you last synchronized your data. You can look back up to 7 days or 168 hours.</p>
    /// <p>Change log updates your index only if new content was added since you last synced your data. Updated or deleted content from before you last synced does not get updated in your index. To capture updated or deleted content before you last synced, set the <code>LookBackPeriod</code> to the number of hours you want change log to look back.</p>
    #[doc(hidden)]
    pub look_back_period: ::std::option::Option<i32>,
    /// <p>The list of private channel names from your Slack workspace team. You use this if you want to index specific private channels, not all private channels. You can also use regular expression patterns to filter private channels.</p>
    #[doc(hidden)]
    pub private_channel_filter: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The list of public channel names to index from your Slack workspace team. You use this if you want to index specific public channels, not all public channels. You can also use regular expression patterns to filter public channels.</p>
    #[doc(hidden)]
    pub public_channel_filter: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A list of regular expression patterns to include certain attached files in your Slack workspace team. Files that match the patterns are included in the index. Files that don't match the patterns are excluded from the index. If a file matches both an inclusion and exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    #[doc(hidden)]
    pub inclusion_patterns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A list of regular expression patterns to exclude certain attached files in your Slack workspace team. Files that match the patterns are excluded from the index. Files that don’t match the patterns are included in the index. If a file matches both an inclusion and exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    #[doc(hidden)]
    pub exclusion_patterns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map Slack data source attributes or field names to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Slack fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html">Mapping data source fields</a>. The Slack data source field names must exist in your Slack custom metadata.</p>
    #[doc(hidden)]
    pub field_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
}
impl SlackConfiguration {
    /// <p>The identifier of the team in the Slack workspace. For example, <i>T0123456789</i>.</p>
    /// <p>You can find your team ID in the URL of the main page of your Slack workspace. When you log in to Slack via a browser, you are directed to the URL of the main page. For example, <i>https://app.slack.com/client/<b>T0123456789</b>/...</i>.</p>
    pub fn team_id(&self) -> ::std::option::Option<&str> {
        self.team_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of an Secrets Manager secret that contains the key-value pairs required to connect to your Slack workspace team. The secret must contain a JSON structure with the following keys:</p>
    /// <ul>
    /// <li> <p>slackToken—The user or bot token created in Slack. For more information on creating a token in Slack, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/data-source-slack.html#slack-authentication">Authentication for a Slack data source</a>.</p> </li>
    /// </ul>
    pub fn secret_arn(&self) -> ::std::option::Option<&str> {
        self.secret_arn.as_deref()
    }
    /// <p>Configuration information for an Amazon Virtual Private Cloud to connect to your Slack. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/vpc-configuration.html">Configuring a VPC</a>.</p>
    pub fn vpc_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::DataSourceVpcConfiguration> {
        self.vpc_configuration.as_ref()
    }
    /// <p>Specify whether to index public channels, private channels, group messages, and direct messages. You can specify one or more of these options.</p>
    pub fn slack_entity_list(&self) -> ::std::option::Option<&[crate::types::SlackEntity]> {
        self.slack_entity_list.as_deref()
    }
    /// <p> <code>TRUE</code> to use the Slack change log to determine which documents require updating in the index. Depending on the Slack change log's size, it may take longer for Amazon Kendra to use the change log than to scan all of your documents in Slack.</p>
    pub fn use_change_log(&self) -> bool {
        self.use_change_log
    }
    /// <p> <code>TRUE</code> to index bot messages from your Slack workspace team.</p>
    pub fn crawl_bot_message(&self) -> bool {
        self.crawl_bot_message
    }
    /// <p> <code>TRUE</code> to exclude archived messages to index from your Slack workspace team.</p>
    pub fn exclude_archived(&self) -> bool {
        self.exclude_archived
    }
    /// <p>The date to start crawling your data from your Slack workspace team. The date must follow this format: <code>yyyy-mm-dd</code>.</p>
    pub fn since_crawl_date(&self) -> ::std::option::Option<&str> {
        self.since_crawl_date.as_deref()
    }
    /// <p>The number of hours for change log to look back from when you last synchronized your data. You can look back up to 7 days or 168 hours.</p>
    /// <p>Change log updates your index only if new content was added since you last synced your data. Updated or deleted content from before you last synced does not get updated in your index. To capture updated or deleted content before you last synced, set the <code>LookBackPeriod</code> to the number of hours you want change log to look back.</p>
    pub fn look_back_period(&self) -> ::std::option::Option<i32> {
        self.look_back_period
    }
    /// <p>The list of private channel names from your Slack workspace team. You use this if you want to index specific private channels, not all private channels. You can also use regular expression patterns to filter private channels.</p>
    pub fn private_channel_filter(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.private_channel_filter.as_deref()
    }
    /// <p>The list of public channel names to index from your Slack workspace team. You use this if you want to index specific public channels, not all public channels. You can also use regular expression patterns to filter public channels.</p>
    pub fn public_channel_filter(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.public_channel_filter.as_deref()
    }
    /// <p>A list of regular expression patterns to include certain attached files in your Slack workspace team. Files that match the patterns are included in the index. Files that don't match the patterns are excluded from the index. If a file matches both an inclusion and exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    pub fn inclusion_patterns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.inclusion_patterns.as_deref()
    }
    /// <p>A list of regular expression patterns to exclude certain attached files in your Slack workspace team. Files that match the patterns are excluded from the index. Files that don’t match the patterns are included in the index. If a file matches both an inclusion and exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    pub fn exclusion_patterns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.exclusion_patterns.as_deref()
    }
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map Slack data source attributes or field names to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Slack fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html">Mapping data source fields</a>. The Slack data source field names must exist in your Slack custom metadata.</p>
    pub fn field_mappings(
        &self,
    ) -> ::std::option::Option<&[crate::types::DataSourceToIndexFieldMapping]> {
        self.field_mappings.as_deref()
    }
}
impl SlackConfiguration {
    /// Creates a new builder-style object to manufacture [`SlackConfiguration`](crate::types::SlackConfiguration).
    pub fn builder() -> crate::types::builders::SlackConfigurationBuilder {
        crate::types::builders::SlackConfigurationBuilder::default()
    }
}

/// A builder for [`SlackConfiguration`](crate::types::SlackConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SlackConfigurationBuilder {
    pub(crate) team_id: ::std::option::Option<::std::string::String>,
    pub(crate) secret_arn: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_configuration: ::std::option::Option<crate::types::DataSourceVpcConfiguration>,
    pub(crate) slack_entity_list: ::std::option::Option<::std::vec::Vec<crate::types::SlackEntity>>,
    pub(crate) use_change_log: ::std::option::Option<bool>,
    pub(crate) crawl_bot_message: ::std::option::Option<bool>,
    pub(crate) exclude_archived: ::std::option::Option<bool>,
    pub(crate) since_crawl_date: ::std::option::Option<::std::string::String>,
    pub(crate) look_back_period: ::std::option::Option<i32>,
    pub(crate) private_channel_filter:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) public_channel_filter: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) inclusion_patterns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) exclusion_patterns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) field_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
}
impl SlackConfigurationBuilder {
    /// <p>The identifier of the team in the Slack workspace. For example, <i>T0123456789</i>.</p>
    /// <p>You can find your team ID in the URL of the main page of your Slack workspace. When you log in to Slack via a browser, you are directed to the URL of the main page. For example, <i>https://app.slack.com/client/<b>T0123456789</b>/...</i>.</p>
    pub fn team_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.team_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the team in the Slack workspace. For example, <i>T0123456789</i>.</p>
    /// <p>You can find your team ID in the URL of the main page of your Slack workspace. When you log in to Slack via a browser, you are directed to the URL of the main page. For example, <i>https://app.slack.com/client/<b>T0123456789</b>/...</i>.</p>
    pub fn set_team_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.team_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an Secrets Manager secret that contains the key-value pairs required to connect to your Slack workspace team. The secret must contain a JSON structure with the following keys:</p>
    /// <ul>
    /// <li> <p>slackToken—The user or bot token created in Slack. For more information on creating a token in Slack, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/data-source-slack.html#slack-authentication">Authentication for a Slack data source</a>.</p> </li>
    /// </ul>
    pub fn secret_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.secret_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an Secrets Manager secret that contains the key-value pairs required to connect to your Slack workspace team. The secret must contain a JSON structure with the following keys:</p>
    /// <ul>
    /// <li> <p>slackToken—The user or bot token created in Slack. For more information on creating a token in Slack, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/data-source-slack.html#slack-authentication">Authentication for a Slack data source</a>.</p> </li>
    /// </ul>
    pub fn set_secret_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.secret_arn = input;
        self
    }
    /// <p>Configuration information for an Amazon Virtual Private Cloud to connect to your Slack. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/vpc-configuration.html">Configuring a VPC</a>.</p>
    pub fn vpc_configuration(mut self, input: crate::types::DataSourceVpcConfiguration) -> Self {
        self.vpc_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Configuration information for an Amazon Virtual Private Cloud to connect to your Slack. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/vpc-configuration.html">Configuring a VPC</a>.</p>
    pub fn set_vpc_configuration(
        mut self,
        input: ::std::option::Option<crate::types::DataSourceVpcConfiguration>,
    ) -> Self {
        self.vpc_configuration = input;
        self
    }
    /// Appends an item to `slack_entity_list`.
    ///
    /// To override the contents of this collection use [`set_slack_entity_list`](Self::set_slack_entity_list).
    ///
    /// <p>Specify whether to index public channels, private channels, group messages, and direct messages. You can specify one or more of these options.</p>
    pub fn slack_entity_list(mut self, input: crate::types::SlackEntity) -> Self {
        let mut v = self.slack_entity_list.unwrap_or_default();
        v.push(input);
        self.slack_entity_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specify whether to index public channels, private channels, group messages, and direct messages. You can specify one or more of these options.</p>
    pub fn set_slack_entity_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SlackEntity>>,
    ) -> Self {
        self.slack_entity_list = input;
        self
    }
    /// <p> <code>TRUE</code> to use the Slack change log to determine which documents require updating in the index. Depending on the Slack change log's size, it may take longer for Amazon Kendra to use the change log than to scan all of your documents in Slack.</p>
    pub fn use_change_log(mut self, input: bool) -> Self {
        self.use_change_log = ::std::option::Option::Some(input);
        self
    }
    /// <p> <code>TRUE</code> to use the Slack change log to determine which documents require updating in the index. Depending on the Slack change log's size, it may take longer for Amazon Kendra to use the change log than to scan all of your documents in Slack.</p>
    pub fn set_use_change_log(mut self, input: ::std::option::Option<bool>) -> Self {
        self.use_change_log = input;
        self
    }
    /// <p> <code>TRUE</code> to index bot messages from your Slack workspace team.</p>
    pub fn crawl_bot_message(mut self, input: bool) -> Self {
        self.crawl_bot_message = ::std::option::Option::Some(input);
        self
    }
    /// <p> <code>TRUE</code> to index bot messages from your Slack workspace team.</p>
    pub fn set_crawl_bot_message(mut self, input: ::std::option::Option<bool>) -> Self {
        self.crawl_bot_message = input;
        self
    }
    /// <p> <code>TRUE</code> to exclude archived messages to index from your Slack workspace team.</p>
    pub fn exclude_archived(mut self, input: bool) -> Self {
        self.exclude_archived = ::std::option::Option::Some(input);
        self
    }
    /// <p> <code>TRUE</code> to exclude archived messages to index from your Slack workspace team.</p>
    pub fn set_exclude_archived(mut self, input: ::std::option::Option<bool>) -> Self {
        self.exclude_archived = input;
        self
    }
    /// <p>The date to start crawling your data from your Slack workspace team. The date must follow this format: <code>yyyy-mm-dd</code>.</p>
    pub fn since_crawl_date(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.since_crawl_date = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The date to start crawling your data from your Slack workspace team. The date must follow this format: <code>yyyy-mm-dd</code>.</p>
    pub fn set_since_crawl_date(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.since_crawl_date = input;
        self
    }
    /// <p>The number of hours for change log to look back from when you last synchronized your data. You can look back up to 7 days or 168 hours.</p>
    /// <p>Change log updates your index only if new content was added since you last synced your data. Updated or deleted content from before you last synced does not get updated in your index. To capture updated or deleted content before you last synced, set the <code>LookBackPeriod</code> to the number of hours you want change log to look back.</p>
    pub fn look_back_period(mut self, input: i32) -> Self {
        self.look_back_period = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of hours for change log to look back from when you last synchronized your data. You can look back up to 7 days or 168 hours.</p>
    /// <p>Change log updates your index only if new content was added since you last synced your data. Updated or deleted content from before you last synced does not get updated in your index. To capture updated or deleted content before you last synced, set the <code>LookBackPeriod</code> to the number of hours you want change log to look back.</p>
    pub fn set_look_back_period(mut self, input: ::std::option::Option<i32>) -> Self {
        self.look_back_period = input;
        self
    }
    /// Appends an item to `private_channel_filter`.
    ///
    /// To override the contents of this collection use [`set_private_channel_filter`](Self::set_private_channel_filter).
    ///
    /// <p>The list of private channel names from your Slack workspace team. You use this if you want to index specific private channels, not all private channels. You can also use regular expression patterns to filter private channels.</p>
    pub fn private_channel_filter(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.private_channel_filter.unwrap_or_default();
        v.push(input.into());
        self.private_channel_filter = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of private channel names from your Slack workspace team. You use this if you want to index specific private channels, not all private channels. You can also use regular expression patterns to filter private channels.</p>
    pub fn set_private_channel_filter(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.private_channel_filter = input;
        self
    }
    /// Appends an item to `public_channel_filter`.
    ///
    /// To override the contents of this collection use [`set_public_channel_filter`](Self::set_public_channel_filter).
    ///
    /// <p>The list of public channel names to index from your Slack workspace team. You use this if you want to index specific public channels, not all public channels. You can also use regular expression patterns to filter public channels.</p>
    pub fn public_channel_filter(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.public_channel_filter.unwrap_or_default();
        v.push(input.into());
        self.public_channel_filter = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of public channel names to index from your Slack workspace team. You use this if you want to index specific public channels, not all public channels. You can also use regular expression patterns to filter public channels.</p>
    pub fn set_public_channel_filter(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.public_channel_filter = input;
        self
    }
    /// Appends an item to `inclusion_patterns`.
    ///
    /// To override the contents of this collection use [`set_inclusion_patterns`](Self::set_inclusion_patterns).
    ///
    /// <p>A list of regular expression patterns to include certain attached files in your Slack workspace team. Files that match the patterns are included in the index. Files that don't match the patterns are excluded from the index. If a file matches both an inclusion and exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    pub fn inclusion_patterns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.inclusion_patterns.unwrap_or_default();
        v.push(input.into());
        self.inclusion_patterns = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of regular expression patterns to include certain attached files in your Slack workspace team. Files that match the patterns are included in the index. Files that don't match the patterns are excluded from the index. If a file matches both an inclusion and exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
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
    /// <p>A list of regular expression patterns to exclude certain attached files in your Slack workspace team. Files that match the patterns are excluded from the index. Files that don’t match the patterns are included in the index. If a file matches both an inclusion and exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    pub fn exclusion_patterns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.exclusion_patterns.unwrap_or_default();
        v.push(input.into());
        self.exclusion_patterns = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of regular expression patterns to exclude certain attached files in your Slack workspace team. Files that match the patterns are excluded from the index. Files that don’t match the patterns are included in the index. If a file matches both an inclusion and exclusion pattern, the exclusion pattern takes precedence and the file isn't included in the index.</p>
    pub fn set_exclusion_patterns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.exclusion_patterns = input;
        self
    }
    /// Appends an item to `field_mappings`.
    ///
    /// To override the contents of this collection use [`set_field_mappings`](Self::set_field_mappings).
    ///
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map Slack data source attributes or field names to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Slack fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html">Mapping data source fields</a>. The Slack data source field names must exist in your Slack custom metadata.</p>
    pub fn field_mappings(mut self, input: crate::types::DataSourceToIndexFieldMapping) -> Self {
        let mut v = self.field_mappings.unwrap_or_default();
        v.push(input);
        self.field_mappings = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>DataSourceToIndexFieldMapping</code> objects that map Slack data source attributes or field names to Amazon Kendra index field names. To create custom fields, use the <code>UpdateIndex</code> API before you map to Slack fields. For more information, see <a href="https://docs.aws.amazon.com/kendra/latest/dg/field-mapping.html">Mapping data source fields</a>. The Slack data source field names must exist in your Slack custom metadata.</p>
    pub fn set_field_mappings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DataSourceToIndexFieldMapping>>,
    ) -> Self {
        self.field_mappings = input;
        self
    }
    /// Consumes the builder and constructs a [`SlackConfiguration`](crate::types::SlackConfiguration).
    pub fn build(self) -> crate::types::SlackConfiguration {
        crate::types::SlackConfiguration {
            team_id: self.team_id,
            secret_arn: self.secret_arn,
            vpc_configuration: self.vpc_configuration,
            slack_entity_list: self.slack_entity_list,
            use_change_log: self.use_change_log.unwrap_or_default(),
            crawl_bot_message: self.crawl_bot_message.unwrap_or_default(),
            exclude_archived: self.exclude_archived.unwrap_or_default(),
            since_crawl_date: self.since_crawl_date,
            look_back_period: self.look_back_period,
            private_channel_filter: self.private_channel_filter,
            public_channel_filter: self.public_channel_filter,
            inclusion_patterns: self.inclusion_patterns,
            exclusion_patterns: self.exclusion_patterns,
            field_mappings: self.field_mappings,
        }
    }
}
