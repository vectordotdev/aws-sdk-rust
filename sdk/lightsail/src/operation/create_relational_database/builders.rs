// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_relational_database::_create_relational_database_output::CreateRelationalDatabaseOutputBuilder;

pub use crate::operation::create_relational_database::_create_relational_database_input::CreateRelationalDatabaseInputBuilder;

/// Fluent builder constructing a request to `CreateRelationalDatabase`.
///
/// <p>Creates a new database in Amazon Lightsail.</p>
/// <p>The <code>create relational database</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en_us/articles/amazon-lightsail-controlling-access-using-tags">Amazon Lightsail Developer Guide</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRelationalDatabaseFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_relational_database::builders::CreateRelationalDatabaseInputBuilder,
}
impl CreateRelationalDatabaseFluentBuilder {
    /// Creates a new `CreateRelationalDatabase`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_relational_database::CreateRelationalDatabase,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_relational_database::CreateRelationalDatabaseError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_relational_database::CreateRelationalDatabaseOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_relational_database::CreateRelationalDatabaseError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_relational_database::CreateRelationalDatabaseOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_relational_database::CreateRelationalDatabaseError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_relational_database::CreateRelationalDatabase,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_relational_database::CreateRelationalDatabaseError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name to use for your new Lightsail database resource.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p> </li>
    /// <li> <p>The first and last character must be a letter or number.</p> </li>
    /// </ul>
    pub fn relational_database_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.relational_database_name(input.into());
        self
    }
    /// <p>The name to use for your new Lightsail database resource.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p> </li>
    /// <li> <p>The first and last character must be a letter or number.</p> </li>
    /// </ul>
    pub fn set_relational_database_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_relational_database_name(input);
        self
    }
    /// <p>The Availability Zone in which to create your new database. Use the <code>us-east-2a</code> case-sensitive format.</p>
    /// <p>You can get a list of Availability Zones by using the <code>get regions</code> operation. Be sure to add the <code>include relational database Availability Zones</code> parameter to your request.</p>
    pub fn availability_zone(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// <p>The Availability Zone in which to create your new database. Use the <code>us-east-2a</code> case-sensitive format.</p>
    /// <p>You can get a list of Availability Zones by using the <code>get regions</code> operation. Be sure to add the <code>include relational database Availability Zones</code> parameter to your request.</p>
    pub fn set_availability_zone(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// <p>The blueprint ID for your new database. A blueprint describes the major engine version of a database.</p>
    /// <p>You can get a list of database blueprints IDs by using the <code>get relational database blueprints</code> operation.</p>
    pub fn relational_database_blueprint_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.relational_database_blueprint_id(input.into());
        self
    }
    /// <p>The blueprint ID for your new database. A blueprint describes the major engine version of a database.</p>
    /// <p>You can get a list of database blueprints IDs by using the <code>get relational database blueprints</code> operation.</p>
    pub fn set_relational_database_blueprint_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_relational_database_blueprint_id(input);
        self
    }
    /// <p>The bundle ID for your new database. A bundle describes the performance specifications for your database.</p>
    /// <p>You can get a list of database bundle IDs by using the <code>get relational database bundles</code> operation.</p>
    pub fn relational_database_bundle_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.relational_database_bundle_id(input.into());
        self
    }
    /// <p>The bundle ID for your new database. A bundle describes the performance specifications for your database.</p>
    /// <p>You can get a list of database bundle IDs by using the <code>get relational database bundles</code> operation.</p>
    pub fn set_relational_database_bundle_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_relational_database_bundle_id(input);
        self
    }
    /// <p>The meaning of this parameter differs according to the database engine you use.</p>
    /// <p> <b>MySQL</b> </p>
    /// <p>The name of the database to create when the Lightsail database resource is created. If this parameter isn't specified, no database is created in the database resource.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain 1 to 64 letters or numbers.</p> </li>
    /// <li> <p>Must begin with a letter. Subsequent characters can be letters, underscores, or digits (0- 9).</p> </li>
    /// <li> <p>Can't be a word reserved by the specified database engine.</p> <p>For more information about reserved words in MySQL, see the Keywords and Reserved Words articles for <a href="https://dev.mysql.com/doc/refman/5.6/en/keywords.html">MySQL 5.6</a>, <a href="https://dev.mysql.com/doc/refman/5.7/en/keywords.html">MySQL 5.7</a>, and <a href="https://dev.mysql.com/doc/refman/8.0/en/keywords.html">MySQL 8.0</a>.</p> </li>
    /// </ul>
    /// <p> <b>PostgreSQL</b> </p>
    /// <p>The name of the database to create when the Lightsail database resource is created. If this parameter isn't specified, a database named <code>postgres</code> is created in the database resource.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain 1 to 63 letters or numbers.</p> </li>
    /// <li> <p>Must begin with a letter. Subsequent characters can be letters, underscores, or digits (0- 9).</p> </li>
    /// <li> <p>Can't be a word reserved by the specified database engine.</p> <p>For more information about reserved words in PostgreSQL, see the SQL Key Words articles for <a href="https://www.postgresql.org/docs/9.6/sql-keywords-appendix.html">PostgreSQL 9.6</a>, <a href="https://www.postgresql.org/docs/10/sql-keywords-appendix.html">PostgreSQL 10</a>, <a href="https://www.postgresql.org/docs/11/sql-keywords-appendix.html">PostgreSQL 11</a>, and <a href="https://www.postgresql.org/docs/12/sql-keywords-appendix.html">PostgreSQL 12</a>.</p> </li>
    /// </ul>
    pub fn master_database_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.master_database_name(input.into());
        self
    }
    /// <p>The meaning of this parameter differs according to the database engine you use.</p>
    /// <p> <b>MySQL</b> </p>
    /// <p>The name of the database to create when the Lightsail database resource is created. If this parameter isn't specified, no database is created in the database resource.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain 1 to 64 letters or numbers.</p> </li>
    /// <li> <p>Must begin with a letter. Subsequent characters can be letters, underscores, or digits (0- 9).</p> </li>
    /// <li> <p>Can't be a word reserved by the specified database engine.</p> <p>For more information about reserved words in MySQL, see the Keywords and Reserved Words articles for <a href="https://dev.mysql.com/doc/refman/5.6/en/keywords.html">MySQL 5.6</a>, <a href="https://dev.mysql.com/doc/refman/5.7/en/keywords.html">MySQL 5.7</a>, and <a href="https://dev.mysql.com/doc/refman/8.0/en/keywords.html">MySQL 8.0</a>.</p> </li>
    /// </ul>
    /// <p> <b>PostgreSQL</b> </p>
    /// <p>The name of the database to create when the Lightsail database resource is created. If this parameter isn't specified, a database named <code>postgres</code> is created in the database resource.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain 1 to 63 letters or numbers.</p> </li>
    /// <li> <p>Must begin with a letter. Subsequent characters can be letters, underscores, or digits (0- 9).</p> </li>
    /// <li> <p>Can't be a word reserved by the specified database engine.</p> <p>For more information about reserved words in PostgreSQL, see the SQL Key Words articles for <a href="https://www.postgresql.org/docs/9.6/sql-keywords-appendix.html">PostgreSQL 9.6</a>, <a href="https://www.postgresql.org/docs/10/sql-keywords-appendix.html">PostgreSQL 10</a>, <a href="https://www.postgresql.org/docs/11/sql-keywords-appendix.html">PostgreSQL 11</a>, and <a href="https://www.postgresql.org/docs/12/sql-keywords-appendix.html">PostgreSQL 12</a>.</p> </li>
    /// </ul>
    pub fn set_master_database_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_master_database_name(input);
        self
    }
    /// <p>The name for the master user.</p>
    /// <p> <b>MySQL</b> </p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Required for MySQL.</p> </li>
    /// <li> <p>Must be 1 to 16 letters or numbers. Can contain underscores.</p> </li>
    /// <li> <p>First character must be a letter.</p> </li>
    /// <li> <p>Can't be a reserved word for the chosen database engine.</p> <p>For more information about reserved words in MySQL 5.6 or 5.7, see the Keywords and Reserved Words articles for <a href="https://dev.mysql.com/doc/refman/5.6/en/keywords.html">MySQL 5.6</a>, <a href="https://dev.mysql.com/doc/refman/5.7/en/keywords.html">MySQL 5.7</a>, or <a href="https://dev.mysql.com/doc/refman/8.0/en/keywords.html">MySQL 8.0</a>.</p> </li>
    /// </ul>
    /// <p> <b>PostgreSQL</b> </p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Required for PostgreSQL.</p> </li>
    /// <li> <p>Must be 1 to 63 letters or numbers. Can contain underscores.</p> </li>
    /// <li> <p>First character must be a letter.</p> </li>
    /// <li> <p>Can't be a reserved word for the chosen database engine.</p> <p>For more information about reserved words in MySQL 5.6 or 5.7, see the Keywords and Reserved Words articles for <a href="https://www.postgresql.org/docs/9.6/sql-keywords-appendix.html">PostgreSQL 9.6</a>, <a href="https://www.postgresql.org/docs/10/sql-keywords-appendix.html">PostgreSQL 10</a>, <a href="https://www.postgresql.org/docs/11/sql-keywords-appendix.html">PostgreSQL 11</a>, and <a href="https://www.postgresql.org/docs/12/sql-keywords-appendix.html">PostgreSQL 12</a>.</p> </li>
    /// </ul>
    pub fn master_username(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.master_username(input.into());
        self
    }
    /// <p>The name for the master user.</p>
    /// <p> <b>MySQL</b> </p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Required for MySQL.</p> </li>
    /// <li> <p>Must be 1 to 16 letters or numbers. Can contain underscores.</p> </li>
    /// <li> <p>First character must be a letter.</p> </li>
    /// <li> <p>Can't be a reserved word for the chosen database engine.</p> <p>For more information about reserved words in MySQL 5.6 or 5.7, see the Keywords and Reserved Words articles for <a href="https://dev.mysql.com/doc/refman/5.6/en/keywords.html">MySQL 5.6</a>, <a href="https://dev.mysql.com/doc/refman/5.7/en/keywords.html">MySQL 5.7</a>, or <a href="https://dev.mysql.com/doc/refman/8.0/en/keywords.html">MySQL 8.0</a>.</p> </li>
    /// </ul>
    /// <p> <b>PostgreSQL</b> </p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Required for PostgreSQL.</p> </li>
    /// <li> <p>Must be 1 to 63 letters or numbers. Can contain underscores.</p> </li>
    /// <li> <p>First character must be a letter.</p> </li>
    /// <li> <p>Can't be a reserved word for the chosen database engine.</p> <p>For more information about reserved words in MySQL 5.6 or 5.7, see the Keywords and Reserved Words articles for <a href="https://www.postgresql.org/docs/9.6/sql-keywords-appendix.html">PostgreSQL 9.6</a>, <a href="https://www.postgresql.org/docs/10/sql-keywords-appendix.html">PostgreSQL 10</a>, <a href="https://www.postgresql.org/docs/11/sql-keywords-appendix.html">PostgreSQL 11</a>, and <a href="https://www.postgresql.org/docs/12/sql-keywords-appendix.html">PostgreSQL 12</a>.</p> </li>
    /// </ul>
    pub fn set_master_username(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_master_username(input);
        self
    }
    /// <p>The password for the master user. The password can include any printable ASCII character except "/", """, or "@". It cannot contain spaces.</p>
    /// <p> <b>MySQL</b> </p>
    /// <p>Constraints: Must contain from 8 to 41 characters.</p>
    /// <p> <b>PostgreSQL</b> </p>
    /// <p>Constraints: Must contain from 8 to 128 characters.</p>
    pub fn master_user_password(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.master_user_password(input.into());
        self
    }
    /// <p>The password for the master user. The password can include any printable ASCII character except "/", """, or "@". It cannot contain spaces.</p>
    /// <p> <b>MySQL</b> </p>
    /// <p>Constraints: Must contain from 8 to 41 characters.</p>
    /// <p> <b>PostgreSQL</b> </p>
    /// <p>Constraints: Must contain from 8 to 128 characters.</p>
    pub fn set_master_user_password(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_master_user_password(input);
        self
    }
    /// <p>The daily time range during which automated backups are created for your new database if automated backups are enabled.</p>
    /// <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region. For more information about the preferred backup window time blocks for each region, see the <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_WorkingWithAutomatedBackups.html#USER_WorkingWithAutomatedBackups.BackupWindow">Working With Backups</a> guide in the Amazon Relational Database Service documentation.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must be in the <code>hh24:mi-hh24:mi</code> format.</p> <p>Example: <code>16:00-16:30</code> </p> </li>
    /// <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li>
    /// <li> <p>Must not conflict with the preferred maintenance window.</p> </li>
    /// <li> <p>Must be at least 30 minutes.</p> </li>
    /// </ul>
    pub fn preferred_backup_window(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.preferred_backup_window(input.into());
        self
    }
    /// <p>The daily time range during which automated backups are created for your new database if automated backups are enabled.</p>
    /// <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region. For more information about the preferred backup window time blocks for each region, see the <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_WorkingWithAutomatedBackups.html#USER_WorkingWithAutomatedBackups.BackupWindow">Working With Backups</a> guide in the Amazon Relational Database Service documentation.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must be in the <code>hh24:mi-hh24:mi</code> format.</p> <p>Example: <code>16:00-16:30</code> </p> </li>
    /// <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li>
    /// <li> <p>Must not conflict with the preferred maintenance window.</p> </li>
    /// <li> <p>Must be at least 30 minutes.</p> </li>
    /// </ul>
    pub fn set_preferred_backup_window(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_preferred_backup_window(input);
        self
    }
    /// <p>The weekly time range during which system maintenance can occur on your new database.</p>
    /// <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must be in the <code>ddd:hh24:mi-ddd:hh24:mi</code> format.</p> </li>
    /// <li> <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p> </li>
    /// <li> <p>Must be at least 30 minutes.</p> </li>
    /// <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li>
    /// <li> <p>Example: <code>Tue:17:00-Tue:17:30</code> </p> </li>
    /// </ul>
    pub fn preferred_maintenance_window(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.preferred_maintenance_window(input.into());
        self
    }
    /// <p>The weekly time range during which system maintenance can occur on your new database.</p>
    /// <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must be in the <code>ddd:hh24:mi-ddd:hh24:mi</code> format.</p> </li>
    /// <li> <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p> </li>
    /// <li> <p>Must be at least 30 minutes.</p> </li>
    /// <li> <p>Specified in Coordinated Universal Time (UTC).</p> </li>
    /// <li> <p>Example: <code>Tue:17:00-Tue:17:30</code> </p> </li>
    /// </ul>
    pub fn set_preferred_maintenance_window(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_preferred_maintenance_window(input);
        self
    }
    /// <p>Specifies the accessibility options for your new database. A value of <code>true</code> specifies a database that is available to resources outside of your Lightsail account. A value of <code>false</code> specifies a database that is available only to your Lightsail resources in the same region as your database.</p>
    pub fn publicly_accessible(mut self, input: bool) -> Self {
        self.inner = self.inner.publicly_accessible(input);
        self
    }
    /// <p>Specifies the accessibility options for your new database. A value of <code>true</code> specifies a database that is available to resources outside of your Lightsail account. A value of <code>false</code> specifies a database that is available only to your Lightsail resources in the same region as your database.</p>
    pub fn set_publicly_accessible(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_publicly_accessible(input);
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tag keys and optional values to add to the resource during create.</p>
    /// <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tag keys and optional values to add to the resource during create.</p>
    /// <p>Use the <code>TagResource</code> action to tag a resource after it's created.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
