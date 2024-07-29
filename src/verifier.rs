use kimchi::groupmap::{BWParameters, GroupMap};
use kimchi::mina_curves::pasta::{Fp, Vesta, VestaParameters};
use kimchi::mina_poseidon::{
    constants::PlonkSpongeConstantsKimchi,
    sponge::{DefaultFqSponge, DefaultFrSponge},
};
use kimchi::poly_commitment::{commitment::CommitmentCurve, evaluation_proof::OpeningProof};
use kimchi::{proof::ProverProof, verifier_index::VerifierIndex};

type SpongeParams = PlonkSpongeConstantsKimchi;
type BaseSponge = DefaultFqSponge<VestaParameters, SpongeParams>;
type ScalarSponge = DefaultFrSponge<Fp, SpongeParams>;

pub fn verify_linear_proof(
    proof: ProverProof<Vesta, OpeningProof<Vesta>>,
    public_input: Vec<Fp>,
    verifier_index: VerifierIndex<Vesta, OpeningProof<Vesta>>,
) {
    let group_map = <Vesta as CommitmentCurve>::Map::setup();

    let ctx = kimchi::verifier::Context {
        verifier_index: &verifier_index,
        proof: &proof,
        public_input: &public_input,
    };

    let _is_valid =
        kimchi::verifier::batch_verify::<Vesta, BaseSponge, ScalarSponge, OpeningProof<Vesta>>(
            &group_map,
            &[ctx],
        )
        .unwrap();
}
