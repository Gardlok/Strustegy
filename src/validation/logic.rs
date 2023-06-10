


use std::fmt::{Debug, Display};
use std::error::Error;
use crossbeam::atomic::AtomicCell;
use crossbeam_skiplist::SkipMap as TreeMap;

use std::any::TypeId;


//////////////////////////////////////////////////////
// Contextualizing the ecosystem for Strategy patterns
//
//



// Target level context
//
pub struct TargetContext<T: 'static + Sync + Send + Clone> {
    pub type_id: TypeId,  // Any TypeId for lookups 
    pub priority: u32,    // For sorting and Validation logicistics
    pub omitted: bool,    // All purpose omit switch
    pub value: T,         // 
}





