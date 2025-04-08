use std::{ptr::null, sync::Mutex};

use bevy::prelude::*;
use bevy::render::{Extract, ExtractSchedule, Render, RenderApp, RenderSet};

type RenderDoc = renderdoc::RenderDoc<renderdoc::V100>;

#[derive(Resource)]
struct RenderDocData {
    is_capture_active: bool,
    api: Mutex<RenderDoc>,
}

pub struct RenderdocPlugin;

impl Plugin for RenderdocPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let renderdoc = match RenderDoc::new() {
            Ok(r) => r,
            Err(e) => {
                debug!(
                    "Renderdoc could not be loaded, not registering capture hook: {}",
                    e
                );
                return;
            }
        };

        info!("Renderdoc available, registering capture hook");

        let sub_app = app.sub_app_mut(RenderApp);

        sub_app.add_systems(ExtractSchedule, start_capture);
        sub_app.add_systems(
            Render,
            after_render_end_capture
                .after(RenderSet::Render)
                .before(RenderSet::Cleanup),
        );
        sub_app.insert_resource(RenderDocData {
            is_capture_active: false,
            api: Mutex::new(renderdoc),
        });
    }
}

fn start_capture(
    keyboard: Extract<Res<ButtonInput<KeyCode>>>,
    mut renderdoc: ResMut<RenderDocData>,
) {
    if !keyboard.just_pressed(KeyCode::F10) {
        return;
    }

    renderdoc.is_capture_active = true;

    let api = renderdoc.api.get_mut().unwrap();
    api.start_frame_capture(null(), null());
}

fn after_render_end_capture(mut renderdoc: ResMut<RenderDocData>) {
    if !renderdoc.is_capture_active {
        return;
    }

    renderdoc.is_capture_active = false;

    let api = renderdoc.api.get_mut().unwrap();
    api.end_frame_capture(null(), null());
}
