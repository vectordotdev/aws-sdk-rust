// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSteps`](crate::operation::list_steps::builders::ListStepsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_steps::builders::ListStepsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_id(impl ::std::convert::Into<String>)`](crate::operation::list_steps::builders::ListStepsFluentBuilder::cluster_id) / [`set_cluster_id(Option<String>)`](crate::operation::list_steps::builders::ListStepsFluentBuilder::set_cluster_id): <p>The identifier of the cluster for which to list the steps.</p>
    ///   - [`step_states(Vec<StepState>)`](crate::operation::list_steps::builders::ListStepsFluentBuilder::step_states) / [`set_step_states(Option<Vec<StepState>>)`](crate::operation::list_steps::builders::ListStepsFluentBuilder::set_step_states): <p>The filter to limit the step list based on certain states.</p>
    ///   - [`step_ids(Vec<String>)`](crate::operation::list_steps::builders::ListStepsFluentBuilder::step_ids) / [`set_step_ids(Option<Vec<String>>)`](crate::operation::list_steps::builders::ListStepsFluentBuilder::set_step_ids): <p>The filter to limit the step list based on the identifier of the steps. You can specify a maximum of ten Step IDs. The character constraint applies to the overall length of the array.</p>
    ///   - [`marker(impl ::std::convert::Into<String>)`](crate::operation::list_steps::builders::ListStepsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_steps::builders::ListStepsFluentBuilder::set_marker): <p>The maximum number of steps that a single <code>ListSteps</code> action returns is 50. To return a longer list of steps, use multiple <code>ListSteps</code> actions along with the <code>Marker</code> parameter, which is a pagination token that indicates the next set of results to retrieve.</p>
    /// - On success, responds with [`ListStepsOutput`](crate::operation::list_steps::ListStepsOutput) with field(s):
    ///   - [`steps(Option<Vec<StepSummary>>)`](crate::operation::list_steps::ListStepsOutput::steps): <p>The filtered list of steps for the cluster.</p>
    ///   - [`marker(Option<String>)`](crate::operation::list_steps::ListStepsOutput::marker): <p>The maximum number of steps that a single <code>ListSteps</code> action returns is 50. To return a longer list of steps, use multiple <code>ListSteps</code> actions along with the <code>Marker</code> parameter, which is a pagination token that indicates the next set of results to retrieve.</p>
    /// - On failure, responds with [`SdkError<ListStepsError>`](crate::operation::list_steps::ListStepsError)
    pub fn list_steps(&self) -> crate::operation::list_steps::builders::ListStepsFluentBuilder {
        crate::operation::list_steps::builders::ListStepsFluentBuilder::new(self.handle.clone())
    }
}
