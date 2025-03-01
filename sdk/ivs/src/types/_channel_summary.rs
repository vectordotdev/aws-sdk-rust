// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Summary information about a channel.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ChannelSummary {
    /// <p>Channel ARN.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>Channel name.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>Channel latency mode. Use <code>NORMAL</code> to broadcast and deliver live video up to Full HD. Use <code>LOW</code> for near-real-time interaction with viewers. Default: <code>LOW</code>. (Note: In the Amazon IVS console, <code>LOW</code> and <code>NORMAL</code> correspond to Ultra-low and Standard, respectively.)</p>
    #[doc(hidden)]
    pub latency_mode: ::std::option::Option<crate::types::ChannelLatencyMode>,
    /// <p>Whether the channel is private (enabled for playback authorization). Default: <code>false</code>.</p>
    #[doc(hidden)]
    pub authorized: bool,
    /// <p>Recording-configuration ARN. A value other than an empty string indicates that recording is enabled. Default: "" (empty string, recording is disabled).</p>
    #[doc(hidden)]
    pub recording_configuration_arn: ::std::option::Option<::std::string::String>,
    /// <p>Tags attached to the resource. Array of 1-50 maps, each of the form <code>string:string (key:value)</code>. See <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a> for more information, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS has no service-specific constraints beyond what is documented there.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    /// <p>Whether the channel allows insecure RTMP ingest. Default: <code>false</code>.</p>
    #[doc(hidden)]
    pub insecure_ingest: bool,
}
impl ChannelSummary {
    /// <p>Channel ARN.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>Channel name.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Channel latency mode. Use <code>NORMAL</code> to broadcast and deliver live video up to Full HD. Use <code>LOW</code> for near-real-time interaction with viewers. Default: <code>LOW</code>. (Note: In the Amazon IVS console, <code>LOW</code> and <code>NORMAL</code> correspond to Ultra-low and Standard, respectively.)</p>
    pub fn latency_mode(&self) -> ::std::option::Option<&crate::types::ChannelLatencyMode> {
        self.latency_mode.as_ref()
    }
    /// <p>Whether the channel is private (enabled for playback authorization). Default: <code>false</code>.</p>
    pub fn authorized(&self) -> bool {
        self.authorized
    }
    /// <p>Recording-configuration ARN. A value other than an empty string indicates that recording is enabled. Default: "" (empty string, recording is disabled).</p>
    pub fn recording_configuration_arn(&self) -> ::std::option::Option<&str> {
        self.recording_configuration_arn.as_deref()
    }
    /// <p>Tags attached to the resource. Array of 1-50 maps, each of the form <code>string:string (key:value)</code>. See <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a> for more information, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS has no service-specific constraints beyond what is documented there.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
    /// <p>Whether the channel allows insecure RTMP ingest. Default: <code>false</code>.</p>
    pub fn insecure_ingest(&self) -> bool {
        self.insecure_ingest
    }
}
impl ChannelSummary {
    /// Creates a new builder-style object to manufacture [`ChannelSummary`](crate::types::ChannelSummary).
    pub fn builder() -> crate::types::builders::ChannelSummaryBuilder {
        crate::types::builders::ChannelSummaryBuilder::default()
    }
}

/// A builder for [`ChannelSummary`](crate::types::ChannelSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ChannelSummaryBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) latency_mode: ::std::option::Option<crate::types::ChannelLatencyMode>,
    pub(crate) authorized: ::std::option::Option<bool>,
    pub(crate) recording_configuration_arn: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) insecure_ingest: ::std::option::Option<bool>,
}
impl ChannelSummaryBuilder {
    /// <p>Channel ARN.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Channel ARN.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>Channel name.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Channel name.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Channel latency mode. Use <code>NORMAL</code> to broadcast and deliver live video up to Full HD. Use <code>LOW</code> for near-real-time interaction with viewers. Default: <code>LOW</code>. (Note: In the Amazon IVS console, <code>LOW</code> and <code>NORMAL</code> correspond to Ultra-low and Standard, respectively.)</p>
    pub fn latency_mode(mut self, input: crate::types::ChannelLatencyMode) -> Self {
        self.latency_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>Channel latency mode. Use <code>NORMAL</code> to broadcast and deliver live video up to Full HD. Use <code>LOW</code> for near-real-time interaction with viewers. Default: <code>LOW</code>. (Note: In the Amazon IVS console, <code>LOW</code> and <code>NORMAL</code> correspond to Ultra-low and Standard, respectively.)</p>
    pub fn set_latency_mode(
        mut self,
        input: ::std::option::Option<crate::types::ChannelLatencyMode>,
    ) -> Self {
        self.latency_mode = input;
        self
    }
    /// <p>Whether the channel is private (enabled for playback authorization). Default: <code>false</code>.</p>
    pub fn authorized(mut self, input: bool) -> Self {
        self.authorized = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether the channel is private (enabled for playback authorization). Default: <code>false</code>.</p>
    pub fn set_authorized(mut self, input: ::std::option::Option<bool>) -> Self {
        self.authorized = input;
        self
    }
    /// <p>Recording-configuration ARN. A value other than an empty string indicates that recording is enabled. Default: "" (empty string, recording is disabled).</p>
    pub fn recording_configuration_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.recording_configuration_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Recording-configuration ARN. A value other than an empty string indicates that recording is enabled. Default: "" (empty string, recording is disabled).</p>
    pub fn set_recording_configuration_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.recording_configuration_arn = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags attached to the resource. Array of 1-50 maps, each of the form <code>string:string (key:value)</code>. See <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a> for more information, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS has no service-specific constraints beyond what is documented there.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Tags attached to the resource. Array of 1-50 maps, each of the form <code>string:string (key:value)</code>. See <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services Resources</a> for more information, including restrictions that apply to tags and "Tag naming limits and requirements"; Amazon IVS has no service-specific constraints beyond what is documented there.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>Whether the channel allows insecure RTMP ingest. Default: <code>false</code>.</p>
    pub fn insecure_ingest(mut self, input: bool) -> Self {
        self.insecure_ingest = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether the channel allows insecure RTMP ingest. Default: <code>false</code>.</p>
    pub fn set_insecure_ingest(mut self, input: ::std::option::Option<bool>) -> Self {
        self.insecure_ingest = input;
        self
    }
    /// Consumes the builder and constructs a [`ChannelSummary`](crate::types::ChannelSummary).
    pub fn build(self) -> crate::types::ChannelSummary {
        crate::types::ChannelSummary {
            arn: self.arn,
            name: self.name,
            latency_mode: self.latency_mode,
            authorized: self.authorized.unwrap_or_default(),
            recording_configuration_arn: self.recording_configuration_arn,
            tags: self.tags,
            insecure_ingest: self.insecure_ingest.unwrap_or_default(),
        }
    }
}
