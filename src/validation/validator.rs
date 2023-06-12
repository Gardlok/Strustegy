
use crate::validation::error::ValidationError;

use std::any::{Any, TypeId};
use std::marker::PhantomData;

use std::error::Error;

use crate::validation::validity::Validity;
// use crate::validation::target::Target;
use crate::validation::strategy::{Strategy, GenericStrategy, GenericProof};
use crate::validation::proof::{Proof, };
use crate::validation::logic::Scope;

use crate::validation::logic::CompositionOperator;





