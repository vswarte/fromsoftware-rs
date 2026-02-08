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
                    Ok(instance) => {
                        ::debug::UiExt::header(ui, &T::name(), || {
                            // Render this as a text input so it's easy to
                            // copy/paste the address into a debugger.
                            let mut address = format!("{instance:p}");
                            let label = format!("{} address", T::name());
                            ui.input_text(&label, &mut address).read_only(true).build();

                            instance.render_debug_mut(ui, &mut self.state);
                        })
                    }
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
    };
}
