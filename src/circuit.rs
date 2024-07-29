use kimchi::{
    circuits::{gate::CircuitGate, polynomials::generic::GenericGateSpec, wires::Wire},
    mina_curves::pasta::Fp,
};

pub(crate) fn create_linear_circuit(a: Fp, b: Fp) -> Vec<CircuitGate<Fp>> {
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

    gates
}
