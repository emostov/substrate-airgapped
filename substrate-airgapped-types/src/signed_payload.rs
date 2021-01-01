use crate::{Encoded, frame::CallMethod, hashing::blake2_256};
use codec::Encode;
use sp_runtime::{traits::SignedExtension, transaction_validity::TransactionValidityError};

/// A payload that has been signed for an unchecked extrinsics.
///
/// Note that the payload that we sign to produce unchecked extrinsic signature
/// is going to be different than the `SignaturePayload` - so the thing the extrinsic
/// actually contains.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct SignedPayload<Call: CallMethod, Extra: SignedExtension>(
	(Encoded<Call>, Extra, Extra::AdditionalSigned),
);

impl<Call, Extra> SignedPayload<Call, Extra>
where
	Call: CallMethod,
	Extra: SignedExtension,
{
	/// Create new `SignedPayload`.
	///
	/// This function may fail if `additional_signed` of `Extra` is not available.
	pub fn new(call: Encoded<Call>, extra: Extra) -> Result<Self, TransactionValidityError> {
		let additional_signed = extra.additional_signed()?;
		let raw_payload = (call, extra, additional_signed);

		Ok(Self(raw_payload))
	}

	/// Create new `SignedPayload` from raw components.
	pub fn from_raw(call: Encoded<Call>, extra: Extra, additional_signed: Extra::AdditionalSigned) -> Self {
		Self((call, extra, additional_signed))
	}
}

impl<Call, Extra> Encode for SignedPayload<Call, Extra>
where
	Call: CallMethod,
	Extra: SignedExtension,
{
	/// Get an encoded version of this payload.
	///
	/// Payloads longer than 256 bytes are going to be `blake2_256`-hashed.
	fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
		self.0.using_encoded(|payload| {
			if payload.len() > 256 {
				f(&blake2_256(payload)[..])
			} else {
				f(payload)
			}
		})
	}
}