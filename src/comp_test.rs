use crate::Interpreter;

#[cfg(test)]

mod comp_tests {

  #[test]
  fn test_core() {
    let mut test_cinter = super::Interpreter::new();

    test_cinter.stack.push(1.0);
    test_cinter.stack.push(2.0);
    test_cinter.stack.push(3.0);
    test_cinter.stack.push(4.0);

    test_cinter.c_rot();
    test_cinter.c_rot();
    test_cinter.c_roll();
    test_cinter.c_roll();

    test_cinter.c_dtor();
    test_cinter.c_cos();
    test_cinter.c_acos();
    test_cinter.c_sin();
    test_cinter.c_asin();
    test_cinter.c_tan();
    test_cinter.c_atan();
    test_cinter.c_rtod();
    test_cinter.c_round();
    test_cinter.c_roll();
    test_cinter.c_roll();
    test_cinter.c_roll();
    test_cinter.c_roll();
    test_cinter.c_dup();
    test_cinter.c_drop();
    test_cinter.c_swap();
    test_cinter.c_swap();
    test_cinter.c_add();
    test_cinter.c_sub();
    test_cinter.c_div();

    test_cinter.stack.push(10.0);
    test_cinter.c_log2();
    test_cinter.stack.push(10.0);
    test_cinter.stack.push(2.0);
    test_cinter.c_logn();
    test_cinter.c_sub();
    test_cinter.c_round();
    test_cinter.c_add();

    assert!(test_cinter.stack.pop().unwrap() == -0.2);
  }

  #[test]
  fn test_support() {
    assert!(super::Interpreter::gcd(55, 10) == 5);
    assert!(super::Interpreter::factorial(10) == 3628800);
  }

  #[test]
  fn test_roots() {
    let mut test_cinter = super::Interpreter::new();

    test_cinter.stack.push(2.0);
    test_cinter.c_dup();
    test_cinter.c_sqrt();
    test_cinter.c_swap();
    test_cinter.stack.push(32.0);
    test_cinter.c_exp();
    test_cinter.stack.push(32.0 * 2.0);
    test_cinter.c_throot();

    assert!(test_cinter.stack.pop().unwrap() == test_cinter.stack.pop().unwrap());

    test_cinter.stack.push(1.0);
    test_cinter.stack.push(-2.0);
    test_cinter.c_chs();
    test_cinter.c_chs();
    test_cinter.c_pi();
    test_cinter.c_mult();
    test_cinter.c_pi();
    test_cinter.stack.push(2.0);
    test_cinter.c_exp();
    test_cinter.stack.push(1.0);
    test_cinter.c_add();
    test_cinter.c_proot();
    test_cinter.c_add_all();
    test_cinter.stack.push(2.0);
    test_cinter.c_div();
    test_cinter.c_pi();

    assert!(test_cinter.stack.pop().unwrap() == test_cinter.stack.pop().unwrap());
  }

  #[test]
  #[should_panic]
  fn test_cls() {
    let mut test_cinter = super::Interpreter::new();

    test_cinter.stack.push(1.0);
    test_cinter.stack.push(2.0);
    test_cinter.stack.push(3.0);
    test_cinter.stack.push(4.0);
    test_cinter.stack.push(1.0);
    test_cinter.stack.push(2.0);
    test_cinter.stack.push(3.0);
    test_cinter.stack.push(4.0);
    test_cinter.stack.push(1.0);
    test_cinter.stack.push(2.0);
    test_cinter.stack.push(3.0);
    test_cinter.stack.push(4.0);
    test_cinter.c_cls();

    assert!(test_cinter.stack.pop().unwrap() == 0.0);
  }

  #[test]
  fn test_mem() {
    let mut test_cinter = super::Interpreter::new();

    test_cinter.stack.push(1.0);
    test_cinter.stack.push(2.0);
    test_cinter.stack.push(3.0);
    test_cinter.stack.push(4.0);
    test_cinter.stack.push(1.0);
    test_cinter.stack.push(2.0);
    test_cinter.stack.push(3.0);
    test_cinter.stack.push(4.0);
    test_cinter.stack.push(1.0);
    test_cinter.stack.push(2.0);
    test_cinter.stack.push(3.0);
    test_cinter.stack.push(4.0);
    test_cinter.c_chs();
    test_cinter.c_abs();
    test_cinter.c_inv();
    test_cinter.c_inv();
    test_cinter.c_pi();
    test_cinter.c_euler();
    test_cinter.stack.push(0.0);
    test_cinter.c_store_b(); // 0
    test_cinter.c_store_a(); // e
    test_cinter.c_store_c(); // pi
    test_cinter.c_cls();
    test_cinter.c_push_b(); // 0
    test_cinter.c_push_c(); // pi
    test_cinter.c_add();
    test_cinter.c_push_a(); // e
    test_cinter.c_add();

    assert!(test_cinter.stack.pop().unwrap() == std::f64::consts::PI + std::f64::consts::E);
  }

  #[test]
  fn test_cmp() {
    let mut test_cinter = super::Interpreter::new();

    test_cinter.stack.push(10.0);
    test_cinter.c_log10();
    test_cinter.c_euler();
    test_cinter.c_ln();
    test_cinter.stack.push(105.0);
    test_cinter.stack.push(2.0);
    test_cinter.c_mod();
    test_cinter.stack.push(3049.0);
    test_cinter.stack.push(1009.0);
    test_cinter.c_gcd();
    test_cinter.c_mult_all();

    assert!(test_cinter.stack.pop().unwrap() == 1.0);

    test_cinter.stack.push(20.0);
    test_cinter.c_fact();

    assert!(test_cinter.stack.pop().unwrap() == 2432902008176640000.0);
  }
}
