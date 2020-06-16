use blake2b_simd::Params;

use crate::jubjub::{JubjubEngine, ToUniform};

pub fn hash_to_scalar<E: JubjubEngine>(persona: &[u8], a: &[u8], b: &[u8]) -> E::Fs {
    let mut hasher = Params::new().hash_length(64).personal(persona).to_state();
    hasher.update(a);
    hasher.update(b);
    let ret = hasher.finalize();
    E::Fs::to_uniform(ret.as_ref())
}

pub fn forall<A, F>(opt: Option<A>, f: F) -> bool
    where F: FnOnce(A) -> bool {
    match opt {
        Some(a) => f(a),
        None => true,
    }
}

pub fn exists<A, F>(opt: Option<A>, f: F) -> bool
    where F: FnOnce(A) -> bool {
    match opt {
        Some(a) => f(a),
        None => false,
    }
}
