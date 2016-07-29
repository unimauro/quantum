//! Implementations of quantum gates, intended for consumer use.

use complex::Complex;

use gate::Gate;
use ket::Ket;
use matrix::Matrix;

/// The identity gate, not mutating the state at all.
#[allow(unused)]
pub fn identity(width: usize) -> Gate {
    let m = Matrix::identity(Ket::size(width));

    Gate::new(width, m)
}

/// The Hadamard gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Hadamard_transform#Quantum_computing_applications)
/// for more information.
#[allow(unused, trivial_numeric_casts)]
pub fn hadamard() -> Gate {
    let sqrt2inv = 2.0f64.sqrt().recip();

    let mut m = m_real![sqrt2inv,  sqrt2inv;
                        sqrt2inv, -sqrt2inv];

    Gate::new(1, m)
}

/// The Pauli-X gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Pauli-X_gate)
/// for more information.
#[allow(unused)]
pub fn pauli_x() -> Gate {
    let m = m_real![0, 1;
                    1, 0];

    Gate::new(1, m)
}

/// The Pauli-Y gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Pauli-Y_gate)
/// for more information.
#[allow(unused)]
pub fn pauli_y() -> Gate {
    let m = m![Complex::zero(),
               -Complex::i();
               Complex::i(),
               Complex::zero()];

    Gate::new(1, m)
}

/// The Pauli-Z gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Pauli-Z_gate)
/// for more information.
#[allow(unused)]
pub fn pauli_z() -> Gate {
    let m = m_real![1,  0;
                    0, -1];

    Gate::new(1, m)
}

/// A single qubit phase-shift gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Phase_shift_gates)
/// for more information.
#[allow(unused)]
pub fn phase_shift(phi: f64) -> Gate {
    let m = m![Complex::one(),
               Complex::zero();
               Complex::zero(),
               Complex::new_euler(1f64, phi)];

    Gate::new(1, m)
}

/// The two qubit swap gate.
///
/// This swaps the value of the first and second qubit.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Swap_gate)
/// for more information.
#[allow(unused)]
pub fn swap() -> Gate {
    let m = m_real![1, 0, 0, 0;
                    0, 0, 1, 0;
                    0, 1, 0, 0;
                    0, 0, 0, 1];

    Gate::new(2, m)
}

/// The two qubit sqrt(swap) gate.
///
/// This performs half-way of a two-qubit swap.
///
/// It is universal such that any quantum many qubit gate can be constructed from
/// only sqrt(swap) and single qubit gates.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Square_root_of_Swap_gate)
/// for more information.
#[allow(unused)]
pub fn sqrt_swap() -> Gate {
    let alpha_one = c!(0.5f64, 0.5f64);
    let alpha_two = c!(0.5f64, -0.5f64);

    let m = m![Complex::one(),  Complex::zero(), Complex::zero(), Complex::zero();
               Complex::zero(), alpha_one,       alpha_two,       Complex::zero();
               Complex::zero(), alpha_two,       alpha_one,       Complex::zero();
               Complex::zero(), Complex::zero(), Complex::zero(), Complex::one() ];

    Gate::new(2, m)
}

/// The two qubit controlled not gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Controlled_gates)
/// for more information.
#[allow(unused)]
pub fn controlled_not() -> Gate {
    let m = m_real![1, 0, 0, 0;
                    0, 1, 0, 0;
                    0, 0, 0, 1;
                    0, 0, 1, 0];

    Gate::new(2, m)
}

/// A two qubit controlled gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Controlled_gates)
/// for more information.
///
/// # Panics
///
/// We panic if this supplied matrix isn't of size 2x2.
#[allow(unused)]
pub fn controlled(u: &Matrix) -> Gate {
    assert_eq!(2, u.size());

    let mut m = m_real![1, 0, 0, 0;
                        0, 1, 0, 0;
                        0, 0, 0, 0;
                        0, 0, 0, 0];

    m.embed(&u, 2, 2);
    Gate::new(2, m)
}

/// The two qubit controlled-X gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Controlled_gates)
/// for more information.
#[allow(unused)]
pub fn controlled_x() -> Gate {
    controlled(pauli_x().matrix())
}

