#[cfg(test)]

mod comp_tests {
    use crate::cmdin::Interpreter;

    #[test]
    fn test_core() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(1.0.to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.stack.push(3.0.to_string());
        test_interc.stack.push(4.0.to_string());

        test_interc.c_add_one("");
        test_interc.c_add_one("");
        test_interc.c_add_one("");
        test_interc.c_sub_one("");
        test_interc.c_sub_one("");
        test_interc.c_sub_one("");

        test_interc.c_rot("");
        test_interc.c_rot("");
        test_interc.c_roll("");
        test_interc.c_roll("");

        test_interc.c_degrad("");
        test_interc.c_cos("");
        test_interc.c_acos("");
        test_interc.c_sin("");
        test_interc.c_asin("");
        test_interc.c_tan("");
        test_interc.c_atan("");
        test_interc.c_raddeg("");
        test_interc.c_round("");
        test_interc.c_roll("");
        test_interc.c_roll("");
        test_interc.c_roll("");
        test_interc.c_roll("");
        test_interc.c_dup("");
        test_interc.c_drop("");
        test_interc.c_swap("");
        test_interc.c_swap("");
        test_interc.c_add("");
        test_interc.c_sub("");
        test_interc.c_div("");

        test_interc.stack.push(10.0.to_string());
        test_interc.c_log2("");
        test_interc.stack.push(10.0.to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.c_logn("");
        test_interc.c_sub("");
        test_interc.c_round("");
        test_interc.c_add("");

        assert!(test_interc.pop_stack_float() == -0.2);
    }

    #[test]
    fn test_support() {
        assert!(Interpreter::gcd(55, 10) == 5);
        assert!(Interpreter::factorial(10.0) == 3628800.0);
    }

    #[test]
    fn test_roots() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(2.0.to_string());
        test_interc.c_dup("");
        test_interc.c_sqrt("");
        test_interc.c_swap("");
        test_interc.stack.push(32.0.to_string());
        test_interc.c_exp("");
        test_interc.stack.push((32.0 * 2.0).to_string());
        test_interc.c_throot("");

        assert!(test_interc.pop_stack_float() == test_interc.pop_stack_float());

        test_interc.stack.push(1.0.to_string());
        test_interc.stack.push((-2.0).to_string());
        test_interc.c_chs("");
        test_interc.c_chs("");
        test_interc.c_pi("");
        test_interc.c_mult("");
        test_interc.c_pi("");
        test_interc.stack.push(2.0.to_string());
        test_interc.c_exp("");
        test_interc.stack.push(1.0.to_string());
        test_interc.c_add("");
        test_interc.c_proot("");
        test_interc.c_add_all("");
        test_interc.stack.push(2.0.to_string());
        test_interc.c_div("");
        test_interc.c_pi("");

        assert!(test_interc.pop_stack_float() == test_interc.pop_stack_float());
    }

    #[test]
    #[should_panic]
    fn test_cls() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(1.0.to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.stack.push(3.0.to_string());
        test_interc.stack.push(4.0.to_string());
        test_interc.stack.push(1.0.to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.stack.push(3.0.to_string());
        test_interc.stack.push(4.0.to_string());
        test_interc.stack.push(1.0.to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.stack.push(3.0.to_string());
        test_interc.stack.push(4.0.to_string());
        test_interc.c_cls("");

        assert!(test_interc.pop_stack_float() == 0.0);
    }

    #[test]
    fn test_mem() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(1.0.to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.stack.push(3.0.to_string());
        test_interc.stack.push(4.0.to_string());
        test_interc.stack.push(1.0.to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.stack.push(3.0.to_string());
        test_interc.stack.push(4.0.to_string());
        test_interc.stack.push(1.0.to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.stack.push(3.0.to_string());
        test_interc.stack.push(4.0.to_string());
        test_interc.c_chs("");
        test_interc.c_abs("");
        test_interc.c_inv("");
        test_interc.c_inv("");
        test_interc.c_pi("");
        test_interc.c_euler("");
        test_interc.stack.push(0.0.to_string());
        test_interc.c_store_b(""); // 0
        test_interc.c_store_a(""); // e
        test_interc.c_store_c(""); // pi
        test_interc.c_cls("");
        test_interc.c_push_b(""); // 0
        test_interc.c_push_c(""); // pi
        test_interc.c_add("");
        test_interc.c_push_a(""); // e
        test_interc.c_add("");

        assert!(test_interc.pop_stack_float() == std::f64::consts::PI + std::f64::consts::E);
    }

    #[test]
    fn test_cmp() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(10.0.to_string());
        test_interc.c_log10("");
        test_interc.c_euler("");
        test_interc.c_ln("");
        test_interc.stack.push(105.0.to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.c_mod("");
        test_interc.stack.push(3049.0.to_string());
        test_interc.stack.push(1009.0.to_string());
        test_interc.c_gcd("");
        test_interc.c_mult_all("");

        assert!(test_interc.pop_stack_float() == 1.0);

        test_interc.stack.push(20.0.to_string());
        test_interc.c_fact("");

        assert!(test_interc.pop_stack_float() == 2432902008176640000.0);
    }

    #[test]
    fn test_rand() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(2.0.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.0.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.0.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.0.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.0.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.0.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.0.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.0.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.0.to_string());
        test_interc.c_rand("");
        test_interc.stack.push(2.0.to_string());
        test_interc.c_rand("");
        test_interc.c_max("");

        assert!(test_interc.pop_stack_float() <= 1.0);
    }

    #[test]
    fn test_minmax() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(1.0.to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.c_min("");

        assert!(test_interc.pop_stack_float() == 1.0);

        test_interc.stack.push(1.0.to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.c_max("");

        assert!(test_interc.pop_stack_float() == 2.0);

        test_interc.stack.push(1.0.to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.stack.push(3.0.to_string());
        test_interc.stack.push(4.0.to_string());
        test_interc.c_min_all("");

        assert!(test_interc.pop_stack_float() == 1.0);

        test_interc.stack.push(1.0.to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.stack.push(3.0.to_string());
        test_interc.stack.push(4.0.to_string());
        test_interc.c_max_all("");

        assert!(test_interc.pop_stack_float() == 4.0);
    }

    #[test]
    fn test_conv() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push(100.0.to_string());
        test_interc.c_celfah("");
        test_interc.c_fahcel("");
        test_interc.c_dechex("");
        test_interc.c_hexbin("");
        test_interc.c_binhex("");
        test_interc.c_hexdec("");
        test_interc.c_decbin("");
        test_interc.c_bindec("");
        test_interc.c_ftm("");
        test_interc.c_mft("");

        assert!(test_interc.pop_stack_float() == 100.0);
    }

    #[test]
    fn test_avg() {
        let mut test_interc = Interpreter::new();

        test_interc.stack.push((-2.0).to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.c_avg("");

        assert!(test_interc.pop_stack_float() == 0.0);

        test_interc.stack.push(1.0.to_string());
        test_interc.stack.push(2.0.to_string());
        test_interc.stack.push(3.0.to_string());
        test_interc.stack.push(4.0.to_string());
        test_interc.c_avg_all("");

        assert!(test_interc.pop_stack_float() == 2.5);
    }

}
