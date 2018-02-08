use ast::*;
use parser::*;

#[test]
fn ident() {
    parses_to! {
        parser: RossParser,
        input: "_ab12de",
        rule: Rule::ident,
        tokens: [
            ident(0, 7)
        ]
    };

    fails_with! {
        parser: RossParser,
        input: "1abc",
        rule: Rule::ident,
        positives: vec![Rule::ident],
        negatives: vec![],
        pos: 0
    };
}

/*
#[test]
fn fn_decl() {
    assert_eq!(
        parse_Fn("fn main(foo, bar) {}"),
        Ok(Fn {
            ident: Ident::new("main"),
            args: vec![
                FnArg { ident: Ident::new("foo") },
                FnArg { ident: Ident::new("bar") },
            ],
            body: vec![],
        })
    );
}

#[test]
fn some_code() {
    let res = parse_Module("
        fn main(foo, bar) {
            let hello = 14. + (bar * 4.);
            foo = hello - 1.;
            foo -= 123.;
            return foo;
        }
    ").unwrap();
    // println!("{:#?}", res);
}

#[test]
fn float_lits() {
    assert_eq!(parse_FloatLit("123.456"), Ok(123.456));
    assert_eq!(parse_FloatLit(".456"), Ok(0.456));
    assert_eq!(parse_FloatLit("0.456"), Ok(0.456));
    assert_eq!(parse_FloatLit("123."), Ok(123.));
}


#[test]
#[should_panic]
fn recursion_not_allowed() {
    let module = "
        fn main() {
            foo();
        }

        fn foo() {
            bar();
        }

        fn bar() {
            main();
        }
    ";
    let module = match parse_Module(module) {
        Err(e) => {
            println!("{:?}", e);
            return;
        },
        Ok(module) => module,
    };

    ::validate::functions_dont_recur(&module).unwrap();
}
*/