/// The two qubit controlled-Y gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Controlled_gates)
/// for more information.
#[allow(unused)]
pub fn controlled_y() -> Gate {
    controlled(pauli_y().matrix())
}

/// The two qubit controlled-Z gate.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/Quantum_gate#Controlled_gates)
/// for more information.
#[allow(unused)]
pub fn controlled_z() -> Gate {
    controlled(pauli_z().matrix())
}


/// Convenience macro for testing a quantum gate.
macro_rules! test_gate {
    ($computer:expr, $gate:expr, $from:expr, $to:expr) => {
        $computer.initialize($from);
        $computer.apply($gate);
        $computer.collapse();
        assert_eq!($to, $computer.value());
        $computer.reset();
    };
}

#[test]
fn identity_test() {
    use complex::Complex;

    let id_gate = identity(3);
    let mut ket = Ket::new(8);
    ket.elements[5] = c![99f64, 0f64];

    let expected = ket.clone();

    ket.apply(id_gate);

    assert_eq!(expected, ket);
}

#[test]
fn hadamard_test() {
    use computer::QuantumComputer;

    let mut c = QuantumComputer::new(1);

    let mut apply_hadamard = || {
        c.initialize(0);
        c.apply(hadamard());
        c.collapse();
        let v = c.value();
        c.reset();

        v
    };

    let mut ones = 0;

    for _ in 0..1000 {
        if 1 == apply_hadamard() {
            ones += 1;
        }
    }

    assert!( ones <= 600 && 400 <= ones)
}

#[test]
fn pauli_x_test() {
    use computer::QuantumComputer;

    let mut c = QuantumComputer::new(1);

    // |0> goes to |1>
    test_gate!(c, pauli_x(), 0, 1);

    // |1> goes to |0>
    test_gate!(c, pauli_x(), 1, 0);
}

#[test]
fn pauli_y_test() {
    use computer::QuantumComputer;

    let mut c = QuantumComputer::new(1);

    // |0> goes to i|1>
    test_gate!(c, pauli_y(), 0, 1);

    // |1> goes to -i|0>
    test_gate!(c, pauli_y(), 1, 0);
}

#[test]
fn pauli_z_test() {
    use computer::QuantumComputer;

    let mut c = QuantumComputer::new(1);

    // |0> goes to |0>
    test_gate!(c, pauli_z(), 0, 0);

    // |1> goes to -|1>
    test_gate!(c, pauli_z(), 1, 1);
}

#[test]
fn phase_shift_test() {
    use computer::QuantumComputer;

    let phi = 0.3f64;
    let mut c = QuantumComputer::new(1);

    // |0> goes to |0>
    test_gate!(c, phase_shift(phi), 0, 0);

    // |1> goes to exp(i * phi)|1>
    test_gate!(c, phase_shift(phi), 1, 1);
}

#[test]
fn swap_test() {
    use computer::QuantumComputer;

    let mut c = QuantumComputer::new(2);

    // |00> goes to |00>
    test_gate!(c, swap(), 0, 0);

    // |01> goes to |10>
    test_gate!(c, swap(), 2, 1);
}

#[test]
fn sqrt_swap_test() {
    use computer::QuantumComputer;

    let mut c = QuantumComputer::new(2);

    // |00> goes to |00>
    test_gate!(c, sqrt_swap(), 0, 0);

    // |11> goes to |11>
    test_gate!(c, sqrt_swap(), 3, 3);
}

#[test]
fn controlled_not_test() {
    use computer::QuantumComputer;

    let mut c = QuantumComputer::new(2);

    // |00> goes to |00>
    test_gate!(c, controlled_not(), 0, 0);

    // |01> goes to |01>
    test_gate!(c, controlled_not(), 1, 1);

    // |10> goes to |11>
    test_gate!(c, controlled_not(), 2, 3);

    // |11> goes to |10>
    test_gate!(c, controlled_not(), 3, 2);
}

#[test]
fn controlled_test() {
    let g = controlled(&m_real![0, 1; 1, 0]);

    assert_eq!(controlled_not(), g);
}
