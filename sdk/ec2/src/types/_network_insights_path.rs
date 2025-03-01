// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a path.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NetworkInsightsPath {
    /// <p>The ID of the path.</p>
    #[doc(hidden)]
    pub network_insights_path_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the path.</p>
    #[doc(hidden)]
    pub network_insights_path_arn: ::std::option::Option<::std::string::String>,
    /// <p>The time stamp when the path was created.</p>
    #[doc(hidden)]
    pub created_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The ID of the source.</p>
    #[doc(hidden)]
    pub source: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the destination.</p>
    #[doc(hidden)]
    pub destination: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the source.</p>
    #[doc(hidden)]
    pub source_arn: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the destination.</p>
    #[doc(hidden)]
    pub destination_arn: ::std::option::Option<::std::string::String>,
    /// <p>The IP address of the source.</p>
    #[doc(hidden)]
    pub source_ip: ::std::option::Option<::std::string::String>,
    /// <p>The IP address of the destination.</p>
    #[doc(hidden)]
    pub destination_ip: ::std::option::Option<::std::string::String>,
    /// <p>The protocol.</p>
    #[doc(hidden)]
    pub protocol: ::std::option::Option<crate::types::Protocol>,
    /// <p>The destination port.</p>
    #[doc(hidden)]
    pub destination_port: ::std::option::Option<i32>,
    /// <p>The tags associated with the path.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>Scopes the analysis to network paths that match specific filters at the source.</p>
    #[doc(hidden)]
    pub filter_at_source: ::std::option::Option<crate::types::PathFilter>,
    /// <p>Scopes the analysis to network paths that match specific filters at the destination.</p>
    #[doc(hidden)]
    pub filter_at_destination: ::std::option::Option<crate::types::PathFilter>,
}
impl NetworkInsightsPath {
    /// <p>The ID of the path.</p>
    pub fn network_insights_path_id(&self) -> ::std::option::Option<&str> {
        self.network_insights_path_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the path.</p>
    pub fn network_insights_path_arn(&self) -> ::std::option::Option<&str> {
        self.network_insights_path_arn.as_deref()
    }
    /// <p>The time stamp when the path was created.</p>
    pub fn created_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_date.as_ref()
    }
    /// <p>The ID of the source.</p>
    pub fn source(&self) -> ::std::option::Option<&str> {
        self.source.as_deref()
    }
    /// <p>The ID of the destination.</p>
    pub fn destination(&self) -> ::std::option::Option<&str> {
        self.destination.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the source.</p>
    pub fn source_arn(&self) -> ::std::option::Option<&str> {
        self.source_arn.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the destination.</p>
    pub fn destination_arn(&self) -> ::std::option::Option<&str> {
        self.destination_arn.as_deref()
    }
    /// <p>The IP address of the source.</p>
    pub fn source_ip(&self) -> ::std::option::Option<&str> {
        self.source_ip.as_deref()
    }
    /// <p>The IP address of the destination.</p>
    pub fn destination_ip(&self) -> ::std::option::Option<&str> {
        self.destination_ip.as_deref()
    }
    /// <p>The protocol.</p>
    pub fn protocol(&self) -> ::std::option::Option<&crate::types::Protocol> {
        self.protocol.as_ref()
    }
    /// <p>The destination port.</p>
    pub fn destination_port(&self) -> ::std::option::Option<i32> {
        self.destination_port
    }
    /// <p>The tags associated with the path.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the source.</p>
    pub fn filter_at_source(&self) -> ::std::option::Option<&crate::types::PathFilter> {
        self.filter_at_source.as_ref()
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the destination.</p>
    pub fn filter_at_destination(&self) -> ::std::option::Option<&crate::types::PathFilter> {
        self.filter_at_destination.as_ref()
    }
}
impl NetworkInsightsPath {
    /// Creates a new builder-style object to manufacture [`NetworkInsightsPath`](crate::types::NetworkInsightsPath).
    pub fn builder() -> crate::types::builders::NetworkInsightsPathBuilder {
        crate::types::builders::NetworkInsightsPathBuilder::default()
    }
}

/// A builder for [`NetworkInsightsPath`](crate::types::NetworkInsightsPath).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct NetworkInsightsPathBuilder {
    pub(crate) network_insights_path_id: ::std::option::Option<::std::string::String>,
    pub(crate) network_insights_path_arn: ::std::option::Option<::std::string::String>,
    pub(crate) created_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) source: ::std::option::Option<::std::string::String>,
    pub(crate) destination: ::std::option::Option<::std::string::String>,
    pub(crate) source_arn: ::std::option::Option<::std::string::String>,
    pub(crate) destination_arn: ::std::option::Option<::std::string::String>,
    pub(crate) source_ip: ::std::option::Option<::std::string::String>,
    pub(crate) destination_ip: ::std::option::Option<::std::string::String>,
    pub(crate) protocol: ::std::option::Option<crate::types::Protocol>,
    pub(crate) destination_port: ::std::option::Option<i32>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) filter_at_source: ::std::option::Option<crate::types::PathFilter>,
    pub(crate) filter_at_destination: ::std::option::Option<crate::types::PathFilter>,
}
impl NetworkInsightsPathBuilder {
    /// <p>The ID of the path.</p>
    pub fn network_insights_path_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.network_insights_path_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the path.</p>
    pub fn set_network_insights_path_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.network_insights_path_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the path.</p>
    pub fn network_insights_path_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.network_insights_path_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the path.</p>
    pub fn set_network_insights_path_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.network_insights_path_arn = input;
        self
    }
    /// <p>The time stamp when the path was created.</p>
    pub fn created_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time stamp when the path was created.</p>
    pub fn set_created_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_date = input;
        self
    }
    /// <p>The ID of the source.</p>
    pub fn source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the source.</p>
    pub fn set_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source = input;
        self
    }
    /// <p>The ID of the destination.</p>
    pub fn destination(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the destination.</p>
    pub fn set_destination(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the source.</p>
    pub fn source_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the source.</p>
    pub fn set_source_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the destination.</p>
    pub fn destination_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.destination_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the destination.</p>
    pub fn set_destination_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.destination_arn = input;
        self
    }
    /// <p>The IP address of the source.</p>
    pub fn source_ip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_ip = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP address of the source.</p>
    pub fn set_source_ip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_ip = input;
        self
    }
    /// <p>The IP address of the destination.</p>
    pub fn destination_ip(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.destination_ip = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP address of the destination.</p>
    pub fn set_destination_ip(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.destination_ip = input;
        self
    }
    /// <p>The protocol.</p>
    pub fn protocol(mut self, input: crate::types::Protocol) -> Self {
        self.protocol = ::std::option::Option::Some(input);
        self
    }
    /// <p>The protocol.</p>
    pub fn set_protocol(mut self, input: ::std::option::Option<crate::types::Protocol>) -> Self {
        self.protocol = input;
        self
    }
    /// <p>The destination port.</p>
    pub fn destination_port(mut self, input: i32) -> Self {
        self.destination_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination port.</p>
    pub fn set_destination_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.destination_port = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags associated with the path.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags associated with the path.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the source.</p>
    pub fn filter_at_source(mut self, input: crate::types::PathFilter) -> Self {
        self.filter_at_source = ::std::option::Option::Some(input);
        self
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the source.</p>
    pub fn set_filter_at_source(
        mut self,
        input: ::std::option::Option<crate::types::PathFilter>,
    ) -> Self {
        self.filter_at_source = input;
        self
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the destination.</p>
    pub fn filter_at_destination(mut self, input: crate::types::PathFilter) -> Self {
        self.filter_at_destination = ::std::option::Option::Some(input);
        self
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the destination.</p>
    pub fn set_filter_at_destination(
        mut self,
        input: ::std::option::Option<crate::types::PathFilter>,
    ) -> Self {
        self.filter_at_destination = input;
        self
    }
    /// Consumes the builder and constructs a [`NetworkInsightsPath`](crate::types::NetworkInsightsPath).
    pub fn build(self) -> crate::types::NetworkInsightsPath {
        crate::types::NetworkInsightsPath {
            network_insights_path_id: self.network_insights_path_id,
            network_insights_path_arn: self.network_insights_path_arn,
            created_date: self.created_date,
            source: self.source,
            destination: self.destination,
            source_arn: self.source_arn,
            destination_arn: self.destination_arn,
            source_ip: self.source_ip,
            destination_ip: self.destination_ip,
            protocol: self.protocol,
            destination_port: self.destination_port,
            tags: self.tags,
            filter_at_source: self.filter_at_source,
            filter_at_destination: self.filter_at_destination,
        }
    }
}
