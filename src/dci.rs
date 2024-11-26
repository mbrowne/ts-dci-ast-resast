use std::fmt::Debug;

use crate::{Ident, Prop, IntoAllocated};

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Role<T> {
    pub id: Option<Ident<T>>,
    pub body: RoleBody<T>,
}

impl<T> IntoAllocated for Role<T>
where
    T: ToString,
{
    type Allocated = Role<String>;

    fn into_allocated(self) -> Self::Allocated {
        Role {
            id: self.id.map(IntoAllocated::into_allocated),
            body: self.body.into_allocated(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RoleBody<T>(pub Vec<Prop<T>>);

impl<T> IntoAllocated for RoleBody<T>
where
    T: ToString,
{
    type Allocated = RoleBody<String>;

    fn into_allocated(self) -> Self::Allocated {
        RoleBody(
            self.0
                .into_iter()
                .map(IntoAllocated::into_allocated)
                .collect(),
        )
    }
}
