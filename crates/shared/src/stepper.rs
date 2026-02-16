/// State indices for steppers.
///
/// # Safety
///
/// The implementer must ensure that the trait is implemented exclusively on enums with a
/// #[repr(i32)].
/// The implementer must ensure that the enum contains only unit variants.
/// The implementer must ensure that the enum has a -1 discriminant to represent the inactive
/// state.
/// The implementer must ensure that the enum is exhaustive as unknown discriminants can be used to
/// trigger undefined behavior.
/// The implementer must ensure that the enum does not have more states than the game defines.
/// Failing to do so will allow for out-of-bound access to the stepper array.
/// The implementer must ensure that the enum discriminants have no gaps. Failing to do so will
/// allow out of bounds access to the stepper array as well as cause unknown discriminants.
pub unsafe trait StepperStates: Copy + std::fmt::Debug + 'static {
    // GAT since we can't use the count itself on FD4StepTemplateBase.
    type StepperFnArray<StepperFn>: AsRef<[StepperFn]>;
}
