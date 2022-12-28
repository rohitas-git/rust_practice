use macro_demo::*;

// macro converts struct S to struct H
#[trace_vars(a)]
fn do_something(){
  let a=9;
  a=6;
  a=0;

#[test]
fn test_macro(){
// due to macro we have struct H in scope
    let _demo=H{};
}