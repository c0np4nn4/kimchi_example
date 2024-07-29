// use kimchi::{
//     circuits::{gate::CircuitGate, polynomials::generic::GenericGateSpec, wires::Wire},
//     mina_curves::pasta::Fp,
// };

// pub(crate) fn create_linear_circuit(a: Fp, b: Fp, num_gates: usize) -> Vec<CircuitGate<Fp>> {
//     let mut gates = vec![];

//     for row in 0..num_gates {
//         let wires = Wire::for_row(row);

//         // 첫 번째 게이트: ax
//         let mul_gate = CircuitGate::create_generic_gadget(
//             wires.clone(),
//             GenericGateSpec::Mul {
//                 output_coeff: None,
//                 mul_coeff: Some(a),
//             },
//             None,
//         );
//         gates.push(mul_gate);

//         // 두 번째 게이트: ax + b
//         let add_gate = CircuitGate::create_generic_gadget(
//             wires,
//             GenericGateSpec::Add {
//                 left_coeff: None,
//                 right_coeff: Some(b),
//                 output_coeff: None,
//             },
//             None,
//         );
//         gates.push(add_gate);
//     }
//     gates
// }

