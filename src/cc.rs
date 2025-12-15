//! A programming interface for CC-zkSNARKs of interactive proof.
//!
//! It has been enhanced (relative to R1CS) in two ways:
//!
//! 1) It supports commitment-carrying proofs (CC-zkSNARKs), which allow:
//!
//!    * the constraints system to mark sets of auxiliary variables as "blocks"
//!    * the prover to create proofs about a witness that satisfies the
//!      constraints, and also contains values for the blocks that agree with a
//!      public commitment.
//!
//!    Note that "CC" stands for "commitment-carrying", which is a synonym for
//!    "commit-and-prove".
//!
//! 2) It supports expressing (constant-round, public-coin) interactive proofs
//!    as constraint systems. The constraint system can allocate "random variables",
//!    which will be assigned values that are uniformly random and independent
//!    of the values of the prior aux blocks that the prover has committed to.
//!
//! From the application-level, these two extensions to the programming
//! interface seems quite different, but they are implemented similarly, which
//! is why we bundle them.
//!
//! We prefix the names of types and traits associated with this extension with
//! "CC" or `Cc`.
//!
//! See [Mirage](crate::mirage) for an example zkSNARK for this interface.
//!
//! See [KW15](crate::kw15) to link commitments created via this interface
//! against external generators.
use crate::{ConstraintSystem, SynthesisError, Variable};
use ff::PrimeField;

/// A constraint system builder for CC-zkSNARKs
/// (see the [module-level documentation](crate::cc) for an intro to CC-zkSNARKs).
///
/// The analogue of bellman's standard [ConstraintSystem] trait.
pub trait CcConstraintSystem<Scalar: PrimeField>: ConstraintSystem<Scalar> {
    /// Mark all auxiliary variables before this (but after the last block) as a
    /// new "block". A block is just a vector of auxiliary variables.
    ///
    /// In a CC-zkSNARK, the proof will include a pedersen commitment to the
    /// values of each block. The pedersen generators will be sampled during
    /// setup. Thus, if the setup's trust assumptions fail, the commitment does
    /// too. One can use a commitment-linking protocol to further bind the
    /// values to a pedersen commitment with different generators. For example,
    /// generators that were transparently generated. See [crate::kw15] for a
    /// linking protocol.
    ///
    /// In a zkSNARK for interactive proofs, each random variable allocated
    /// after the block will take a value that is independent of the block's
    /// value.
    fn end_aux_block<A, AR>(&mut self, annotation: A) -> Result<(), SynthesisError>
    where
        A: FnOnce() -> AR,
        AR: Into<String>;

    /// Allocate a random variable.
    ///
    /// ## Returns
    ///
    /// * the variable and
    /// * its scalar value, as an option that is filled only in proving mode
    fn alloc_random<A, AR>(
        &mut self,
        annotation: A,
    ) -> Result<(Variable, Option<Scalar>), SynthesisError>
    where
        A: FnOnce() -> AR,
        AR: Into<String>;
}

/// A constraint system description (and witness generator) for a CC-zkSNARK
/// (see the [module-level documentation](crate::cc) for an intro to CC-zkSNARKs).
///
/// The analogue of bellman's standard [Circuit](crate::Circuit) trait.
pub trait CcCircuit<Scalar: PrimeField> {
    /// Synthesize the constraint system
    fn synthesize<CS: CcConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError>;
    /// How many auxilary blocks will this circuit produce?
    fn num_aux_blocks(&self) -> usize;
}
