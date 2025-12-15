//! Verifying
use ff::{Field, PrimeField};
use group::{prime::PrimeCurveAffine, Curve, UncompressedEncoding};
use merlin::Transcript;
use pairing::{MillerLoopResult, MultiMillerLoop};
use std::ops::{AddAssign, Neg};

use super::{merlin_rng, PreparedVerifyingKey, Proof, VerifyingKey};

use crate::VerificationError;

/// Preprocessing on a verifying key to make verification faster.
pub fn prepare_verifying_key<E: MultiMillerLoop>(vk: &VerifyingKey<E>) -> PreparedVerifyingKey<E> {
    let gamma = vk.gamma_g2.neg();
    let neg_deltas_g2: Vec<E::G2Prepared> = vk.deltas_g2.iter().map(|d| d.neg().into()).collect();

    PreparedVerifyingKey {
        alpha_g1_beta_g2: E::pairing(&vk.alpha_g1, &vk.beta_g2),
        neg_gamma_g2: gamma.into(),
        neg_deltas_g2,
        ic: vk.ic.clone(),
        transcript: vk.transcript.clone(),
    }
}

/// Verify a Mirage proof given a prepared verifying key and public inputs.
///
/// Note that this also verifies that the proof is respect to a witness whose
/// aux blocks are committed to as the commitments stored in `proof` (see
/// [super::Proof] for where those commitments are).
pub fn verify_proof<'a, E: MultiMillerLoop>(
    pvk: &'a PreparedVerifyingKey<E>,
    proof: &Proof<E>,
    public_inputs: &[E::Fr],
) -> Result<(), VerificationError> {
    let mut transcript = Transcript::new(b"mirage_aozdemir_1");
    let mut acc = pvk.ic[0].to_curve();
    transcript.append_message(b"input", E::Fr::from(1).to_repr().as_ref());

    let mut public_inputs_i = 0;
    let mut aux_commits_i = 0;
    let mut i = 1;
    for t in &pvk.transcript {
        match t {
            crate::mirage::TranscriptEntry::Coin => {
                let mut rng = merlin_rng(&mut transcript, b"random");
                let coin = E::Fr::random(&mut *rng);
                transcript.append_message(b"input", coin.to_repr().as_ref());
                AddAssign::<&E::G1>::add_assign(&mut acc, &(pvk.ic[i] * coin));
                i += 1;
            }
            crate::mirage::TranscriptEntry::PublicInput => {
                AddAssign::<&E::G1>::add_assign(
                    &mut acc,
                    &(pvk.ic[i] * public_inputs[public_inputs_i]),
                );
                transcript
                    .append_message(b"input", public_inputs[public_inputs_i].to_repr().as_ref());
                public_inputs_i += 1;
                i += 1;
            }
            crate::mirage::TranscriptEntry::AuxCommit => {
                transcript.append_message(
                    b"aux_commit",
                    proof.ds[aux_commits_i].to_uncompressed().as_ref(),
                );
                aux_commits_i += 1;
            }
        }
    }
    if i != pvk.ic.len() || aux_commits_i != proof.ds.len() {
        return Err(VerificationError::InvalidVerifyingKey);
    }

    // The original verification equation is:
    // A * B = alpha * beta + inputs * gamma + C * delta
    // ... however, we rearrange it so that it is:
    // A * B - inputs * gamma - C * delta = alpha * beta
    // or equivalently:
    // A * B + inputs * (-gamma) + C * (-delta) = alpha * beta
    // which allows us to do a single final exponentiation.

    let b = proof.b.into();
    let acc = acc.to_affine();
    let last = pvk.neg_deltas_g2.len() - 1;
    let mut multi_miller_input = vec![
        (&proof.a, &b),
        (&acc, &pvk.neg_gamma_g2),
        (&proof.c, &pvk.neg_deltas_g2[last]),
    ];
    assert_eq!(pvk.neg_deltas_g2.len(), proof.ds.len() + 1);
    for (i, d) in proof.ds.iter().enumerate() {
        multi_miller_input.push((d, &pvk.neg_deltas_g2[i]));
    }
    if pvk.alpha_g1_beta_g2 == E::multi_miller_loop(&multi_miller_input).final_exponentiation() {
        Ok(())
    } else {
        Err(VerificationError::InvalidProof)
    }
}
