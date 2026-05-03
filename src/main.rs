// support configuring bevy lints within code
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// disable console on windows for non-dev builds
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

pub(crate) mod prelude;

use crate::prelude::*;
use bevy::asset::AssetMetaCheck;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                // wasm builds will check for meta files (that don't exist) if this isn't set
                // this causes errors and even panics on web build on itch
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
    );

    app.run()
}
