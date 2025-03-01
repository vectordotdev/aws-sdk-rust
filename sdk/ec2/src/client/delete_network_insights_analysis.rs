// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteNetworkInsightsAnalysis`](crate::operation::delete_network_insights_analysis::builders::DeleteNetworkInsightsAnalysisFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::delete_network_insights_analysis::builders::DeleteNetworkInsightsAnalysisFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_network_insights_analysis::builders::DeleteNetworkInsightsAnalysisFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`network_insights_analysis_id(impl ::std::convert::Into<String>)`](crate::operation::delete_network_insights_analysis::builders::DeleteNetworkInsightsAnalysisFluentBuilder::network_insights_analysis_id) / [`set_network_insights_analysis_id(Option<String>)`](crate::operation::delete_network_insights_analysis::builders::DeleteNetworkInsightsAnalysisFluentBuilder::set_network_insights_analysis_id): <p>The ID of the network insights analysis.</p>
    /// - On success, responds with [`DeleteNetworkInsightsAnalysisOutput`](crate::operation::delete_network_insights_analysis::DeleteNetworkInsightsAnalysisOutput) with field(s):
    ///   - [`network_insights_analysis_id(Option<String>)`](crate::operation::delete_network_insights_analysis::DeleteNetworkInsightsAnalysisOutput::network_insights_analysis_id): <p>The ID of the network insights analysis.</p>
    /// - On failure, responds with [`SdkError<DeleteNetworkInsightsAnalysisError>`](crate::operation::delete_network_insights_analysis::DeleteNetworkInsightsAnalysisError)
    pub fn delete_network_insights_analysis(&self) -> crate::operation::delete_network_insights_analysis::builders::DeleteNetworkInsightsAnalysisFluentBuilder{
        crate::operation::delete_network_insights_analysis::builders::DeleteNetworkInsightsAnalysisFluentBuilder::new(self.handle.clone())
    }
}
