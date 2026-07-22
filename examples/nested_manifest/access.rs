//! Example-local nested indexing helpers and index aliases.

use strustegy::{Get, HList, Here, There};

pub type HeaderIndex = Here;
pub type InputGroupIndex = There<Here>;
pub type IdentityGroupIndex = There<There<Here>>;
pub type ModeIndex = There<There<There<Here>>>;
pub type ArtifactGroupIndex = There<There<There<There<Here>>>>;
pub type LimitsGroupIndex = There<There<There<There<There<Here>>>>>;
pub type SummaryIndex = There<There<There<There<There<There<Here>>>>>>;

pub type FirstIndex = Here;
pub type SecondIndex = There<Here>;

pub trait Get2Ext: HList {
    fn get2<'a, OuterIndex, InnerIndex>(
        &'a self,
    ) -> &'a <<Self as Get<OuterIndex>>::Output as Get<InnerIndex>>::Output
    where
        Self: Get<OuterIndex>,
        <Self as Get<OuterIndex>>::Output: Get<InnerIndex> + 'a,
    {
        let outer = <Self as Get<OuterIndex>>::get(self);
        <<Self as Get<OuterIndex>>::Output as Get<InnerIndex>>::get(outer)
    }
}

impl<List: HList> Get2Ext for List {}
