// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteAppAssessmentOutput {
    /// <p>The Amazon Resource Name (ARN) of the assessment. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app-assessment/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    #[doc(hidden)]
    pub assessment_arn: ::std::option::Option<::std::string::String>,
    /// <p>The current status of the assessment for the resiliency policy.</p>
    #[doc(hidden)]
    pub assessment_status: ::std::option::Option<crate::types::AssessmentStatus>,
    _request_id: Option<String>,
}
impl DeleteAppAssessmentOutput {
    /// <p>The Amazon Resource Name (ARN) of the assessment. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app-assessment/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn assessment_arn(&self) -> ::std::option::Option<&str> {
        self.assessment_arn.as_deref()
    }
    /// <p>The current status of the assessment for the resiliency policy.</p>
    pub fn assessment_status(&self) -> ::std::option::Option<&crate::types::AssessmentStatus> {
        self.assessment_status.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DeleteAppAssessmentOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteAppAssessmentOutput {
    /// Creates a new builder-style object to manufacture [`DeleteAppAssessmentOutput`](crate::operation::delete_app_assessment::DeleteAppAssessmentOutput).
    pub fn builder(
    ) -> crate::operation::delete_app_assessment::builders::DeleteAppAssessmentOutputBuilder {
        crate::operation::delete_app_assessment::builders::DeleteAppAssessmentOutputBuilder::default(
        )
    }
}

/// A builder for [`DeleteAppAssessmentOutput`](crate::operation::delete_app_assessment::DeleteAppAssessmentOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteAppAssessmentOutputBuilder {
    pub(crate) assessment_arn: ::std::option::Option<::std::string::String>,
    pub(crate) assessment_status: ::std::option::Option<crate::types::AssessmentStatus>,
    _request_id: Option<String>,
}
impl DeleteAppAssessmentOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the assessment. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app-assessment/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn assessment_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.assessment_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the assessment. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app-assessment/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn set_assessment_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.assessment_arn = input;
        self
    }
    /// <p>The current status of the assessment for the resiliency policy.</p>
    pub fn assessment_status(mut self, input: crate::types::AssessmentStatus) -> Self {
        self.assessment_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the assessment for the resiliency policy.</p>
    pub fn set_assessment_status(
        mut self,
        input: ::std::option::Option<crate::types::AssessmentStatus>,
    ) -> Self {
        self.assessment_status = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteAppAssessmentOutput`](crate::operation::delete_app_assessment::DeleteAppAssessmentOutput).
    pub fn build(self) -> crate::operation::delete_app_assessment::DeleteAppAssessmentOutput {
        crate::operation::delete_app_assessment::DeleteAppAssessmentOutput {
            assessment_arn: self.assessment_arn,
            assessment_status: self.assessment_status,
            _request_id: self._request_id,
        }
    }
}
