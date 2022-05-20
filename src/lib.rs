/*
 * @Author: shijianzhong
 * @Date: 2022-05-20 09:55:18
 * @LastEditors: shijianzhong
 * @LastEditTime: 2022-05-20 15:38:09
 * @Description: file content
 */
mod utils;
extern crate wasm_bindgen;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(){
  alert("hello,world");
  set_panic_hook();
}


#[wasm_bindgen]
pub fn f1_on(n:u32){

  for _i in 1..n{

  }
}
#[wasm_bindgen]
pub fn f1_on2(n:u32){
  for _i in 1..n{
    for _j in 1..n{
    }
  }
}