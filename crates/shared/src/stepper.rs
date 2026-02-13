/// State index for steppers.
pub trait StepperStates: Copy + std::fmt::Debug + 'static {
    // GAT since we can't use the count itself on FD4StepTemplateBase.
    type StepperFnArray<TStepperFn>: AsRef<[TStepperFn]>;
}
