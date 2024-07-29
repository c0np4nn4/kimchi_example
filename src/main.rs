use kimchi::circuits::lookup::runtime_tables::RuntimeTable;
use kimchi::circuits::polynomials::generic::GenericGateSpec;
use kimchi::groupmap::{BWParameters, GroupMap};
use kimchi::mina_curves::pasta::{Fp, Vesta, VestaParameters};
use kimchi::mina_poseidon::{
    constants::PlonkSpongeConstantsKimchi,
    sponge::{DefaultFqSponge, DefaultFrSponge},
};
use kimchi::poly_commitment::{commitment::CommitmentCurve, evaluation_proof::OpeningProof};
use kimchi::{
    circuits::{gate::CircuitGate, wires::Wire},
    proof::ProverProof,
    prover_index::ProverIndex,
    verifier_index::VerifierIndex,
};
use std::sync::Arc;

type SpongeParams = PlonkSpongeConstantsKimchi;
type BaseSponge = DefaultFqSponge<VestaParameters, SpongeParams>;
type ScalarSponge = DefaultFrSponge<Fp, SpongeParams>;

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
    let mut gates = vec![];

    // 첫 번째 게이트: ax
    let wires_mul = Wire::for_row(0);
    let mul_gate = CircuitGate::create_generic_gadget(
        wires_mul.clone(),
        GenericGateSpec::Mul {
            output_coeff: None,
            mul_coeff: Some(a),
        },
        None,
    );
    gates.push(mul_gate);

    // 두 번째 게이트: ax + b
    let wires_add = Wire::for_row(1);
    let add_gate = CircuitGate::create_generic_gadget(
        wires_add.clone(),
        GenericGateSpec::Add {
            left_coeff: None,
            right_coeff: Some(b),
            output_coeff: None,
        },
        None,
    );
    gates.push(add_gate);

    let index = create_prover_index(gates);
    let verifier_index = index.verifier_index();

    // group map 설정
    let group_map = <Vesta as CommitmentCurve>::Map::setup();

    // create witness
    let ax = a * x;
    let ax_plus_b = ax + b;
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

    let is_valid =
        kimchi::verifier::batch_verify::<Vesta, BaseSponge, ScalarSponge, OpeningProof<Vesta>>(
            &group_map,
            &[ctx],
        )
        .unwrap();

    // println!("Proof verification failed?: {}", is_valid);
}

fn main() {
    // a, b, x 값을 설정합니다.
    let a = Fp::from(3);
    let b = Fp::from(5);
    let x = Fp::from(7);

    // 증명을 생성합니다.
    let (proof, public_input, verifier_index) = create_linear_proof(a, b, x);

    // 생성된 증명을 검증합니다.
    verify_linear_proof(proof, public_input, verifier_index);

    println!("Proof successfully verified!");
}
