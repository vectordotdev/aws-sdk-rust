// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Meeting room settings of a room profile.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MeetingRoomConfiguration {
    /// <p>Whether room utilization metrics are enabled or not.</p>
    #[doc(hidden)]
    pub room_utilization_metrics_enabled: ::std::option::Option<bool>,
    /// <p>Settings for the end of meeting reminder feature that are applied to a room profile. The end of meeting reminder enables Alexa to remind users when a meeting is ending. </p>
    #[doc(hidden)]
    pub end_of_meeting_reminder: ::std::option::Option<crate::types::EndOfMeetingReminder>,
    /// <p>Settings to automatically book the room if available for a configured duration when joining a meeting with Alexa. </p>
    #[doc(hidden)]
    pub instant_booking: ::std::option::Option<crate::types::InstantBooking>,
    /// <p>Settings for requiring a check in when a room is reserved. Alexa can cancel a room reservation if it's not checked into. This makes the room available for others. Users can check in by joining the meeting with Alexa or an AVS device, or by saying “Alexa, check in.” </p>
    #[doc(hidden)]
    pub require_check_in: ::std::option::Option<crate::types::RequireCheckIn>,
}
impl MeetingRoomConfiguration {
    /// <p>Whether room utilization metrics are enabled or not.</p>
    pub fn room_utilization_metrics_enabled(&self) -> ::std::option::Option<bool> {
        self.room_utilization_metrics_enabled
    }
    /// <p>Settings for the end of meeting reminder feature that are applied to a room profile. The end of meeting reminder enables Alexa to remind users when a meeting is ending. </p>
    pub fn end_of_meeting_reminder(
        &self,
    ) -> ::std::option::Option<&crate::types::EndOfMeetingReminder> {
        self.end_of_meeting_reminder.as_ref()
    }
    /// <p>Settings to automatically book the room if available for a configured duration when joining a meeting with Alexa. </p>
    pub fn instant_booking(&self) -> ::std::option::Option<&crate::types::InstantBooking> {
        self.instant_booking.as_ref()
    }
    /// <p>Settings for requiring a check in when a room is reserved. Alexa can cancel a room reservation if it's not checked into. This makes the room available for others. Users can check in by joining the meeting with Alexa or an AVS device, or by saying “Alexa, check in.” </p>
    pub fn require_check_in(&self) -> ::std::option::Option<&crate::types::RequireCheckIn> {
        self.require_check_in.as_ref()
    }
}
impl MeetingRoomConfiguration {
    /// Creates a new builder-style object to manufacture [`MeetingRoomConfiguration`](crate::types::MeetingRoomConfiguration).
    pub fn builder() -> crate::types::builders::MeetingRoomConfigurationBuilder {
        crate::types::builders::MeetingRoomConfigurationBuilder::default()
    }
}

/// A builder for [`MeetingRoomConfiguration`](crate::types::MeetingRoomConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MeetingRoomConfigurationBuilder {
    pub(crate) room_utilization_metrics_enabled: ::std::option::Option<bool>,
    pub(crate) end_of_meeting_reminder: ::std::option::Option<crate::types::EndOfMeetingReminder>,
    pub(crate) instant_booking: ::std::option::Option<crate::types::InstantBooking>,
    pub(crate) require_check_in: ::std::option::Option<crate::types::RequireCheckIn>,
}
impl MeetingRoomConfigurationBuilder {
    /// <p>Whether room utilization metrics are enabled or not.</p>
    pub fn room_utilization_metrics_enabled(mut self, input: bool) -> Self {
        self.room_utilization_metrics_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether room utilization metrics are enabled or not.</p>
    pub fn set_room_utilization_metrics_enabled(
        mut self,
        input: ::std::option::Option<bool>,
    ) -> Self {
        self.room_utilization_metrics_enabled = input;
        self
    }
    /// <p>Settings for the end of meeting reminder feature that are applied to a room profile. The end of meeting reminder enables Alexa to remind users when a meeting is ending. </p>
    pub fn end_of_meeting_reminder(mut self, input: crate::types::EndOfMeetingReminder) -> Self {
        self.end_of_meeting_reminder = ::std::option::Option::Some(input);
        self
    }
    /// <p>Settings for the end of meeting reminder feature that are applied to a room profile. The end of meeting reminder enables Alexa to remind users when a meeting is ending. </p>
    pub fn set_end_of_meeting_reminder(
        mut self,
        input: ::std::option::Option<crate::types::EndOfMeetingReminder>,
    ) -> Self {
        self.end_of_meeting_reminder = input;
        self
    }
    /// <p>Settings to automatically book the room if available for a configured duration when joining a meeting with Alexa. </p>
    pub fn instant_booking(mut self, input: crate::types::InstantBooking) -> Self {
        self.instant_booking = ::std::option::Option::Some(input);
        self
    }
    /// <p>Settings to automatically book the room if available for a configured duration when joining a meeting with Alexa. </p>
    pub fn set_instant_booking(
        mut self,
        input: ::std::option::Option<crate::types::InstantBooking>,
    ) -> Self {
        self.instant_booking = input;
        self
    }
    /// <p>Settings for requiring a check in when a room is reserved. Alexa can cancel a room reservation if it's not checked into. This makes the room available for others. Users can check in by joining the meeting with Alexa or an AVS device, or by saying “Alexa, check in.” </p>
    pub fn require_check_in(mut self, input: crate::types::RequireCheckIn) -> Self {
        self.require_check_in = ::std::option::Option::Some(input);
        self
    }
    /// <p>Settings for requiring a check in when a room is reserved. Alexa can cancel a room reservation if it's not checked into. This makes the room available for others. Users can check in by joining the meeting with Alexa or an AVS device, or by saying “Alexa, check in.” </p>
    pub fn set_require_check_in(
        mut self,
        input: ::std::option::Option<crate::types::RequireCheckIn>,
    ) -> Self {
        self.require_check_in = input;
        self
    }
    /// Consumes the builder and constructs a [`MeetingRoomConfiguration`](crate::types::MeetingRoomConfiguration).
    pub fn build(self) -> crate::types::MeetingRoomConfiguration {
        crate::types::MeetingRoomConfiguration {
            room_utilization_metrics_enabled: self.room_utilization_metrics_enabled,
            end_of_meeting_reminder: self.end_of_meeting_reminder,
            instant_booking: self.instant_booking,
            require_check_in: self.require_check_in,
        }
    }
}
