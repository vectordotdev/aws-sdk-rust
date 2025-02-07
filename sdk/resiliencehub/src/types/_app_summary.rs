// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Defines an application summary.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AppSummary {
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    #[doc(hidden)]
    pub app_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the application.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The optional description for an app.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The timestamp for when the app was created.</p>
    #[doc(hidden)]
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The current status of compliance for the resiliency policy.</p>
    #[doc(hidden)]
    pub compliance_status: ::std::option::Option<crate::types::AppComplianceStatusType>,
    /// <p>The current resiliency score for the application.</p>
    #[doc(hidden)]
    pub resiliency_score: f64,
    /// <p> Assessment execution schedule with 'Daily' or 'Disabled' values. </p>
    #[doc(hidden)]
    pub assessment_schedule: ::std::option::Option<crate::types::AppAssessmentScheduleType>,
    /// <p>The status of the application.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::AppStatusType>,
}
impl AppSummary {
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn app_arn(&self) -> ::std::option::Option<&str> {
        self.app_arn.as_deref()
    }
    /// <p>The name of the application.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The optional description for an app.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The timestamp for when the app was created.</p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The current status of compliance for the resiliency policy.</p>
    pub fn compliance_status(
        &self,
    ) -> ::std::option::Option<&crate::types::AppComplianceStatusType> {
        self.compliance_status.as_ref()
    }
    /// <p>The current resiliency score for the application.</p>
    pub fn resiliency_score(&self) -> f64 {
        self.resiliency_score
    }
    /// <p> Assessment execution schedule with 'Daily' or 'Disabled' values. </p>
    pub fn assessment_schedule(
        &self,
    ) -> ::std::option::Option<&crate::types::AppAssessmentScheduleType> {
        self.assessment_schedule.as_ref()
    }
    /// <p>The status of the application.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::AppStatusType> {
        self.status.as_ref()
    }
}
impl AppSummary {
    /// Creates a new builder-style object to manufacture [`AppSummary`](crate::types::AppSummary).
    pub fn builder() -> crate::types::builders::AppSummaryBuilder {
        crate::types::builders::AppSummaryBuilder::default()
    }
}

/// A builder for [`AppSummary`](crate::types::AppSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AppSummaryBuilder {
    pub(crate) app_arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) compliance_status: ::std::option::Option<crate::types::AppComplianceStatusType>,
    pub(crate) resiliency_score: ::std::option::Option<f64>,
    pub(crate) assessment_schedule: ::std::option::Option<crate::types::AppAssessmentScheduleType>,
    pub(crate) status: ::std::option::Option<crate::types::AppStatusType>,
}
impl AppSummaryBuilder {
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn app_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.app_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn set_app_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.app_arn = input;
        self
    }
    /// <p>The name of the application.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the application.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The optional description for an app.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The optional description for an app.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The timestamp for when the app was created.</p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp for when the app was created.</p>
    pub fn set_creation_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The current status of compliance for the resiliency policy.</p>
    pub fn compliance_status(mut self, input: crate::types::AppComplianceStatusType) -> Self {
        self.compliance_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of compliance for the resiliency policy.</p>
    pub fn set_compliance_status(
        mut self,
        input: ::std::option::Option<crate::types::AppComplianceStatusType>,
    ) -> Self {
        self.compliance_status = input;
        self
    }
    /// <p>The current resiliency score for the application.</p>
    pub fn resiliency_score(mut self, input: f64) -> Self {
        self.resiliency_score = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current resiliency score for the application.</p>
    pub fn set_resiliency_score(mut self, input: ::std::option::Option<f64>) -> Self {
        self.resiliency_score = input;
        self
    }
    /// <p> Assessment execution schedule with 'Daily' or 'Disabled' values. </p>
    pub fn assessment_schedule(mut self, input: crate::types::AppAssessmentScheduleType) -> Self {
        self.assessment_schedule = ::std::option::Option::Some(input);
        self
    }
    /// <p> Assessment execution schedule with 'Daily' or 'Disabled' values. </p>
    pub fn set_assessment_schedule(
        mut self,
        input: ::std::option::Option<crate::types::AppAssessmentScheduleType>,
    ) -> Self {
        self.assessment_schedule = input;
        self
    }
    /// <p>The status of the application.</p>
    pub fn status(mut self, input: crate::types::AppStatusType) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the application.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::AppStatusType>) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`AppSummary`](crate::types::AppSummary).
    pub fn build(self) -> crate::types::AppSummary {
        crate::types::AppSummary {
            app_arn: self.app_arn,
            name: self.name,
            description: self.description,
            creation_time: self.creation_time,
            compliance_status: self.compliance_status,
            resiliency_score: self.resiliency_score.unwrap_or_default(),
            assessment_schedule: self.assessment_schedule,
            status: self.status,
        }
    }
}
