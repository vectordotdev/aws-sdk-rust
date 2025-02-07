// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The output configuration for this channel.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RequestOutputItem {
    /// <p>DASH manifest configuration parameters.</p>
    #[doc(hidden)]
    pub dash_playlist_settings: ::std::option::Option<crate::types::DashPlaylistSettings>,
    /// <p>HLS playlist configuration parameters.</p>
    #[doc(hidden)]
    pub hls_playlist_settings: ::std::option::Option<crate::types::HlsPlaylistSettings>,
    /// <p>The name of the manifest for the channel. The name appears in the <code>PlaybackUrl</code>.</p>
    #[doc(hidden)]
    pub manifest_name: ::std::option::Option<::std::string::String>,
    /// <p>A string used to match which <code>HttpPackageConfiguration</code> is used for each <code>VodSource</code>.</p>
    #[doc(hidden)]
    pub source_group: ::std::option::Option<::std::string::String>,
}
impl RequestOutputItem {
    /// <p>DASH manifest configuration parameters.</p>
    pub fn dash_playlist_settings(
        &self,
    ) -> ::std::option::Option<&crate::types::DashPlaylistSettings> {
        self.dash_playlist_settings.as_ref()
    }
    /// <p>HLS playlist configuration parameters.</p>
    pub fn hls_playlist_settings(
        &self,
    ) -> ::std::option::Option<&crate::types::HlsPlaylistSettings> {
        self.hls_playlist_settings.as_ref()
    }
    /// <p>The name of the manifest for the channel. The name appears in the <code>PlaybackUrl</code>.</p>
    pub fn manifest_name(&self) -> ::std::option::Option<&str> {
        self.manifest_name.as_deref()
    }
    /// <p>A string used to match which <code>HttpPackageConfiguration</code> is used for each <code>VodSource</code>.</p>
    pub fn source_group(&self) -> ::std::option::Option<&str> {
        self.source_group.as_deref()
    }
}
impl RequestOutputItem {
    /// Creates a new builder-style object to manufacture [`RequestOutputItem`](crate::types::RequestOutputItem).
    pub fn builder() -> crate::types::builders::RequestOutputItemBuilder {
        crate::types::builders::RequestOutputItemBuilder::default()
    }
}

/// A builder for [`RequestOutputItem`](crate::types::RequestOutputItem).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RequestOutputItemBuilder {
    pub(crate) dash_playlist_settings: ::std::option::Option<crate::types::DashPlaylistSettings>,
    pub(crate) hls_playlist_settings: ::std::option::Option<crate::types::HlsPlaylistSettings>,
    pub(crate) manifest_name: ::std::option::Option<::std::string::String>,
    pub(crate) source_group: ::std::option::Option<::std::string::String>,
}
impl RequestOutputItemBuilder {
    /// <p>DASH manifest configuration parameters.</p>
    pub fn dash_playlist_settings(mut self, input: crate::types::DashPlaylistSettings) -> Self {
        self.dash_playlist_settings = ::std::option::Option::Some(input);
        self
    }
    /// <p>DASH manifest configuration parameters.</p>
    pub fn set_dash_playlist_settings(
        mut self,
        input: ::std::option::Option<crate::types::DashPlaylistSettings>,
    ) -> Self {
        self.dash_playlist_settings = input;
        self
    }
    /// <p>HLS playlist configuration parameters.</p>
    pub fn hls_playlist_settings(mut self, input: crate::types::HlsPlaylistSettings) -> Self {
        self.hls_playlist_settings = ::std::option::Option::Some(input);
        self
    }
    /// <p>HLS playlist configuration parameters.</p>
    pub fn set_hls_playlist_settings(
        mut self,
        input: ::std::option::Option<crate::types::HlsPlaylistSettings>,
    ) -> Self {
        self.hls_playlist_settings = input;
        self
    }
    /// <p>The name of the manifest for the channel. The name appears in the <code>PlaybackUrl</code>.</p>
    pub fn manifest_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.manifest_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the manifest for the channel. The name appears in the <code>PlaybackUrl</code>.</p>
    pub fn set_manifest_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.manifest_name = input;
        self
    }
    /// <p>A string used to match which <code>HttpPackageConfiguration</code> is used for each <code>VodSource</code>.</p>
    pub fn source_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_group = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string used to match which <code>HttpPackageConfiguration</code> is used for each <code>VodSource</code>.</p>
    pub fn set_source_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_group = input;
        self
    }
    /// Consumes the builder and constructs a [`RequestOutputItem`](crate::types::RequestOutputItem).
    pub fn build(self) -> crate::types::RequestOutputItem {
        crate::types::RequestOutputItem {
            dash_playlist_settings: self.dash_playlist_settings,
            hls_playlist_settings: self.hls_playlist_settings,
            manifest_name: self.manifest_name,
            source_group: self.source_group,
        }
    }
}
