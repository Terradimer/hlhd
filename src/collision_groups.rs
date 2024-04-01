use bevy_rapier3d::geometry::{CollisionGroups, Group};

pub struct Groups;

impl Groups {
    pub const PLAYER: Group = Group::GROUP_1;
    pub const ENVIRONMENT: Group = Group::GROUP_2;
    pub const KICKABLE: Group = Group::GROUP_3;
    pub const PROJECTILES: Group = Group::GROUP_4;
    pub const NONE: Group = Group::empty();

    // Helper methods to create collision groups for different entities
    pub fn player() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::PLAYER,
            filters: Self::ENVIRONMENT,
        }
    }

    pub fn environment() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::ENVIRONMENT,
            filters: Self::PLAYER | Self::PROJECTILES,
        }
    }

    pub fn kickable() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::KICKABLE,
            filters: Self::PLAYER,
        }
    }
}
