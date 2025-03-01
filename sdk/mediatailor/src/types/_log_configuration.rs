// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Returns Amazon CloudWatch log settings for a playback configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LogConfiguration {
    /// <p>The percentage of session logs that MediaTailor sends to your Cloudwatch Logs account. For example, if your playback configuration has 1000 sessions and <code>percentEnabled</code> is set to <code>60</code>, MediaTailor sends logs for 600 of the sessions to CloudWatch Logs. MediaTailor decides at random which of the playback configuration sessions to send logs for. If you want to view logs for a specific session, you can use the <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/debug-log-mode.html">debug log mode</a>.</p>
    /// <p>Valid values: <code>0</code> - <code>100</code> </p>
    #[doc(hidden)]
    pub percent_enabled: i32,
}
impl LogConfiguration {
    /// <p>The percentage of session logs that MediaTailor sends to your Cloudwatch Logs account. For example, if your playback configuration has 1000 sessions and <code>percentEnabled</code> is set to <code>60</code>, MediaTailor sends logs for 600 of the sessions to CloudWatch Logs. MediaTailor decides at random which of the playback configuration sessions to send logs for. If you want to view logs for a specific session, you can use the <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/debug-log-mode.html">debug log mode</a>.</p>
    /// <p>Valid values: <code>0</code> - <code>100</code> </p>
    pub fn percent_enabled(&self) -> i32 {
        self.percent_enabled
    }
}
impl LogConfiguration {
    /// Creates a new builder-style object to manufacture [`LogConfiguration`](crate::types::LogConfiguration).
    pub fn builder() -> crate::types::builders::LogConfigurationBuilder {
        crate::types::builders::LogConfigurationBuilder::default()
    }
}

/// A builder for [`LogConfiguration`](crate::types::LogConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LogConfigurationBuilder {
    pub(crate) percent_enabled: ::std::option::Option<i32>,
}
impl LogConfigurationBuilder {
    /// <p>The percentage of session logs that MediaTailor sends to your Cloudwatch Logs account. For example, if your playback configuration has 1000 sessions and <code>percentEnabled</code> is set to <code>60</code>, MediaTailor sends logs for 600 of the sessions to CloudWatch Logs. MediaTailor decides at random which of the playback configuration sessions to send logs for. If you want to view logs for a specific session, you can use the <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/debug-log-mode.html">debug log mode</a>.</p>
    /// <p>Valid values: <code>0</code> - <code>100</code> </p>
    pub fn percent_enabled(mut self, input: i32) -> Self {
        self.percent_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>The percentage of session logs that MediaTailor sends to your Cloudwatch Logs account. For example, if your playback configuration has 1000 sessions and <code>percentEnabled</code> is set to <code>60</code>, MediaTailor sends logs for 600 of the sessions to CloudWatch Logs. MediaTailor decides at random which of the playback configuration sessions to send logs for. If you want to view logs for a specific session, you can use the <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/debug-log-mode.html">debug log mode</a>.</p>
    /// <p>Valid values: <code>0</code> - <code>100</code> </p>
    pub fn set_percent_enabled(mut self, input: ::std::option::Option<i32>) -> Self {
        self.percent_enabled = input;
        self
    }
    /// Consumes the builder and constructs a [`LogConfiguration`](crate::types::LogConfiguration).
    pub fn build(self) -> crate::types::LogConfiguration {
        crate::types::LogConfiguration {
            percent_enabled: self.percent_enabled.unwrap_or_default(),
        }
    }
}
