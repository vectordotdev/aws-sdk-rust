// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`PutEventsInput`](crate::input::PutEventsInput)
pub mod put_events_input {
    /// A builder for [`PutEventsInput`](crate::input::PutEventsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tracking_id: std::option::Option<std::string::String>,
        pub(crate) user_id: std::option::Option<std::string::String>,
        pub(crate) session_id: std::option::Option<std::string::String>,
        pub(crate) event_list: std::option::Option<std::vec::Vec<crate::model::Event>>,
    }
    impl Builder {
        /// <p>The tracking ID for the event.
        /// The ID is generated by a call to the
        /// <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CreateEventTracker.html">CreateEventTracker</a> API.</p>
        pub fn tracking_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.tracking_id = Some(input.into());
            self
        }
        /// <p>The tracking ID for the event.
        /// The ID is generated by a call to the
        /// <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CreateEventTracker.html">CreateEventTracker</a> API.</p>
        pub fn set_tracking_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.tracking_id = input;
            self
        }
        /// <p>The user associated with the event.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.user_id = Some(input.into());
            self
        }
        /// <p>The user associated with the event.</p>
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.user_id = input;
            self
        }
        /// <p>The session ID associated with the user's visit. Your application generates the sessionId when a user first visits your website or uses your application.
        /// Amazon Personalize uses the sessionId to associate events with the user before they log in. For more information, see
        /// <a href="https://docs.aws.amazon.com/personalize/latest/dg/recording-events.html">Recording Events</a>.</p>
        pub fn session_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.session_id = Some(input.into());
            self
        }
        /// <p>The session ID associated with the user's visit. Your application generates the sessionId when a user first visits your website or uses your application.
        /// Amazon Personalize uses the sessionId to associate events with the user before they log in. For more information, see
        /// <a href="https://docs.aws.amazon.com/personalize/latest/dg/recording-events.html">Recording Events</a>.</p>
        pub fn set_session_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.session_id = input;
            self
        }
        /// Appends an item to `event_list`.
        ///
        /// To override the contents of this collection use [`set_event_list`](Self::set_event_list).
        ///
        /// <p>A list of event data from the session.</p>
        pub fn event_list(mut self, input: impl Into<crate::model::Event>) -> Self {
            let mut v = self.event_list.unwrap_or_default();
            v.push(input.into());
            self.event_list = Some(v);
            self
        }
        /// <p>A list of event data from the session.</p>
        pub fn set_event_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Event>>,
        ) -> Self {
            self.event_list = input;
            self
        }
        /// Consumes the builder and constructs a [`PutEventsInput`](crate::input::PutEventsInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::PutEventsInput, aws_smithy_http::operation::BuildError>
        {
            Ok(crate::input::PutEventsInput {
                tracking_id: self.tracking_id,
                user_id: self.user_id,
                session_id: self.session_id,
                event_list: self.event_list,
            })
        }
    }
}
#[doc(hidden)]
pub type PutEventsInputOperationOutputAlias = crate::operation::PutEvents;
#[doc(hidden)]
pub type PutEventsInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl PutEventsInput {
    /// Consumes the builder and constructs an Operation<[`PutEvents`](crate::operation::PutEvents)>
    #[allow(clippy::let_and_return)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::PutEvents,
            aws_http::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::PutEventsInput,
            output: &mut String,
        ) -> Result<(), aws_smithy_http::operation::BuildError> {
            write!(output, "/events").expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::PutEventsInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::PutEventsInput,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            #[allow(unused_mut)]
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/json",
            );
            Ok(builder)
        }
        let properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = crate::operation_ser::serialize_operation_crate_operation_put_events(&self)
            .map_err(|err| {
                aws_smithy_http::operation::BuildError::SerializationError(err.into())
            })?;
        let request = Self::assemble(request, body);
        #[allow(unused_mut)]
        let mut request = aws_smithy_http::operation::Request::from_parts(
            request.map(aws_smithy_http::body::SdkBody::from),
            properties,
        );
        request
            .properties_mut()
            .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                crate::API_METADATA.clone(),
            ));
        #[allow(unused_mut)]
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op =
            aws_smithy_http::operation::Operation::new(request, crate::operation::PutEvents::new())
                .with_metadata(aws_smithy_http::operation::Metadata::new(
                    "PutEvents",
                    "personalizeevents",
                ));
        let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        builder: http::request::Builder,
        body: aws_smithy_http::body::SdkBody,
    ) -> http::request::Request<aws_smithy_http::body::SdkBody> {
        let mut builder = builder;
        if let Some(content_length) = body.content_length() {
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`PutEventsInput`](crate::input::PutEventsInput)
    pub fn builder() -> crate::input::put_events_input::Builder {
        crate::input::put_events_input::Builder::default()
    }
}

