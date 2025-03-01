// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListLogSubscriptions`](crate::operation::list_log_subscriptions::builders::ListLogSubscriptionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_log_subscriptions::builders::ListLogSubscriptionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl ::std::convert::Into<String>)`](crate::operation::list_log_subscriptions::builders::ListLogSubscriptionsFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::list_log_subscriptions::builders::ListLogSubscriptionsFluentBuilder::set_directory_id): <p>If a <i>DirectoryID</i> is provided, lists only the log subscription associated with that directory. If no <i>DirectoryId</i> is provided, lists all log subscriptions associated with your Amazon Web Services account. If there are no log subscriptions for the Amazon Web Services account or the directory, an empty list will be returned.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_log_subscriptions::builders::ListLogSubscriptionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_log_subscriptions::builders::ListLogSubscriptionsFluentBuilder::set_next_token): <p>The token for the next set of items to return.</p>
    ///   - [`limit(i32)`](crate::operation::list_log_subscriptions::builders::ListLogSubscriptionsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_log_subscriptions::builders::ListLogSubscriptionsFluentBuilder::set_limit): <p>The maximum number of items returned.</p>
    /// - On success, responds with [`ListLogSubscriptionsOutput`](crate::operation::list_log_subscriptions::ListLogSubscriptionsOutput) with field(s):
    ///   - [`log_subscriptions(Option<Vec<LogSubscription>>)`](crate::operation::list_log_subscriptions::ListLogSubscriptionsOutput::log_subscriptions): <p>A list of active <code>LogSubscription</code> objects for calling the Amazon Web Services account.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_log_subscriptions::ListLogSubscriptionsOutput::next_token): <p>The token for the next set of items to return.</p>
    /// - On failure, responds with [`SdkError<ListLogSubscriptionsError>`](crate::operation::list_log_subscriptions::ListLogSubscriptionsError)
    pub fn list_log_subscriptions(
        &self,
    ) -> crate::operation::list_log_subscriptions::builders::ListLogSubscriptionsFluentBuilder {
        crate::operation::list_log_subscriptions::builders::ListLogSubscriptionsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
