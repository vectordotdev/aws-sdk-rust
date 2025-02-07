// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ReserveContact`](crate::operation::reserve_contact::builders::ReserveContactFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`mission_profile_arn(impl ::std::convert::Into<String>)`](crate::operation::reserve_contact::builders::ReserveContactFluentBuilder::mission_profile_arn) / [`set_mission_profile_arn(Option<String>)`](crate::operation::reserve_contact::builders::ReserveContactFluentBuilder::set_mission_profile_arn): <p>ARN of a mission profile.</p>
    ///   - [`satellite_arn(impl ::std::convert::Into<String>)`](crate::operation::reserve_contact::builders::ReserveContactFluentBuilder::satellite_arn) / [`set_satellite_arn(Option<String>)`](crate::operation::reserve_contact::builders::ReserveContactFluentBuilder::set_satellite_arn): <p>ARN of a satellite</p>
    ///   - [`start_time(DateTime)`](crate::operation::reserve_contact::builders::ReserveContactFluentBuilder::start_time) / [`set_start_time(Option<DateTime>)`](crate::operation::reserve_contact::builders::ReserveContactFluentBuilder::set_start_time): <p>Start time of a contact in UTC.</p>
    ///   - [`end_time(DateTime)`](crate::operation::reserve_contact::builders::ReserveContactFluentBuilder::end_time) / [`set_end_time(Option<DateTime>)`](crate::operation::reserve_contact::builders::ReserveContactFluentBuilder::set_end_time): <p>End time of a contact in UTC.</p>
    ///   - [`ground_station(impl ::std::convert::Into<String>)`](crate::operation::reserve_contact::builders::ReserveContactFluentBuilder::ground_station) / [`set_ground_station(Option<String>)`](crate::operation::reserve_contact::builders::ReserveContactFluentBuilder::set_ground_station): <p>Name of a ground station.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::operation::reserve_contact::builders::ReserveContactFluentBuilder::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::operation::reserve_contact::builders::ReserveContactFluentBuilder::set_tags): <p>Tags assigned to a contact.</p>
    /// - On success, responds with [`ReserveContactOutput`](crate::operation::reserve_contact::ReserveContactOutput) with field(s):
    ///   - [`contact_id(Option<String>)`](crate::operation::reserve_contact::ReserveContactOutput::contact_id): <p>UUID of a contact.</p>
    /// - On failure, responds with [`SdkError<ReserveContactError>`](crate::operation::reserve_contact::ReserveContactError)
    pub fn reserve_contact(
        &self,
    ) -> crate::operation::reserve_contact::builders::ReserveContactFluentBuilder {
        crate::operation::reserve_contact::builders::ReserveContactFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
