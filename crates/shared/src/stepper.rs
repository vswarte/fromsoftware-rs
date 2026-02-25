/// State indices for steppers.
///
/// # Safety
/// The implementer must ensure that the enum matches the games stepper states one-to-one including
/// the explicit -1 inactive state.
pub unsafe trait StepperStates: Copy + std::fmt::Debug + 'static {
    // Generic associated type since we can't use the count itself on FD4StepTemplateBase.
    type StepperFnArray<StepperFn>: AsRef<[StepperFn]>;
}
