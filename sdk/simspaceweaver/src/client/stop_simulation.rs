// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopSimulation`](crate::operation::stop_simulation::builders::StopSimulationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`simulation(impl ::std::convert::Into<String>)`](crate::operation::stop_simulation::builders::StopSimulationFluentBuilder::simulation) / [`set_simulation(Option<String>)`](crate::operation::stop_simulation::builders::StopSimulationFluentBuilder::set_simulation): <p>The name of the simulation.</p>
    /// - On success, responds with [`StopSimulationOutput`](crate::operation::stop_simulation::StopSimulationOutput)
    /// - On failure, responds with [`SdkError<StopSimulationError>`](crate::operation::stop_simulation::StopSimulationError)
    pub fn stop_simulation(
        &self,
    ) -> crate::operation::stop_simulation::builders::StopSimulationFluentBuilder {
        crate::operation::stop_simulation::builders::StopSimulationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
