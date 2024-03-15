use p3_baby_bear::BabyBear;
use p3_field::extension::BinomialExtensionField;
use p3_field::AbstractField;
use sp1_recursion_compiler::asm::VmBuilder;
use sp1_recursion_compiler::prelude::*;
use sp1_recursion_core::runtime::Runtime;

#[test]
fn test_compiler_conditionals() {
    type F = BabyBear;
    type EF = BinomialExtensionField<BabyBear, 4>;
    let mut builder = VmBuilder::<F>::default();

    let a: Var<_> = builder.eval(F::zero());
    let b: Var<_> = builder.eval(F::one());
    let c: Var<_> = builder.eval(F::zero());

    builder
        .if_ne(a, b)
        .then(|builder| builder.assign(c, F::two()));

    builder.assert_var_eq(b, F::one());

    let code = builder.compile_to_asm();
    println!("{}", code);
    // let program = builder.compile();
    let program = code.machine_code();

    let mut runtime = Runtime::<F, EF>::new(&program);
    runtime.run();
}
