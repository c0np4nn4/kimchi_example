use ark_ff::Fp256;
use kimchi::circuits::lookup::runtime_tables::RuntimeTable;
use kimchi::groupmap::{BWParameters, GroupMap};
use kimchi::mina_curves::pasta::fields::FpParameters;
use kimchi::mina_curves::pasta::{Fp, Vesta, VestaParameters};
use kimchi::mina_poseidon::constants::PlonkSpongeConstantsKimchi;
use kimchi::mina_poseidon::sponge::{DefaultFqSponge, DefaultFrSponge};
use kimchi::poly_commitment::{commitment::CommitmentCurve, evaluation_proof::OpeningProof};
use kimchi::{
    circuits::gate::CircuitGate, proof::ProverProof, prover_index::ProverIndex,
    verifier_index::VerifierIndex,
};

type SpongeParams = PlonkSpongeConstantsKimchi;
type BaseSponge = DefaultFqSponge<VestaParameters, SpongeParams>;
type ScalarSponge = DefaultFrSponge<Fp, SpongeParams>;

use crate::circuit::create_linear_circuit;

pub fn create_prover_index(gates: Vec<CircuitGate<Fp>>) -> ProverIndex<Vesta, OpeningProof<Vesta>> {
    kimchi::prover_index::testing::new_index_for_test::<Vesta>(gates, 0)
}

pub fn create_linear_proof(
    a: Fp,
    b: Fp,
    x: Fp,
) -> (
    ProverProof<Vesta, OpeningProof<Vesta>>,
    Vec<Fp>,
    VerifierIndex<Vesta, OpeningProof<Vesta>>,
) {
    let gates: Vec<CircuitGate<Fp256<FpParameters>>> = create_linear_circuit(a, b);

    let index = create_prover_index(gates);

    let verifier_index = index.verifier_index();

    // group map 설정
    let group_map = <Vesta as CommitmentCurve>::Map::setup();

    // create witness
    let ax = a * x;
    let ax_plus_b = ax + b;

    // witness
    // ---------------------------------
    // |  x   |  1  |    ax    |  ...  | mul_coeff: `a`
    // ---------------------------------
    // |  ax  |  1  |  ax + b  |  ...  | right_coeff: `b`
    // ---------------------------------
    let witness: [Vec<Fp>; kimchi::circuits::wires::COLUMNS] = [
        vec![x, ax],
        vec![Fp::from(1), Fp::from(1)],
        vec![ax, ax_plus_b],
        vec![Fp::from(0); 2],
        vec![Fp::from(0); 2],
        vec![Fp::from(0); 2],
        vec![Fp::from(0); 2],
        vec![Fp::from(0); 2],
        vec![Fp::from(0); 2],
        vec![Fp::from(0); 2],
        vec![Fp::from(0); 2],
        vec![Fp::from(0); 2],
        vec![Fp::from(0); 2],
        vec![Fp::from(0); 2],
        vec![Fp::from(0); 2],
    ];

    // 빈 runtime_tables 사용
    let runtime_tables: &[RuntimeTable<Fp>] = &[];

    let proof = ProverProof::create::<BaseSponge, ScalarSponge>(
        &group_map,
        witness,
        runtime_tables,
        &index,
    )
    .unwrap();

    (proof, vec![], verifier_index)
}
