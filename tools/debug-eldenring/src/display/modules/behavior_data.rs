use crate::display::DebugDisplay;
use debug::UiExt;
use eldenring::cs::CSChrBehaviorDataModule;
use hudhook::imgui::{TableColumnSetup, Ui};

impl DebugDisplay for CSChrBehaviorDataModule {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Has twist modifier", self.has_twist_modifier);
        ui.display("Fixed rotation direction", self.fixed_rotation_direction);
        ui.display("Min twist rank", self.min_twist_rank);
        ui.display("HKS root motion multiplier", self.hks_root_motion_mult);
        ui.display("Turn speed", self.turn_speed);
        ui.display(
            "HKS animation speed multiplier",
            self.hks_animation_speed_multiplier,
        );

        ui.header("Twist modifiers", || {
            ui.table(
                "behavior-data-twist-modifiers",
                [
                    TableColumnSetup::new("ID"),
                    TableColumnSetup::new("Target"),
                    TableColumnSetup::new("Rank"),
                    TableColumnSetup::new("Limits (U/D/L/R)"),
                    TableColumnSetup::new("Minimums (U/D/L/R)"),
                ],
                self.twist_modifiers.iter(),
                |ui, _i, modifier| {
                    ui.table_next_column();
                    ui.text(modifier.modifier_id.to_string());

                    ui.table_next_column();
                    ui.text(modifier.target_type.to_string());

                    ui.table_next_column();
                    ui.text(modifier.rank.to_string());

                    ui.table_next_column();
                    ui.text(format!(
                        "{:.2}/{:.2}/{:.2}/{:.2}",
                        modifier.up_limit_angle,
                        modifier.down_limit_angle,
                        modifier.left_limit_angle,
                        modifier.right_limit_angle
                    ));

                    ui.table_next_column();
                    ui.text(format!(
                        "{:.2}/{:.2}/{:.2}/{:.2}",
                        modifier.up_minimum_angle,
                        modifier.down_minimum_angle,
                        modifier.left_minimum_angle,
                        modifier.right_minimum_angle
                    ));
                },
            );
        });
    }
}
