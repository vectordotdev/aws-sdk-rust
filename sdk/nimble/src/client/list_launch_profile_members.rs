// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListLaunchProfileMembers`](crate::operation::list_launch_profile_members::builders::ListLaunchProfileMembersFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_launch_profile_members::builders::ListLaunchProfileMembersFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`launch_profile_id(impl ::std::convert::Into<String>)`](crate::operation::list_launch_profile_members::builders::ListLaunchProfileMembersFluentBuilder::launch_profile_id) / [`set_launch_profile_id(Option<String>)`](crate::operation::list_launch_profile_members::builders::ListLaunchProfileMembersFluentBuilder::set_launch_profile_id): <p>The ID of the launch profile used to control access from the streaming session.</p>
    ///   - [`max_results(i32)`](crate::operation::list_launch_profile_members::builders::ListLaunchProfileMembersFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_launch_profile_members::builders::ListLaunchProfileMembersFluentBuilder::set_max_results): <p>The max number of results to return in the response.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_launch_profile_members::builders::ListLaunchProfileMembersFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_launch_profile_members::builders::ListLaunchProfileMembersFluentBuilder::set_next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`studio_id(impl ::std::convert::Into<String>)`](crate::operation::list_launch_profile_members::builders::ListLaunchProfileMembersFluentBuilder::studio_id) / [`set_studio_id(Option<String>)`](crate::operation::list_launch_profile_members::builders::ListLaunchProfileMembersFluentBuilder::set_studio_id): <p>The studio ID. </p>
    /// - On success, responds with [`ListLaunchProfileMembersOutput`](crate::operation::list_launch_profile_members::ListLaunchProfileMembersOutput) with field(s):
    ///   - [`members(Option<Vec<LaunchProfileMembership>>)`](crate::operation::list_launch_profile_members::ListLaunchProfileMembersOutput::members): <p>A list of members.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_launch_profile_members::ListLaunchProfileMembersOutput::next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    /// - On failure, responds with [`SdkError<ListLaunchProfileMembersError>`](crate::operation::list_launch_profile_members::ListLaunchProfileMembersError)
    pub fn list_launch_profile_members(&self) -> crate::operation::list_launch_profile_members::builders::ListLaunchProfileMembersFluentBuilder{
        crate::operation::list_launch_profile_members::builders::ListLaunchProfileMembersFluentBuilder::new(self.handle.clone())
    }
}
