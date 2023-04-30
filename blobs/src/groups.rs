use crate::*;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[repr(C)]
pub struct InteractionGroups {
    pub memberships: Group,
    pub filter: Group,
}

impl InteractionGroups {
    /// Initializes with the given interaction groups and interaction mask.
    pub const fn new(memberships: Group, filter: Group) -> Self {
        Self {
            memberships,
            filter,
        }
    }

    /// Allow interaction with everything.
    pub const fn all() -> Self {
        Self::new(Group::ALL, Group::ALL)
    }

    /// Prevent all interactions.
    pub const fn none() -> Self {
        Self::new(Group::NONE, Group::NONE)
    }

    /// Sets the group this filter is part of.
    pub const fn with_memberships(mut self, memberships: Group) -> Self {
        self.memberships = memberships;
        self
    }

    /// Sets the interaction mask of this filter.
    pub const fn with_filter(mut self, filter: Group) -> Self {
        self.filter = filter;
        self
    }

    /// Check if interactions should be allowed based on the interaction
    /// memberships and filter.
    ///
    /// An interaction is allowed iff. the memberships of `self` contain at
    /// least one bit set to 1 in common with the filter of `rhs`, and
    /// vice-versa.
    #[inline]
    pub const fn test(self, rhs: Self) -> bool {
        // NOTE: since const ops is not stable, we have to convert `Group` into
        // u32 to use & operator in const context.
        (self.memberships.bits() & rhs.filter.bits()) != 0
            && (rhs.memberships.bits() & self.filter.bits()) != 0
    }
}

impl Default for InteractionGroups {
    fn default() -> Self {
        Self::all()
    }
}

use bitflags::bitflags;

bitflags! {
    pub struct Group: u32 {
        const GROUP_1 = 1 << 0;
        const GROUP_2 = 1 << 1;
        const GROUP_3 = 1 << 2;
        const GROUP_4 = 1 << 3;
        const GROUP_5 = 1 << 4;
        const GROUP_6 = 1 << 5;
        const GROUP_7 = 1 << 6;
        const GROUP_8 = 1 << 7;
        const GROUP_9 = 1 << 8;
        const GROUP_10 = 1 << 9;
        const GROUP_11 = 1 << 10;
        const GROUP_12 = 1 << 11;
        const GROUP_13 = 1 << 12;
        const GROUP_14 = 1 << 13;
        const GROUP_15 = 1 << 14;
        const GROUP_16 = 1 << 15;
        const GROUP_17 = 1 << 16;
        const GROUP_18 = 1 << 17;
        const GROUP_19 = 1 << 18;
        const GROUP_20 = 1 << 19;
        const GROUP_21 = 1 << 20;
        const GROUP_22 = 1 << 21;
        const GROUP_23 = 1 << 22;
        const GROUP_24 = 1 << 23;
        const GROUP_25 = 1 << 24;
        const GROUP_26 = 1 << 25;
        const GROUP_27 = 1 << 26;
        const GROUP_28 = 1 << 27;
        const GROUP_29 = 1 << 28;
        const GROUP_30 = 1 << 29;
        const GROUP_31 = 1 << 30;
        const GROUP_32 = 1 << 31;

        const ALL = u32::MAX;
        const NONE = 0;
    }
}

impl From<u32> for Group {
    #[inline]
    fn from(val: u32) -> Self {
        unsafe { Self::from_bits_unchecked(val) }
    }
}

impl From<Group> for u32 {
    #[inline]
    fn from(val: Group) -> Self {
        val.bits()
    }
}
