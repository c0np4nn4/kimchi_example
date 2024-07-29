use kimchi::mina_curves::pasta::Fp;

use prover::create_linear_proof;
use verifier::verify_linear_proof;

pub mod circuit;
pub mod prover;
pub mod verifier;

fn main() {
    // a, b, x 값을 설정합니다.
    let a = Fp::from(3);
    let b = Fp::from(5);
    let x = Fp::from(7);

    // 증명을 생성합니다.
    let (proof, public_input, verifier_index) = create_linear_proof(a, b, x);

    // public_input is empty now
    println!("public_input: {:?}", public_input);

    // 생성된 증명을 검증합니다.
    verify_linear_proof(proof, public_input, verifier_index);

    println!("Proof successfully verified!");
}
