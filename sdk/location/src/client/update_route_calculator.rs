// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateRouteCalculator`](crate::operation::update_route_calculator::builders::UpdateRouteCalculatorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`calculator_name(impl ::std::convert::Into<String>)`](crate::operation::update_route_calculator::builders::UpdateRouteCalculatorFluentBuilder::calculator_name) / [`set_calculator_name(Option<String>)`](crate::operation::update_route_calculator::builders::UpdateRouteCalculatorFluentBuilder::set_calculator_name): <p>The name of the route calculator resource to update.</p>
    ///   - [`pricing_plan(PricingPlan)`](crate::operation::update_route_calculator::builders::UpdateRouteCalculatorFluentBuilder::pricing_plan) / [`set_pricing_plan(Option<PricingPlan>)`](crate::operation::update_route_calculator::builders::UpdateRouteCalculatorFluentBuilder::set_pricing_plan): <p>No longer used. If included, the only allowed value is <code>RequestBasedUsage</code>.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_route_calculator::builders::UpdateRouteCalculatorFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_route_calculator::builders::UpdateRouteCalculatorFluentBuilder::set_description): <p>Updates the description for the route calculator resource.</p>
    /// - On success, responds with [`UpdateRouteCalculatorOutput`](crate::operation::update_route_calculator::UpdateRouteCalculatorOutput) with field(s):
    ///   - [`calculator_name(Option<String>)`](crate::operation::update_route_calculator::UpdateRouteCalculatorOutput::calculator_name): <p>The name of the updated route calculator resource.</p>
    ///   - [`calculator_arn(Option<String>)`](crate::operation::update_route_calculator::UpdateRouteCalculatorOutput::calculator_arn): <p>The Amazon Resource Name (ARN) of the updated route calculator resource. Used to specify a resource across AWS.</p>  <ul>   <li> <p>Format example: <code>arn:aws:geo:region:account-id:route- calculator/ExampleCalculator</code> </p> </li>  </ul>
    ///   - [`update_time(Option<DateTime>)`](crate::operation::update_route_calculator::UpdateRouteCalculatorOutput::update_time): <p>The timestamp for when the route calculator was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. </p>
    /// - On failure, responds with [`SdkError<UpdateRouteCalculatorError>`](crate::operation::update_route_calculator::UpdateRouteCalculatorError)
    pub fn update_route_calculator(
        &self,
    ) -> crate::operation::update_route_calculator::builders::UpdateRouteCalculatorFluentBuilder
    {
        crate::operation::update_route_calculator::builders::UpdateRouteCalculatorFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
