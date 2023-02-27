use crate::*;

bitflags::bitflags! {
    #[derive(Default)]
    /// Flags for excluding whole sets of colliders from a scene query.
    pub struct QueryFilterFlags: u32 {
        /// Exclude from the query any collider attached to a fixed rigid-body and colliders with no rigid-body attached.
        const EXCLUDE_FIXED = 1 << 1;
        /// Exclude from the query any collider attached to a dynamic rigid-body.
        const EXCLUDE_KINEMATIC = 1 << 2;
        /// Exclude from the query any collider attached to a kinematic rigid-body.
        const EXCLUDE_DYNAMIC = 1 << 3;
        /// Exclude from the query any collider that is a sensor.
        const EXCLUDE_SENSORS = 1 << 4;
        /// Exclude from the query any collider that is not a sensor.
        const EXCLUDE_SOLIDS = 1 << 5;
        /// Excludes all colliders not attached to a dynamic rigid-body.
        const ONLY_DYNAMIC = Self::EXCLUDE_FIXED.bits | Self::EXCLUDE_KINEMATIC.bits;
        /// Excludes all colliders not attached to a kinematic rigid-body.
        const ONLY_KINEMATIC = Self::EXCLUDE_DYNAMIC.bits | Self::EXCLUDE_FIXED.bits;
        /// Exclude all colliders attached to a non-fixed rigid-body
        /// (this will not exclude colliders not attached to any rigid-body).
        const ONLY_FIXED = Self::EXCLUDE_DYNAMIC.bits | Self::EXCLUDE_KINEMATIC.bits;
    }
}

impl QueryFilterFlags {
    /// Tests if the given collider should be taken into account by a scene query, based
    /// on the flags on `self`.
    #[inline]
    pub fn test(&self, _bodies: &RigidBodySet, _collider: &Collider) -> bool {
        todo!()
        // if self.is_empty() {
        //     // No filter.
        //     return true;
        // }
        //
        // if (self.contains(QueryFilterFlags::EXCLUDE_SENSORS)
        //     && collider.is_sensor())
        //     || (self.contains(QueryFilterFlags::EXCLUDE_SOLIDS)
        //         && !collider.is_sensor())
        // {
        //     return false;
        // }
        //
        // if self.contains(QueryFilterFlags::EXCLUDE_FIXED)
        //     && collider.parent.is_none()
        // {
        //     return false;
        // }
        //
        // if let Some(parent) = collider.parent.and_then(|p| bodies.get(p.handle))
        // {
        //     let parent_type = parent.body_type();
        //
        //     if (self.contains(QueryFilterFlags::EXCLUDE_FIXED)
        //         && parent_type.is_fixed())
        //         || (self.contains(QueryFilterFlags::EXCLUDE_KINEMATIC)
        //             && parent_type.is_kinematic())
        //         || (self.contains(QueryFilterFlags::EXCLUDE_DYNAMIC)
        //             && parent_type.is_dynamic())
        //     {
        //         return false;
        //     }
        // }
        //
        // true
    }
}

/// A filter tha describes what collider should be included or excluded from a scene query.
#[derive(Copy, Clone, Default)]
pub struct QueryFilter<'a> {
    /// Flags indicating what particular type of colliders should be excluded from the scene query.
    pub flags: QueryFilterFlags,
    /// If set, only colliders with collision groups compatible with this one will
    /// be included in the scene query.
    pub groups: Option<InteractionGroups>,
    /// If set, this collider will be excluded from the scene query.
    pub exclude_collider: Option<ColliderHandle>,
    /// If set, any collider attached to this rigid-body will be excluded from the scene query.
    pub exclude_rigid_body: Option<RigidBodyHandle>,
    /// If set, any collider for which this closure returns false will be excluded from the scene query.
    pub predicate: Option<&'a dyn Fn(ColliderHandle, &Collider) -> bool>,
}

