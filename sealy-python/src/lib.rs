use pyo3::prelude::*;

mod ciphertext;
mod context;
mod context_data;
mod decryptor;
mod encoder;
mod encryptor;
mod evaluator;
mod keys;
mod memory;
mod parameters;
mod plaintext;
mod poly_array;

use crate::ciphertext::PyCiphertext;
use crate::context::PyContext;
use crate::context_data::PyContextData;
use crate::decryptor::PyDecryptor;
use crate::encoder::{PyBFVDecimalEncoder, PyBFVEncoder, PyCKKSEncoder};
use crate::encryptor::{PyAsymmetricComponents, PyEncryptor};
use crate::evaluator::{PyBFVEvaluator, PyCKKSEvaluator};
use crate::keys::{PyGaloisKey, PyKeyGenerator, PyPublicKey, PyRelinearizationKey, PySecretKey};
use crate::memory::PyMemoryPool;
use crate::parameters::{
	PyCoefficientModulus, PyDegreeType, PyEncryptionParameters, PyModulus, PyPlainModulus,
	PySchemeType, PySecurityLevel,
};
use crate::plaintext::PyPlaintext;
use crate::poly_array::PyPolynomialArray;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn sealy(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_class::<PySchemeType>()?;
	m.add_class::<PyDegreeType>()?;
	m.add_class::<PySecurityLevel>()?;
	m.add_class::<PyModulus>()?;
	m.add_class::<PyPlainModulus>()?;
	m.add_class::<PyCoefficientModulus>()?;
	m.add_class::<PyEncryptionParameters>()?;
	m.add_class::<PyContextData>()?;
	m.add_class::<PyContext>()?;
	m.add_class::<PyPublicKey>()?;
	m.add_class::<PySecretKey>()?;
	m.add_class::<PyKeyGenerator>()?;
	m.add_class::<PyGaloisKey>()?;
	m.add_class::<PyRelinearizationKey>()?;
	m.add_class::<PyMemoryPool>()?;
	m.add_class::<PyPlaintext>()?;
	m.add_class::<PyCiphertext>()?;
	m.add_class::<PyPolynomialArray>()?;
	m.add_class::<PyBFVEncoder>()?;
	m.add_class::<PyBFVDecimalEncoder>()?;
	m.add_class::<PyCKKSEncoder>()?;
	m.add_class::<PyAsymmetricComponents>()?;
	m.add_class::<PyEncryptor>()?;
	m.add_class::<PyDecryptor>()?;
	m.add_class::<PyBFVEvaluator>()?;
	m.add_class::<PyCKKSEvaluator>()?;

	Ok(())
}
