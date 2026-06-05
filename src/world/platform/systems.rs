use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier2d::prelude::*;

use super::components::Platform;

#[derive(SystemParam)]
pub struct PlatformHook<'w, 's> {
    platforms: Query<'w, 's, &'static Platform>,
}

impl<'w, 's> BevyPhysicsHooks for PlatformHook<'w, 's> {
    fn modify_solver_contacts(&self, ctx: ContactModificationContextView) {
        let platform = if let Ok(p) = self.platforms.get(ctx.collider1()) {
            p
        } else if let Ok(p) = self.platforms.get(ctx.collider2()) {
            p
        } else {
            return;
        };


        ctx.raw
            .update_as_oneway_platform(-Vec2::Y, platform.allowed_angle);
    }
}