/// See [`PutItemsInput`](crate::input::PutItemsInput)
pub mod put_items_input {
    /// A builder for [`PutItemsInput`](crate::input::PutItemsInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) dataset_arn: std::option::Option<std::string::String>,
        pub(crate) items: std::option::Option<std::vec::Vec<crate::model::Item>>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the Items dataset you are adding the item or items to.</p>
        pub fn dataset_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.dataset_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the Items dataset you are adding the item or items to.</p>
        pub fn set_dataset_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.dataset_arn = input;
            self
        }
        /// Appends an item to `items`.
        ///
        /// To override the contents of this collection use [`set_items`](Self::set_items).
        ///
        /// <p>A list of item data.</p>
        pub fn items(mut self, input: impl Into<crate::model::Item>) -> Self {
            let mut v = self.items.unwrap_or_default();
            v.push(input.into());
            self.items = Some(v);
            self
        }
        /// <p>A list of item data.</p>
        pub fn set_items(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Item>>,
        ) -> Self {
            self.items = input;
            self
        }
        /// Consumes the builder and constructs a [`PutItemsInput`](crate::input::PutItemsInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::PutItemsInput, aws_smithy_http::operation::BuildError>
        {
            Ok(crate::input::PutItemsInput {
                dataset_arn: self.dataset_arn,
                items: self.items,
            })
        }
    }
}
#[doc(hidden)]
pub type PutItemsInputOperationOutputAlias = crate::operation::PutItems;
#[doc(hidden)]
pub type PutItemsInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl PutItemsInput {
    /// Consumes the builder and constructs an Operation<[`PutItems`](crate::operation::PutItems)>
    #[allow(clippy::let_and_return)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::PutItems,
            aws_http::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::PutItemsInput,
            output: &mut String,
        ) -> Result<(), aws_smithy_http::operation::BuildError> {
            write!(output, "/items").expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::PutItemsInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::PutItemsInput,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            #[allow(unused_mut)]
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/json",
            );
            Ok(builder)
        }
        let properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = crate::operation_ser::serialize_operation_crate_operation_put_items(&self)
            .map_err(|err| {
                aws_smithy_http::operation::BuildError::SerializationError(err.into())
            })?;
        let request = Self::assemble(request, body);
        #[allow(unused_mut)]
        let mut request = aws_smithy_http::operation::Request::from_parts(
            request.map(aws_smithy_http::body::SdkBody::from),
            properties,
        );
        request
            .properties_mut()
            .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                crate::API_METADATA.clone(),
            ));
        #[allow(unused_mut)]
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op =
            aws_smithy_http::operation::Operation::new(request, crate::operation::PutItems::new())
                .with_metadata(aws_smithy_http::operation::Metadata::new(
                    "PutItems",
                    "personalizeevents",
                ));
        let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        builder: http::request::Builder,
        body: aws_smithy_http::body::SdkBody,
    ) -> http::request::Request<aws_smithy_http::body::SdkBody> {
        let mut builder = builder;
        if let Some(content_length) = body.content_length() {
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`PutItemsInput`](crate::input::PutItemsInput)
    pub fn builder() -> crate::input::put_items_input::Builder {
        crate::input::put_items_input::Builder::default()
    }
}

