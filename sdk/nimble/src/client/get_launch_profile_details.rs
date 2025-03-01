// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetLaunchProfileDetails`](crate::operation::get_launch_profile_details::builders::GetLaunchProfileDetailsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`launch_profile_id(impl ::std::convert::Into<String>)`](crate::operation::get_launch_profile_details::builders::GetLaunchProfileDetailsFluentBuilder::launch_profile_id) / [`set_launch_profile_id(Option<String>)`](crate::operation::get_launch_profile_details::builders::GetLaunchProfileDetailsFluentBuilder::set_launch_profile_id): <p>The ID of the launch profile used to control access from the streaming session.</p>
    ///   - [`studio_id(impl ::std::convert::Into<String>)`](crate::operation::get_launch_profile_details::builders::GetLaunchProfileDetailsFluentBuilder::studio_id) / [`set_studio_id(Option<String>)`](crate::operation::get_launch_profile_details::builders::GetLaunchProfileDetailsFluentBuilder::set_studio_id): <p>The studio ID. </p>
    /// - On success, responds with [`GetLaunchProfileDetailsOutput`](crate::operation::get_launch_profile_details::GetLaunchProfileDetailsOutput) with field(s):
    ///   - [`launch_profile(Option<LaunchProfile>)`](crate::operation::get_launch_profile_details::GetLaunchProfileDetailsOutput::launch_profile): <p>The launch profile.</p>
    ///   - [`streaming_images(Option<Vec<StreamingImage>>)`](crate::operation::get_launch_profile_details::GetLaunchProfileDetailsOutput::streaming_images): <p>A collection of streaming images.</p>
    ///   - [`studio_component_summaries(Option<Vec<StudioComponentSummary>>)`](crate::operation::get_launch_profile_details::GetLaunchProfileDetailsOutput::studio_component_summaries): <p>A collection of studio component summaries.</p>
    /// - On failure, responds with [`SdkError<GetLaunchProfileDetailsError>`](crate::operation::get_launch_profile_details::GetLaunchProfileDetailsError)
    pub fn get_launch_profile_details(
        &self,
    ) -> crate::operation::get_launch_profile_details::builders::GetLaunchProfileDetailsFluentBuilder
    {
        crate::operation::get_launch_profile_details::builders::GetLaunchProfileDetailsFluentBuilder::new(self.handle.clone())
    }
}
