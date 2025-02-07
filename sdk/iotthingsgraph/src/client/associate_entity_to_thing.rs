// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateEntityToThing`](crate::operation::associate_entity_to_thing::builders::AssociateEntityToThingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`thing_name(impl ::std::convert::Into<String>)`](crate::operation::associate_entity_to_thing::builders::AssociateEntityToThingFluentBuilder::thing_name) / [`set_thing_name(Option<String>)`](crate::operation::associate_entity_to_thing::builders::AssociateEntityToThingFluentBuilder::set_thing_name): <p>The name of the thing to which the entity is to be associated.</p>
    ///   - [`entity_id(impl ::std::convert::Into<String>)`](crate::operation::associate_entity_to_thing::builders::AssociateEntityToThingFluentBuilder::entity_id) / [`set_entity_id(Option<String>)`](crate::operation::associate_entity_to_thing::builders::AssociateEntityToThingFluentBuilder::set_entity_id): <p>The ID of the device to be associated with the thing.</p>  <p>The ID should be in the following format.</p>  <p> <code>urn:tdm:REGION/ACCOUNT ID/default:device:DEVICENAME</code> </p>
    ///   - [`namespace_version(i64)`](crate::operation::associate_entity_to_thing::builders::AssociateEntityToThingFluentBuilder::namespace_version) / [`set_namespace_version(Option<i64>)`](crate::operation::associate_entity_to_thing::builders::AssociateEntityToThingFluentBuilder::set_namespace_version): <p>The version of the user's namespace. Defaults to the latest version of the user's namespace.</p>
    /// - On success, responds with [`AssociateEntityToThingOutput`](crate::operation::associate_entity_to_thing::AssociateEntityToThingOutput)
    /// - On failure, responds with [`SdkError<AssociateEntityToThingError>`](crate::operation::associate_entity_to_thing::AssociateEntityToThingError)
    #[deprecated(note = "since: 2022-08-30")]
    pub fn associate_entity_to_thing(
        &self,
    ) -> crate::operation::associate_entity_to_thing::builders::AssociateEntityToThingFluentBuilder
    {
        crate::operation::associate_entity_to_thing::builders::AssociateEntityToThingFluentBuilder::new(self.handle.clone())
    }
}