/// See [`PutUsersInput`](crate::input::PutUsersInput)
pub mod put_users_input {
    /// A builder for [`PutUsersInput`](crate::input::PutUsersInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) dataset_arn: std::option::Option<std::string::String>,
        pub(crate) users: std::option::Option<std::vec::Vec<crate::model::User>>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the Users dataset you are adding the user or users to.</p>
        pub fn dataset_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.dataset_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the Users dataset you are adding the user or users to.</p>
        pub fn set_dataset_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.dataset_arn = input;
            self
        }
        /// Appends an item to `users`.
        ///
        /// To override the contents of this collection use [`set_users`](Self::set_users).
        ///
        /// <p>A list of user data.</p>
        pub fn users(mut self, input: impl Into<crate::model::User>) -> Self {
            let mut v = self.users.unwrap_or_default();
            v.push(input.into());
            self.users = Some(v);
            self
        }
        /// <p>A list of user data.</p>
        pub fn set_users(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::User>>,
        ) -> Self {
            self.users = input;
            self
        }
        /// Consumes the builder and constructs a [`PutUsersInput`](crate::input::PutUsersInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::PutUsersInput, aws_smithy_http::operation::BuildError>
        {
            Ok(crate::input::PutUsersInput {
                dataset_arn: self.dataset_arn,
                users: self.users,
            })
        }
    }
}
#[doc(hidden)]
pub type PutUsersInputOperationOutputAlias = crate::operation::PutUsers;
#[doc(hidden)]
pub type PutUsersInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl PutUsersInput {
    /// Consumes the builder and constructs an Operation<[`PutUsers`](crate::operation::PutUsers)>
    #[allow(clippy::let_and_return)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::PutUsers,
            aws_http::AwsErrorRetryPolicy,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        fn uri_base(
            _input: &crate::input::PutUsersInput,
            output: &mut String,
        ) -> Result<(), aws_smithy_http::operation::BuildError> {
            write!(output, "/users").expect("formatting should succeed");
            Ok(())
        }
        #[allow(clippy::unnecessary_wraps)]
        fn update_http_builder(
            input: &crate::input::PutUsersInput,
            builder: http::request::Builder,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            let mut uri = String::new();
            uri_base(input, &mut uri)?;
            Ok(builder.method("POST").uri(uri))
        }
        #[allow(clippy::unnecessary_wraps)]
        fn request_builder_base(
            input: &crate::input::PutUsersInput,
        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
        {
            #[allow(unused_mut)]
            let mut builder = update_http_builder(input, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::HeaderName::from_static("content-type"),
                "application/json",
            );
            Ok(builder)
        }
        let properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        let request = request_builder_base(&self)?;
        let body = crate::operation_ser::serialize_operation_crate_operation_put_users(&self)
            .map_err(|err| {
                aws_smithy_http::operation::BuildError::SerializationError(err.into())
            })?;
        let request = Self::assemble(request, body);
        #[allow(unused_mut)]
        let mut request = aws_smithy_http::operation::Request::from_parts(
            request.map(aws_smithy_http::body::SdkBody::from),
            properties,
        );
        request
            .properties_mut()
            .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                crate::API_METADATA.clone(),
            ));
        #[allow(unused_mut)]
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        aws_endpoint::set_endpoint_resolver(
            &mut request.properties_mut(),
            _config.endpoint_resolver.clone(),
        );
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op =
            aws_smithy_http::operation::Operation::new(request, crate::operation::PutUsers::new())
                .with_metadata(aws_smithy_http::operation::Metadata::new(
                    "PutUsers",
                    "personalizeevents",
                ));
        let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
        Ok(op)
    }
    fn assemble(
        builder: http::request::Builder,
        body: aws_smithy_http::body::SdkBody,
    ) -> http::request::Request<aws_smithy_http::body::SdkBody> {
        let mut builder = builder;
        if let Some(content_length) = body.content_length() {
            builder = aws_smithy_http::header::set_header_if_absent(
                builder,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`PutUsersInput`](crate::input::PutUsersInput)
    pub fn builder() -> crate::input::put_users_input::Builder {
        crate::input::put_users_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutUsersInput {
    /// <p>The Amazon Resource Name (ARN) of the Users dataset you are adding the user or users to.</p>
    pub dataset_arn: std::option::Option<std::string::String>,
    /// <p>A list of user data.</p>
    pub users: std::option::Option<std::vec::Vec<crate::model::User>>,
}
impl std::fmt::Debug for PutUsersInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PutUsersInput");
        formatter.field("dataset_arn", &self.dataset_arn);
        formatter.field("users", &self.users);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutItemsInput {
    /// <p>The Amazon Resource Name (ARN) of the Items dataset you are adding the item or items to.</p>
    pub dataset_arn: std::option::Option<std::string::String>,
    /// <p>A list of item data.</p>
    pub items: std::option::Option<std::vec::Vec<crate::model::Item>>,
}
impl std::fmt::Debug for PutItemsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PutItemsInput");
        formatter.field("dataset_arn", &self.dataset_arn);
        formatter.field("items", &self.items);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutEventsInput {
    /// <p>The tracking ID for the event.
    /// The ID is generated by a call to the
    /// <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CreateEventTracker.html">CreateEventTracker</a> API.</p>
    pub tracking_id: std::option::Option<std::string::String>,
    /// <p>The user associated with the event.</p>
    pub user_id: std::option::Option<std::string::String>,
    /// <p>The session ID associated with the user's visit. Your application generates the sessionId when a user first visits your website or uses your application.
    /// Amazon Personalize uses the sessionId to associate events with the user before they log in. For more information, see
    /// <a href="https://docs.aws.amazon.com/personalize/latest/dg/recording-events.html">Recording Events</a>.</p>
    pub session_id: std::option::Option<std::string::String>,
    /// <p>A list of event data from the session.</p>
    pub event_list: std::option::Option<std::vec::Vec<crate::model::Event>>,
}
impl std::fmt::Debug for PutEventsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PutEventsInput");
        formatter.field("tracking_id", &self.tracking_id);
        formatter.field("user_id", &self.user_id);
        formatter.field("session_id", &self.session_id);
        formatter.field("event_list", &self.event_list);
        formatter.finish()
    }
}
