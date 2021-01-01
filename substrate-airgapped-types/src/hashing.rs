/// Do a Blake2 256-bit hash and return result.
///
/// Equivalent of `sp_core::hashing::blake2_256`
pub fn blake2_256(data: &[u8]) -> [u8; 32] {
	let mut r = [0; 32];
	blake2_256_into(data, &mut r);

	r
}

/// Do a Blake2 256-bit hash and place result in `dest`.
///
/// Equivalent of `sp_core::hashing::blake2_256_into`
pub fn blake2_256_into(data: &[u8], dest: &mut [u8; 32]) {
	dest.copy_from_slice(blake2_rfc::blake2b::blake2b(32, &[], data).as_bytes());
}
