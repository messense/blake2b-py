#![feature(test)]

mod blake2b;

use pyo3::exceptions::ValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::wrap_pyfunction;

/// Convenience function for building python value errors.
fn value_error<V>(msg: String) -> PyResult<V> {
    Err(ValueError::py_err(msg))
}

type CompressArgs = (usize, Vec<u64>, Vec<u64>, Vec<u64>, bool);

/// extract_blake2b_parameters(input)
/// --
///
/// Extract parameters for the ``blake2b_compress`` function from a test
/// vector represented by a byte string.
///
/// Parameters
/// ----------
/// input : bytes, List[int]
///     A vector of 213 bytes representing the test vector.
///
/// Returns
/// ----------
/// out : (int, List[int], List[int], List[int], bool)
///     A tuple of parameters to pass to the ``blake2b_compress`` function.
#[pyfunction]
fn extract_blake2b_parameters(py: Python, input: Vec<u8>) -> PyResult<CompressArgs> {
    let result = blake2b::extract_blake2b_parameters(&input);

    match result {
        Err(msg) => Err(PyErr::new::<ValueError, _>(msg)),
        Ok(args) => {
            let (rounds, state, block, offsets, flag) = args;
            Ok((
                rounds,
                state.to_vec(),
                block.to_vec(),
                offsets.to_vec(),
                flag,
            ))
        }
    }
}

/// blake2b_compress(num_rounds, h_starting_state, block, t_offset_counters,
///     final_block_flag)
/// --
///
/// Calculates a blake2b hash for the given message block.
///
/// Parameters
/// ----------
/// num_rounds : int
///     The number of rounds of mixing to occur during hashing.
/// h_starting_state : List[int]
///     A vector of 8 64-bit integers representing the starting state of the
///     hash function.
/// block : List[int]
///     A vector of 16 64-bit integers representing the message block to be hashed.
/// t_offset_counters : List[int]
///     A vector of 2 64-bit integers representing the message byte offset at
///     the end of the current block.
/// final_block_flag : bool
///     A flag indicating the final block of the message.
///
/// Returns
/// -------
/// out : bytes
///     A vector of 64 bytes representing the blake2b hash of the input data.
#[pyfunction]
fn blake2b_compress(
    py: Python,
    num_rounds: usize,
    h_starting_state: Vec<u64>,
    block: Vec<u64>,
    t_offset_counters: Vec<u64>,
    final_block_flag: bool,
) -> PyResult<PyObject> {
    if h_starting_state.len() != 8 {
        return value_error(format!(
            "starting state vector must have length 8, got: {}",
            h_starting_state.len(),
        ));
    }
    if block.len() != 16 {
        return value_error(format!(
            "block vector must have length 16, got: {}",
            block.len(),
        ));
    }
    if t_offset_counters.len() != 2 {
        return value_error(format!(
            "offset counters vector must have length 2, got: {}",
            t_offset_counters.len(),
        ));
    }

    let result = blake2b::blake2b_compress(
        num_rounds,
        &h_starting_state,
        &block,
        &t_offset_counters,
        final_block_flag,
    );

    Ok(PyBytes::new(py, &result).into())
}

/// Functions for calculating blake2b hashes.
#[pymodule]
fn blake2b(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(extract_blake2b_parameters))?;
    m.add_wrapped(wrap_pyfunction!(blake2b_compress))?;

    Ok(())
}
