use crate::Interpreter;

#[cfg(test)]

mod comp_tests {

  #[test]
  fn test_core() {
    let mut test_cinter = super::Interpreter::new();

    test_cinter.stack.push(1.0.to_string());
    test_cinter.stack.push(2.0.to_string());
    test_cinter.stack.push(3.0.to_string());
    test_cinter.stack.push(4.0.to_string());

    test_cinter.c_add_one("o");
    test_cinter.c_sub_one("o");

    test_cinter.c_dechex("o");
    test_cinter.c_hexbin("o");
    test_cinter.c_binhex("o");
    test_cinter.c_hexdec("o");
    test_cinter.c_decbin("o");
    test_cinter.c_bindec("o");

    test_cinter.c_rot("o");
    test_cinter.c_rot("o");
    test_cinter.c_roll("o");
    test_cinter.c_roll("o");

    test_cinter.c_degrad("o");
    test_cinter.c_cos("o");
    test_cinter.c_acos("o");
    test_cinter.c_sin("o");
    test_cinter.c_asin("o");
    test_cinter.c_tan("o");
    test_cinter.c_atan("o");
    test_cinter.c_raddeg("o");
    test_cinter.c_round("o");
    test_cinter.c_roll("o");
    test_cinter.c_roll("o");
    test_cinter.c_roll("o");
    test_cinter.c_roll("o");
    test_cinter.c_dup("o");
    test_cinter.c_drop("o");
    test_cinter.c_swap("o");
    test_cinter.c_swap("o");
    test_cinter.c_add("o");
    test_cinter.c_sub("o");
    test_cinter.c_div("o");

    test_cinter.stack.push(10.0.to_string());
    test_cinter.c_log2("o");
    test_cinter.stack.push(10.0.to_string());
    test_cinter.stack.push(2.0.to_string());
    test_cinter.c_logn("o");
    test_cinter.c_sub("o");
    test_cinter.c_round("o");
    test_cinter.c_add("o");

    assert!(test_cinter.pop_stack_float() == -0.2);
  }

  #[test]
  fn test_support() {
    assert!(super::Interpreter::gcd(55, 10) == 5);
    assert!(super::Interpreter::factorial(10.0) == 3628800.0);
  }

  #[test]
  fn test_roots() {
    let mut test_cinter = super::Interpreter::new();

    test_cinter.stack.push(2.0.to_string());
    test_cinter.c_dup("o");
    test_cinter.c_sqrt("o");
    test_cinter.c_swap("o");
    test_cinter.stack.push(32.0.to_string());
    test_cinter.c_exp("o");
    test_cinter.stack.push((32.0 * 2.0).to_string());
    test_cinter.c_throot("o");

    assert!(test_cinter.pop_stack_float() == test_cinter.pop_stack_float());

    test_cinter.stack.push(1.0.to_string());
    test_cinter.stack.push((-2.0).to_string());
    test_cinter.c_chs("o");
    test_cinter.c_chs("o");
    test_cinter.c_pi("o");
    test_cinter.c_mult("o");
    test_cinter.c_pi("o");
    test_cinter.stack.push(2.0.to_string());
    test_cinter.c_exp("o");
    test_cinter.stack.push(1.0.to_string());
    test_cinter.c_add("o");
    test_cinter.c_proot("o");
    test_cinter.c_add_all("o");
    test_cinter.stack.push(2.0.to_string());
    test_cinter.c_div("o");
    test_cinter.c_pi("o");

    assert!(test_cinter.pop_stack_float() == test_cinter.pop_stack_float());
  }

  #[test]
  #[should_panic]
  fn test_cls() {
    let mut test_cinter = super::Interpreter::new();

    test_cinter.stack.push(1.0.to_string());
    test_cinter.stack.push(2.0.to_string());
    test_cinter.stack.push(3.0.to_string());
    test_cinter.stack.push(4.0.to_string());
    test_cinter.stack.push(1.0.to_string());
    test_cinter.stack.push(2.0.to_string());
    test_cinter.stack.push(3.0.to_string());
    test_cinter.stack.push(4.0.to_string());
    test_cinter.stack.push(1.0.to_string());
    test_cinter.stack.push(2.0.to_string());
    test_cinter.stack.push(3.0.to_string());
    test_cinter.stack.push(4.0.to_string());
    test_cinter.c_cls("o");

    assert!(test_cinter.pop_stack_float() == 0.0);
  }

  #[test]
  fn test_mem() {
    let mut test_cinter = super::Interpreter::new();

    test_cinter.stack.push(1.0.to_string());
    test_cinter.stack.push(2.0.to_string());
    test_cinter.stack.push(3.0.to_string());
    test_cinter.stack.push(4.0.to_string());
    test_cinter.stack.push(1.0.to_string());
    test_cinter.stack.push(2.0.to_string());
    test_cinter.stack.push(3.0.to_string());
    test_cinter.stack.push(4.0.to_string());
    test_cinter.stack.push(1.0.to_string());
    test_cinter.stack.push(2.0.to_string());
    test_cinter.stack.push(3.0.to_string());
    test_cinter.stack.push(4.0.to_string());
    test_cinter.c_chs("o");
    test_cinter.c_abs("o");
    test_cinter.c_inv("o");
    test_cinter.c_inv("o");
    test_cinter.c_pi("o");
    test_cinter.c_euler("o");
    test_cinter.stack.push(0.0.to_string());
    test_cinter.c_store_b("o"); // 0
    test_cinter.c_store_a("o"); // e
    test_cinter.c_store_c("o"); // pi
    test_cinter.c_cls("o");
    test_cinter.c_push_b("o"); // 0
    test_cinter.c_push_c("o"); // pi
    test_cinter.c_add("o");
    test_cinter.c_push_a("o"); // e
    test_cinter.c_add("o");

    assert!(test_cinter.pop_stack_float() == std::f64::consts::PI + std::f64::consts::E);
  }

  #[test]
  fn test_cmp() {
    let mut test_cinter = super::Interpreter::new();

    test_cinter.stack.push(10.0.to_string());
    test_cinter.c_log10("o");
    test_cinter.c_euler("o");
    test_cinter.c_ln("o");
    test_cinter.stack.push(105.0.to_string());
    test_cinter.stack.push(2.0.to_string());
    test_cinter.c_mod("o");
    test_cinter.stack.push(3049.0.to_string());
    test_cinter.stack.push(1009.0.to_string());
    test_cinter.c_gcd("o");
    test_cinter.c_mult_all("o");

    assert!(test_cinter.pop_stack_float() == 1.0);

    test_cinter.stack.push(20.0.to_string());
    test_cinter.c_fact("o");

    assert!(test_cinter.pop_stack_float() == 2432902008176640000.0);
  }
}
