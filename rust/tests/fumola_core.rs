use fumola::check::{exp, parse};

#[test]
fn test_record_1() {
    exp(
        "ret [$s => 1]",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret [$s => 1]])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_record_2() {
    exp(
        "ret [$s => 1; $t => $two]",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret [$s => 1; $t => $two]])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_let_record() {
    exp(
        "let name = ret $three; let val = ret 3; ret [name => val]",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret [$three => 3]])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_record_pattern() {
    exp(
        "let [$secret => val] = ret [$secret => 42]; ret [$result => val]",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret [$result => 42]])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_assert_equal_success() {
    exp(
        "assert 1 == 1",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret []])]\n]\n"),
    )
    .unwrap()
}

#[test]
fn test_assert_equal_failure() {
    exp("assert 1 == 2", None, Some("fumola [\n  store = [];\n  procs = [% => error(assertionFailure(1 == 2), [trace = []; stack = []; bxes = []; vals = []; cont = 1 == 2])]\n]\n")).unwrap()
}

#[test]
fn test_assert_not_equal_success() {
    exp(
        "assert 1 != 2",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret []])]\n]\n"),
    )
    .unwrap()
}

#[test]
fn test_assert_not_equal_failure() {
    exp("assert 1 != 1", None, Some("fumola [\n  store = [];\n  procs = [% => error(assertionFailure(1 != 1), [trace = []; stack = []; bxes = []; vals = []; cont = 1 != 1])]\n]\n")).unwrap()
}

#[test]
fn test_ret() {
    exp(
        "ret 1",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret 1])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_let_ret() {
    exp(
        "let x = ret 1; ret x",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret 1])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_let_let_ret() {
    exp(
        "let x = let y = ret 1; ret y; ret x",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret 1])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_let_nest_ret() {
    exp(
        "let x = #$n { ret 1 }; ret x",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([#n {ret 1}; ret 1])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_put() {
    exp(
        "$a := 1",
        None,
        Some("fumola [\n  store = [a => 1];\n  procs = [% => halted([put a <= 1])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_nest_put() {
    exp(
        "#$n { $a := 1 }",
        None,
        Some("fumola [\n  store = [n/a => 1];\n  procs = [% => halted([#n {put n/a <= 1}])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_put_get() {
    parse(
        "@`($a := 1)",
        "Get(CallByValue(Put(Sym(Id(\"a\")), Num(1))))",
    )
    .unwrap();
}

#[test]
fn test_nest_put_get() {
    exp(
        "let x = #$n{ $a := 3 }; @x",
        None,
        Some("fumola [\n  store = [n/a => 3];\n  procs = [% => halted([#n {put n/a <= 3}; get n/a => 3])]\n]\n")).unwrap();
}

#[test]
fn test_get_undef() {
    exp(
        "@$s",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => error(notAPointer($s), [trace = []; stack = []; bxes = []; vals = []; cont = @$s])]\n]\n")).unwrap();
}

#[test]
fn test_let_put_get() {
    parse(
        "let x = $a := 1; @x",
        "Let(Var(\"x\"), Put(Sym(Id(\"a\")), Num(1)), Get(Var(\"x\")))",
    )
    .unwrap();
}

#[test]
fn test_nest() {
    exp(
        "# $311 { ret 311 }",
        Some("Nest(Sym(Num(311)), Ret(Num(311)))"),
        Some("fumola [\n  store = [];\n  procs = [% => halted([#311 {ret 311}])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_switch() {
    parse("switch #$apple(1) { #$apple(x){ret x}; #$banana(x){ret x} }",
              "Switch(Variant(Sym(Id(\"apple\")), Num(1)), Gather(Case(Case { label: Sym(Id(\"apple\")), pattern: Var(\"x\"), body: Ret(Var(\"x\")) }), Case(Case { label: Sym(Id(\"banana\")), pattern: Var(\"x\"), body: Ret(Var(\"x\")) })))").unwrap();
}

#[test]
fn test_branches_1() {
    parse(
        "{ $apple => ret 1 }",
        "Branches(Branch(Branch { label: Sym(Id(\"apple\")), body: Ret(Num(1)) }))",
    )
    .unwrap();
}

#[test]
fn test_branches_2() {
    parse("{ $apple => ret 1; $banana => \\x => ret x }", 
              "Branches(Gather(Branch(Branch { label: Sym(Id(\"apple\")), body: Ret(Num(1)) }), Branch(Branch { label: Sym(Id(\"banana\")), body: Lambda(Var(\"x\"), Ret(Var(\"x\"))) })))").unwrap();
}

#[test]
fn test_project_branches() {
    exp(
	      "{ $apple => ret 1; $banana => \\x => x := x } <= $apple",
	      Some("Project(Branches(Gather(Branch(Branch { label: Sym(Id(\"apple\")), body: Ret(Num(1)) }), Branch(Branch { label: Sym(Id(\"banana\")), body: Lambda(Var(\"x\"), Put(Var(\"x\"), Var(\"x\"))) }))), Sym(Id(\"apple\")))"),
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret 1])]\n]\n")
    ).unwrap();
}

#[test]
fn test_let_switch() {
    exp(
        "let a = ret $apple; switch #a(1) { #a(x){ret x}; #$banana(x){ret x} }",
        Some("Let(Var(\"a\"), Ret(Sym(Id(\"apple\"))), Switch(Variant(Var(\"a\"), Num(1)), Gather(Case(Case { label: Var(\"a\"), pattern: Var(\"x\"), body: Ret(Var(\"x\")) }), Case(Case { label: Sym(Id(\"banana\")), pattern: Var(\"x\"), body: Ret(Var(\"x\")) }))))"),
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret 1])]\n]\n")
    ).unwrap();
}

