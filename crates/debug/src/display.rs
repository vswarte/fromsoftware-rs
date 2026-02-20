/// Defines traits rendering debug widgets, as well as structs that use them.
///
/// We can't define these directly in this package because each individual debug
/// tool needs to implement the trait for types from the game, and that's not
/// possible if both the trait and the types are defined outside the debug tool.
#[macro_export]
macro_rules! define_debug_display {
    () => {
        /// A trait for structs that can render immutable, stateless debug
        /// widgets.
        pub trait DebugDisplay {
            /// Renders a debug widget for this struct.
            fn render_debug(&self, ui: &::hudhook::imgui::Ui);
        }

        impl<T: DebugDisplay> DebugDisplay for &T {
            fn render_debug(&self, ui: &::hudhook::imgui::Ui) {
                <T as DebugDisplay>::render_debug(self, ui);
            }
        }

        impl<T: DebugDisplay> DebugDisplay for &mut T {
            fn render_debug(&self, ui: &::hudhook::imgui::Ui) {
                <T as DebugDisplay>::render_debug(self, ui);
            }
        }

        impl<T: DebugDisplay> DebugDisplay for ::fromsoftware_shared::OwnedPtr<T> {
            fn render_debug(&self, ui: &::hudhook::imgui::Ui) {
                <T as DebugDisplay>::render_debug(self, ui);
            }
        }

        /// A trait for structs that can render mutable debug widgets that track
        /// state across frames.
        ///
        /// This is used for widgets that allow the user to interact with the
        /// game in some way.
        pub trait StatefulDebugDisplay {
            /// The state associated with this trait. The caller should ensure
            /// that an instance of this is created along with this struct and
            /// passed to [render_debug_mut] each time it's called.
            type State: Default;

            /// Renders a debug widget for this struct.
            fn render_debug_mut(&mut self, ui: &::hudhook::imgui::Ui, state: &mut Self::State);
        }

        impl<T: DebugDisplay> StatefulDebugDisplay for T {
            type State = ();

            fn render_debug_mut(&mut self, ui: &::hudhook::imgui::Ui, _: &mut Self::State) {
                self.render_debug(ui);
            }
        }

        /// A struct that renders a widget for `T` when it's available and
        /// automatically handles any state it needs to track.
        pub struct StaticDebugger<T>
        where
            T: StatefulDebugDisplay + ::fromsoftware_shared::FromStatic + 'static,
        {
            state: T::State,
        }

        impl<T> StaticDebugger<T>
        where
            T: StatefulDebugDisplay + ::fromsoftware_shared::FromStatic + 'static,
        {
            pub fn new() -> Self {
                Self {
                    state: Default::default(),
                }
            }

            pub fn render_debug(&mut self, ui: &::hudhook::imgui::Ui) {
                let instance = unsafe { T::instance() };

                match instance {
                    Ok(instance) => ::debug::UiExt::header(ui, &T::name(), || {
                        ::debug::UiExt::pointer(ui, "Address", &*instance);
                        instance.render_debug_mut(ui, &mut self.state);
                    }),
                    Err(err) => ui.text(format!("Couldn't load {}: {:?}", T::name(), err)),
                }
            }
        }

        impl<T> Default for StaticDebugger<T>
        where
            T: StatefulDebugDisplay + ::fromsoftware_shared::FromStatic + 'static,
        {
            fn default() -> Self {
                Self::new()
            }
        }

        /// An additional extension trait for UI helpers that use the
        /// [DebugDisplay] trait.
        pub trait DisplayUiExt {
            /// Renders a collapsing header with the given `label` and the
            /// contents of `display` nested beneath it.
            ///
            /// This automatically creates a unique ID for the header so that
            /// its collapsed state doesn't collide with other headers.
            fn nested(&self, label: impl AsRef<str>, display: impl DebugDisplay);

            /// Renders a collapsing header with the given `label` and the
            /// contents of `display` nested beneath it, or a single line of
            /// text if `display` is `None`.
            ///
            /// This automatically creates a unique ID for the header so that
            /// its collapsed state doesn't collide with other headers.
            fn nested_opt(&self, label: impl AsRef<str>, display: Option<impl DebugDisplay>);
        }

        impl DisplayUiExt for ::hudhook::imgui::Ui {
            fn nested(&self, label: impl AsRef<str>, display: impl DebugDisplay) {
                ::debug::UiExt::header(self, label, || display.render_debug(self));
            }

            fn nested_opt(&self, label: impl AsRef<str>, display: Option<impl DebugDisplay>) {
                if let Some(display) = display {
                    self.nested(label, &display);
                } else {
                    self.text(format!("{}: None", label.as_ref()));
                }
            }
        }
    };
}
