use crate::display::{DebugDisplay, DisplayUiExt};
use debug::UiExt;
use eldenring::cs::{CSChrRideModule, CSPairAnimNode, CSRideNode};
use hudhook::imgui::Ui;

impl DebugDisplay for CSChrRideModule {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("CSRideNode", &self.ride_node);
        ui.debug("Last mounted", self.last_mounted);
        ui.display("Has ride param", self.has_ride_param);
        ui.display("Is ridden character", self.is_ride_character);
        ui.display("Mount rotation", self.mount_data.rotation);
        ui.display("Mount position", self.mount_data.mount_position);
        ui.display("Mount velocity", self.mount_data.velocity);
        ui.display("Attack direction", self.mount_data.attack_direction);
        ui.display(
            "Attack received damage type",
            self.mount_data.received_damage_type,
        );
        ui.display("Mount health", self.mount_data.mount_health);
        ui.display("Fall height", self.mount_data.fall_height);
        ui.display(
            "Is touching solid ground",
            self.mount_data.is_touching_solid_ground,
        );
        ui.display("Is falling", self.mount_data.is_falling);
        ui.display("Is sliding", self.mount_data.is_sliding);
        ui.display("Is mounting", self.is_mounting);
        ui.display("Is mounted", self.is_mounted);
    }
}

impl DebugDisplay for CSPairAnimNode {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Counter party", self.counter_party);
        ui.display("Start position", self.start_position);
        ui.display("Start rotation", self.start_rotation);
    }
}

impl DebugDisplay for CSRideNode {
    fn render_debug(&self, ui: &Ui) {
        self.pair_anim_node.render_debug(ui);
        ui.display("Ride state", self.ride_state);
        ui.display("Ride param ID", self.ride_param_id);
        ui.display("Camera mount control", self.camera_mount_control);
    }
}
