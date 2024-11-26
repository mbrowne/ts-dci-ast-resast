use super::{Ident, Node, SourceLocation};
use crate::expr::Prop;
use crate::IntoAllocated;
use crate::spanned::{
    tokens,
    tokens::{
        CloseBrace, OpenBrace, Token,
    }
};


#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Role<T> {
    pub keyword: tokens::Role,
    pub id: Option<Ident<T>>,
    pub body: RoleBody<T>,
}

impl<T> IntoAllocated for Role<T>
where
    T: ToString,
{
    type Allocated = Role<String>;
    fn into_allocated(self) -> Role<String> {
        Role {
            keyword: self.keyword,
            id: self.id.map(|i| i.into_allocated()),
            body: self.body.into_allocated(),
        }
    }
}

impl<T> Node for Role<T> {
    fn loc(&self) -> SourceLocation {
        SourceLocation {
            start: self.keyword.start(),
            end: self.body.close_brace.end(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RoleBody<T> {
    pub open_brace: OpenBrace,
    pub props: Vec<Prop<T>>,
    pub close_brace: CloseBrace,
}

impl<T> IntoAllocated for RoleBody<T>
where
    T: ToString,
{
    type Allocated = RoleBody<String>;
    fn into_allocated(self) -> RoleBody<String> {
        RoleBody {
            open_brace: self.open_brace,
            props: self
                .props
                .into_iter()
                .map(IntoAllocated::into_allocated)
                .collect(),
            close_brace: self.close_brace,
        }
    }
}