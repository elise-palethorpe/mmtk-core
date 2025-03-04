//! GC algorithms from the MMTk suite.
//!
//! This module provides various GC plans, each of which implements a GC algorithm.
//! Generally a plan consists of a few parts:
//! * A plan type that implements the [`Plan`](crate::plan::Plan) trait, which defines
//!   spaces used in the plan, and their behaviors in GC and page accounting.
//! * A mutator definition, which describes the mapping between allocators and allocation semantics,
//!   and the mapping between allocators and spaces. If the plan needs barrier, the barrier definition is
//!   also included here.
//! * A constant for [`PlanConstraints`](crate::plan::PlanConstraints), which defines
//!   plan-specific constants.
//! * Plan-specific [`GCWork`](crate::scheduler::GCWork), which is scheduled during GC. If the plan
//!   implements a copying GC, a [`CopyContext`](crate::plan::CopyContext) also needs to be provided.
//!
//! For more about implementing a plan, it is recommended to read the [MMTk tutorial](/docs/tutorial/Tutorial.md).

mod barriers;
pub use barriers::BarrierSelector;

mod controller_collector_context;

mod global;
pub(crate) use global::create_mutator;
pub(crate) use global::create_plan;
pub use global::AllocationSemantics;
pub use global::CopyContext;
pub(crate) use global::GcStatus;
pub use global::Plan;

mod mutator_context;
pub use mutator_context::Mutator;
pub use mutator_context::MutatorContext;

mod plan_constraints;
pub use plan_constraints::PlanConstraints;

mod tracelocal;
pub use tracelocal::TraceLocal;

mod transitive_closure;
pub use transitive_closure::{ObjectsClosure, TransitiveClosure};

mod generational;
mod immix;
mod marksweep;
mod nogc;
mod pageprotect;
mod semispace;

// Expose plan constraints as public. Though a binding can get them from plan.constraints(),
// it is possible for performance reasons that they want the constraints as constants.

pub use generational::copying::GENCOPY_CONSTRAINTS;
pub use immix::IMMIX_CONSTRAINTS;
pub use marksweep::MS_CONSTRAINTS;
pub use nogc::NOGC_CONSTRAINTS;
pub use pageprotect::PP_CONSTRAINTS;
pub use semispace::SS_CONSTRAINTS;