impl<'a> QueryFilter<'a> {
    /// Applies the filters described by `self` to a collider to determine if it has to be
    /// included in a scene query (`true`) or not (`false`).
    #[inline]
    pub fn test(
        &self,
        _bodies: &RigidBodySet,
        _handle: ColliderHandle,
        _collider: &Collider,
    ) -> bool {
        todo!()
        // self.exclude_collider != Some(handle)
        //     && (self.exclude_rigid_body.is_none() // NOTE: deal with the `None` case separately otherwise the next test is incorrect if the collider’s parent is `None` too.
        //         || self.exclude_rigid_body != collider.parent.map(|p| p.handle))
        //     && self
        //         .groups
        //         .map(|grps| collider.flags.collision_groups.test(grps))
        //         .unwrap_or(true)
        //     && self.flags.test(bodies, collider)
        //     && self.predicate.map(|f| f(handle, collider)).unwrap_or(true)
    }
}

impl<'a> From<QueryFilterFlags> for QueryFilter<'a> {
    fn from(flags: QueryFilterFlags) -> Self {
        Self {
            flags,
            ..QueryFilter::default()
        }
    }
}

impl<'a> From<InteractionGroups> for QueryFilter<'a> {
    fn from(groups: InteractionGroups) -> Self {
        Self {
            groups: Some(groups),
            ..QueryFilter::default()
        }
    }
}

impl<'a> QueryFilter<'a> {
    /// A query filter that doesn’t exclude any collider.
    pub fn new() -> Self {
        Self::default()
    }

    /// Exclude from the query any collider attached to a fixed rigid-body and colliders with no rigid-body attached.
    pub fn exclude_fixed() -> Self {
        QueryFilterFlags::EXCLUDE_FIXED.into()
    }

    /// Exclude from the query any collider attached to a dynamic rigid-body.
    pub fn exclude_kinematic() -> Self {
        QueryFilterFlags::EXCLUDE_KINEMATIC.into()
    }

    /// Exclude from the query any collider attached to a kinematic rigid-body.
    pub fn exclude_dynamic() -> Self {
        QueryFilterFlags::EXCLUDE_DYNAMIC.into()
    }

    /// Excludes all colliders not attached to a dynamic rigid-body.
    pub fn only_dynamic() -> Self {
        QueryFilterFlags::ONLY_DYNAMIC.into()
    }

    /// Excludes all colliders not attached to a kinematic rigid-body.
    pub fn only_kinematic() -> Self {
        QueryFilterFlags::ONLY_KINEMATIC.into()
    }

    /// Exclude all colliders attached to a non-fixed rigid-body
    /// (this will not exclude colliders not attached to any rigid-body).
    pub fn only_fixed() -> Self {
        QueryFilterFlags::ONLY_FIXED.into()
    }

    /// Exclude from the query any collider that is a sensor.
    pub fn exclude_sensors(mut self) -> Self {
        self.flags |= QueryFilterFlags::EXCLUDE_SENSORS;
        self
    }

    /// Exclude from the query any collider that is not a sensor.
    pub fn exclude_solids(mut self) -> Self {
        self.flags |= QueryFilterFlags::EXCLUDE_SOLIDS;
        self
    }

    /// Only colliders with collision groups compatible with this one will
    /// be included in the scene query.
    pub fn groups(mut self, groups: InteractionGroups) -> Self {
        self.groups = Some(groups);
        self
    }

    /// Set the collider that will be excluded from the scene query.
    pub fn exclude_collider(mut self, collider: ColliderHandle) -> Self {
        self.exclude_collider = Some(collider);
        self
    }

    /// Set the rigid-body that will be excluded from the scene query.
    pub fn exclude_rigid_body(mut self, rigid_body: RigidBodyHandle) -> Self {
        self.exclude_rigid_body = Some(rigid_body);
        self
    }

    /// Set the predicate to apply a custom collider filtering during the scene query.
    pub fn predicate(
        mut self,
        predicate: &'a impl Fn(ColliderHandle, &Collider) -> bool,
    ) -> Self {
        self.predicate = Some(predicate);
        self
    }
}
