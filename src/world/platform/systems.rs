use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_rapier2d::prelude::*;

use super::components::Platform;

#[derive(SystemParam)]
pub struct PlatformHook<'w, 's> {
    platforms: Query<'w, 's, &'static Platform>,
}

impl<'w, 's> BevyPhysicsHooks for PlatformHook<'w, 's> {
    fn modify_solver_contacts(&self, ctx: ContactModificationContextView) {
        let (platform, normal) = if let Ok(p) = self.platforms.get(ctx.collider1()) {
            (p, Vec2::Y)
        } else if let Ok(p) = self.platforms.get(ctx.collider2()) {
            (p, -Vec2::Y)
        } else {
            return;
        };


        ctx.raw
            .update_as_oneway_platform(normal, platform.allowed_angle);
    }
}
