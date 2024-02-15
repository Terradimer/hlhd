use bevy_rapier2d::geometry::{CollisionGroups, Group};

pub struct GroupsConfig;

impl GroupsConfig {
    pub const PLAYER: Group = Group::GROUP_1;
    pub const GROUND: Group = Group::GROUP_2;
    pub const KICKABLE: Group = Group::GROUP_3;
    pub const PROJECTILES: Group = Group::GROUP_4;

    // Helper methods to create collision groups for different entities
    pub fn player_group() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::PLAYER,
            filters: Self::GROUND,
        } // Player interacts with ground
    }

    pub fn ground_group() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::GROUND,
            filters: Self::PLAYER | Self::PROJECTILES,
        } // Ground interacts with player
    }

    pub fn kickable_group() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::KICKABLE,
            filters: Self::PLAYER,
        } // Kickable interacts with player
    }
}