#[test]
fn test_syms() {
    parse(
        "let _ = ret $1; let _ = ret $a; ret 0",
        "Let(Ignore, Ret(Sym(Num(1))), Let(Ignore, Ret(Sym(Id(\"a\"))), Ret(Num(0))))",
    )
    .unwrap();

    parse("let _ = ret $a-1; let _ = ret $a.1; ret 0",
              "Let(Ignore, Ret(Sym(Tri(Id(\"a\"), Dash, Num(1)))), Let(Ignore, Ret(Sym(Tri(Id(\"a\"), Dot, Num(1)))), Ret(Num(0))))").unwrap();

    parse("let _ = ret $a_1-b_2.c; ret 0",
              "Let(Ignore, Ret(Sym(Tri(Id(\"a_1\"), Dash, Tri(Id(\"b_2\"), Dot, Id(\"c\"))))), Ret(Num(0)))").unwrap();
}

#[test]
fn test_let_box_syntax() {
    let ast = "LetBx(Var(\"f\"), Ret(Bx(BxVal { bxes: BxesEnv({}), name: None, code: Lambda(Var(\"x\"), Lambda(Var(\"y\"), Put(Var(\"x\"), Var(\"y\")))) })), App(App(Extract(Var(\"f\")), Sym(Id(\"a\"))), Num(1)))";

    // 0. most verbose, with least special syntax.
    parse("let box f = ret {\\x => \\y => x := y}; f $a 1", ast).unwrap();

    // 1. the "ret" keyword is optional when we give a literal box value.
    parse("let box f = {\\x => \\y => x := y}; f $a 1", ast).unwrap();

    // 2. the "let" keyword (and '=') is also optional when we give a literal box value.
    parse("box f {\\x => \\y => x := y}; f $a 1", ast).unwrap();
}

#[test]
fn test_rec_box_syntax() {
    exp(
        "box rec z { ret z }; z",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret rec z {[] |- ret z}])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_let_box() {
    // box 'put_' contains code that, when given a symbol and a value,
    // puts the value at that symbol.
    let result =
        "fumola [\n  store = [n/a => 1];\n  procs = [% => halted([#n {put n/a <= 1}])]\n]\n";

    exp(
        "let box put_ = ret {\\x => \\y => x := y}; #$n { put_ $a 1 }",
        None,
        Some(result),
    )
    .unwrap();

    // shorter syntax.
    exp(
        "box put_ {\\x => \\y => x := y}; #$n { put_ $a 1 }",
        None,
        Some(result),
    )
    .unwrap();
}

#[test]
fn test_put_link() {
    exp("let _ = $s := 42; &$s",
               None,
               Some("fumola [\n  store = [s => 42];\n  procs = [% => halted([put s <= 42; link $s => !s])]\n]\n")).unwrap()
}

#[test]
fn test_put_link_get() {
    exp("let _ = $s := 42; @`(&$s)",
               None,
               Some("fumola [\n  store = [s => 42];\n  procs = [% => halted([put s <= 42; link $s => !s; get s => 42])]\n]\n")).unwrap()
}

#[test]
fn test_link_waiting_for_ptr() {
    exp("&$s",
               None,
               Some("fumola [\n  store = [];\n  procs = [% => waitingForPtr([trace = []; stack = []; bxes = []; vals = []; cont = &$s], s)]\n]\n")).unwrap()
}

#[test]
fn test_link_invalid_proc() {
    exp("&~s",
               None,
               Some("fumola [\n  store = [];\n  procs = [% => error(invalidProc(s), [trace = []; stack = []; bxes = []; vals = []; cont = &~s])]\n]\n")).unwrap()
}

#[test]
fn test_link_wait_for_halt() {
    exp("let p = ~$p { ret 42 }; &p",
               None,
               Some("fumola [\n  store = [p => ~p];\n  procs = [% => halted([link ~p => 42]); p => halted([ret 42])]\n]\n")).unwrap()
}

#[test]
fn test_spawn() {
    exp("~$x { ret 1 }", None, None).unwrap()
}

#[test]
fn test_nest_spawn() {
    exp("#$n{ ~$x { ret 1 } }", None, None).unwrap()
}

#[test]
fn test_let_spawn() {
    exp("let r = ret 1 ; ~$x { ret r }", None, None).unwrap()
}

#[test]
fn test_cbpv_convert() {
    exp(
        "box id3 {\\x => \\y => \\z => ret x}; box one {ret 1}; box two {ret 2}; box three {ret 3}; id3 `(one) `(two) `(three)",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret 1])]\n]\n")).unwrap();
}
