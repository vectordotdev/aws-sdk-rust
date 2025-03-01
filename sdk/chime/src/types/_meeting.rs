// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A meeting created using the Amazon Chime SDK.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct Meeting {
    /// <p>The Amazon Chime SDK meeting ID.</p>
    #[doc(hidden)]
    pub meeting_id: ::std::option::Option<::std::string::String>,
    /// <p>The external meeting ID.</p>
    #[doc(hidden)]
    pub external_meeting_id: ::std::option::Option<::std::string::String>,
    /// <p>The media placement for the meeting.</p>
    #[doc(hidden)]
    pub media_placement: ::std::option::Option<crate::types::MediaPlacement>,
    /// <p>The Region in which you create the meeting. Available values: <code>af-south-1</code>, <code>ap-northeast-1</code>, <code>ap-northeast-2</code>, <code>ap-south-1</code>, <code>ap-southeast-1</code>, <code>ap-southeast-2</code>, <code>ca-central-1</code>, <code>eu-central-1</code>, <code>eu-north-1</code>, <code>eu-south-1</code>, <code>eu-west-1</code>, <code>eu-west-2</code>, <code>eu-west-3</code>, <code>sa-east-1</code>, <code>us-east-1</code>, <code>us-east-2</code>, <code>us-west-1</code>, <code>us-west-2</code>.</p>
    #[doc(hidden)]
    pub media_region: ::std::option::Option<::std::string::String>,
}
impl Meeting {
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn meeting_id(&self) -> ::std::option::Option<&str> {
        self.meeting_id.as_deref()
    }
    /// <p>The external meeting ID.</p>
    pub fn external_meeting_id(&self) -> ::std::option::Option<&str> {
        self.external_meeting_id.as_deref()
    }
    /// <p>The media placement for the meeting.</p>
    pub fn media_placement(&self) -> ::std::option::Option<&crate::types::MediaPlacement> {
        self.media_placement.as_ref()
    }
    /// <p>The Region in which you create the meeting. Available values: <code>af-south-1</code>, <code>ap-northeast-1</code>, <code>ap-northeast-2</code>, <code>ap-south-1</code>, <code>ap-southeast-1</code>, <code>ap-southeast-2</code>, <code>ca-central-1</code>, <code>eu-central-1</code>, <code>eu-north-1</code>, <code>eu-south-1</code>, <code>eu-west-1</code>, <code>eu-west-2</code>, <code>eu-west-3</code>, <code>sa-east-1</code>, <code>us-east-1</code>, <code>us-east-2</code>, <code>us-west-1</code>, <code>us-west-2</code>.</p>
    pub fn media_region(&self) -> ::std::option::Option<&str> {
        self.media_region.as_deref()
    }
}
impl ::std::fmt::Debug for Meeting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("Meeting");
        formatter.field("meeting_id", &self.meeting_id);
        formatter.field("external_meeting_id", &"*** Sensitive Data Redacted ***");
        formatter.field("media_placement", &self.media_placement);
        formatter.field("media_region", &self.media_region);
        formatter.finish()
    }
}
impl Meeting {
    /// Creates a new builder-style object to manufacture [`Meeting`](crate::types::Meeting).
    pub fn builder() -> crate::types::builders::MeetingBuilder {
        crate::types::builders::MeetingBuilder::default()
    }
}

/// A builder for [`Meeting`](crate::types::Meeting).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct MeetingBuilder {
    pub(crate) meeting_id: ::std::option::Option<::std::string::String>,
    pub(crate) external_meeting_id: ::std::option::Option<::std::string::String>,
    pub(crate) media_placement: ::std::option::Option<crate::types::MediaPlacement>,
    pub(crate) media_region: ::std::option::Option<::std::string::String>,
}
impl MeetingBuilder {
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn meeting_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.meeting_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Chime SDK meeting ID.</p>
    pub fn set_meeting_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.meeting_id = input;
        self
    }
    /// <p>The external meeting ID.</p>
    pub fn external_meeting_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.external_meeting_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The external meeting ID.</p>
    pub fn set_external_meeting_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.external_meeting_id = input;
        self
    }
    /// <p>The media placement for the meeting.</p>
    pub fn media_placement(mut self, input: crate::types::MediaPlacement) -> Self {
        self.media_placement = ::std::option::Option::Some(input);
        self
    }
    /// <p>The media placement for the meeting.</p>
    pub fn set_media_placement(
        mut self,
        input: ::std::option::Option<crate::types::MediaPlacement>,
    ) -> Self {
        self.media_placement = input;
        self
    }
    /// <p>The Region in which you create the meeting. Available values: <code>af-south-1</code>, <code>ap-northeast-1</code>, <code>ap-northeast-2</code>, <code>ap-south-1</code>, <code>ap-southeast-1</code>, <code>ap-southeast-2</code>, <code>ca-central-1</code>, <code>eu-central-1</code>, <code>eu-north-1</code>, <code>eu-south-1</code>, <code>eu-west-1</code>, <code>eu-west-2</code>, <code>eu-west-3</code>, <code>sa-east-1</code>, <code>us-east-1</code>, <code>us-east-2</code>, <code>us-west-1</code>, <code>us-west-2</code>.</p>
    pub fn media_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.media_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Region in which you create the meeting. Available values: <code>af-south-1</code>, <code>ap-northeast-1</code>, <code>ap-northeast-2</code>, <code>ap-south-1</code>, <code>ap-southeast-1</code>, <code>ap-southeast-2</code>, <code>ca-central-1</code>, <code>eu-central-1</code>, <code>eu-north-1</code>, <code>eu-south-1</code>, <code>eu-west-1</code>, <code>eu-west-2</code>, <code>eu-west-3</code>, <code>sa-east-1</code>, <code>us-east-1</code>, <code>us-east-2</code>, <code>us-west-1</code>, <code>us-west-2</code>.</p>
    pub fn set_media_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.media_region = input;
        self
    }
    /// Consumes the builder and constructs a [`Meeting`](crate::types::Meeting).
    pub fn build(self) -> crate::types::Meeting {
        crate::types::Meeting {
            meeting_id: self.meeting_id,
            external_meeting_id: self.external_meeting_id,
            media_placement: self.media_placement,
            media_region: self.media_region,
        }
    }
}
impl ::std::fmt::Debug for MeetingBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("MeetingBuilder");
        formatter.field("meeting_id", &self.meeting_id);
        formatter.field("external_meeting_id", &"*** Sensitive Data Redacted ***");
        formatter.field("media_placement", &self.media_placement);
        formatter.field("media_region", &self.media_region);
        formatter.finish()
    }
}
