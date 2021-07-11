mod explorer;

use bevy::{core::FixedTimestep, prelude::*};
use explorer::*;

#[creator::creator_main]
pub fn main() {
    println!("Initialization");
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_resource(WindowDescriptor {
            title: "{{project-name}}".to_string(),
            width: 320.0,
            height: 600.0,
            ..Default::default()
        })
        .add_resource(ExplorerStateChannel::new())
        .add_startup_system(explorer_startup.system())
        .add_startup_system(explorer_ui.system())
        .add_stage_after(
            stage::UPDATE,
            "substrate_update",
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::steps_per_second(1.0))
                .with_system(explorer_text_updater.system()),
        )
        .run();
}
