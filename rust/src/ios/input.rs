use super::result::CResult;
use super::string::CharPtr;
use crate::panic::{handle_exception_result, Zip};
use crate::ptr::RPtr;
use js_chain_libs::{Account, Input, Inputs, Value};

#[no_mangle]
pub unsafe extern "C" fn input_from_account(
  account: RPtr, v: &mut RPtr, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| {
    account
      .typed_ref::<Account>()
      .zip(v.owned::<Value>())
      .map(|(account, v)| Input::from_account(account, v))
  })
  .map(|input| RPtr::new(input))
  .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn inputs_size(
  inputs: RPtr, result: &mut usize, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| inputs.typed_ref::<Inputs>().map(|inputs| inputs.size()))
    .response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn inputs_get(
  inputs: RPtr, index: usize, result: &mut RPtr, error: &mut CharPtr
) -> bool {
  handle_exception_result(|| inputs.typed_ref::<Inputs>().map(|inputs| inputs.get(index)))
    .map(|input| RPtr::new(input))
    .response(result, error)
}
