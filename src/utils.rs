/*
 * @Author: shijianzhong
 * @Date: 2022-05-20 10:15:12
 * @LastEditors: shijianzhong
 * @LastEditTime: 2022-05-20 10:16:13
 * @Description: file content
 */
pub fn set_panic_hook() {
  // When the `console_error_panic_hook` feature is enabled, we can call the
  // `set_panic_hook` function at least once during initialization, and then
  // we will get better error messages if our code ever panics.
  //
  // For more details see
  // https://github.com/rustwasm/console_error_panic_hook#readme
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}
