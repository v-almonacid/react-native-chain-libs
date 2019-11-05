use super::result::CResult;
use super::string::{CharPtr, IntoStr};
use crate::panic::{handle_exception_result, ToResult};
use crate::ptr::RPtr;
use js_chain_libs::PrivateKey;

#[no_mangle]
pub extern "C" fn private_key_from_bech32(
  bech32_str: CharPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| PrivateKey::from_bech32(bech32_str.into_str()).into_result())
    .map(|private_key| RPtr::new(private_key))
    .response(result, error)
}

#[no_mangle]
pub extern "C" fn private_key_to_public(
  private_key: RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| private_key.typed_ref::<PrivateKey>().map(|pvkey| pvkey.to_public().into()))
    .response(result, error)
}
