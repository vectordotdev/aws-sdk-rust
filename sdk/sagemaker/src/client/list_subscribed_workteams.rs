// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSubscribedWorkteams`](crate::operation::list_subscribed_workteams::builders::ListSubscribedWorkteamsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_subscribed_workteams::builders::ListSubscribedWorkteamsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`name_contains(impl ::std::convert::Into<String>)`](crate::operation::list_subscribed_workteams::builders::ListSubscribedWorkteamsFluentBuilder::name_contains) / [`set_name_contains(Option<String>)`](crate::operation::list_subscribed_workteams::builders::ListSubscribedWorkteamsFluentBuilder::set_name_contains): <p>A string in the work team name. This filter returns only work teams whose name contains the specified string.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_subscribed_workteams::builders::ListSubscribedWorkteamsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_subscribed_workteams::builders::ListSubscribedWorkteamsFluentBuilder::set_next_token): <p>If the result of the previous <code>ListSubscribedWorkteams</code> request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of labeling jobs, use the token in the next request.</p>
    ///   - [`max_results(i32)`](crate::operation::list_subscribed_workteams::builders::ListSubscribedWorkteamsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_subscribed_workteams::builders::ListSubscribedWorkteamsFluentBuilder::set_max_results): <p>The maximum number of work teams to return in each page of the response.</p>
    /// - On success, responds with [`ListSubscribedWorkteamsOutput`](crate::operation::list_subscribed_workteams::ListSubscribedWorkteamsOutput) with field(s):
    ///   - [`subscribed_workteams(Option<Vec<SubscribedWorkteam>>)`](crate::operation::list_subscribed_workteams::ListSubscribedWorkteamsOutput::subscribed_workteams): <p>An array of <code>Workteam</code> objects, each describing a work team.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_subscribed_workteams::ListSubscribedWorkteamsOutput::next_token): <p>If the response is truncated, Amazon SageMaker returns this token. To retrieve the next set of work teams, use it in the subsequent request.</p>
    /// - On failure, responds with [`SdkError<ListSubscribedWorkteamsError>`](crate::operation::list_subscribed_workteams::ListSubscribedWorkteamsError)
    pub fn list_subscribed_workteams(
        &self,
    ) -> crate::operation::list_subscribed_workteams::builders::ListSubscribedWorkteamsFluentBuilder
    {
        crate::operation::list_subscribed_workteams::builders::ListSubscribedWorkteamsFluentBuilder::new(self.handle.clone())
    }
}
