use structopt::StructOpt;

use log::info;
use std::collections::HashMap;
use std::io;
use structopt::{clap, clap::Shell};

use fumola::{
    ast::{
        step::{Proc, Procs, Store, System},
        Exp, Sym,
    }
};

pub type OurResult<X> = Result<X, OurError>;

#[derive(Debug, Clone)]
pub enum OurError {
    String(String),
}

struct FreeVars {
    pub base: String,
    pub index: u32,
}

impl Iterator for FreeVars {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let r = Some(format!("{}{}", self.base, self.index));
        self.index += 1;
        r
    }
}

pub fn system_from_exp(e: &Exp) -> Result<System, ()> {
    let mut fv = FreeVars {
        base: "_t_".to_string(),
        index: 0,
    };
    let mut procs = HashMap::new();
    let e = fumola::cbpv::convert(&mut fv, e)?;
    procs.insert(Sym::None, Proc::Spawn(e));
    Ok(System {
        store: Store(HashMap::new()),
        procs: Procs(procs),
    })
}

fn check_exp_(input: &str, ast: Option<&str>, final_sys: Option<&str>) -> Result<(), ()> {
    let expr = fumola::parser::ExpParser::new().parse(input).unwrap();
    match ast {
        None => (),
        Some(a) => {
            assert_eq!(&format!("{:?}", expr), a);
        }
    };
    let mut sys = system_from_exp(&expr)?;
    fumola::step::fully(&mut sys);
    println!("final system:\n{}", &sys);
    match final_sys {
        None => (),
        Some(s) => {
            assert_eq!(&format!("{}", &sys), s);
        }
    };
    Ok(())
}

#[cfg(test)]
fn check_exp(input: &str, ast: &str) -> Result<(), ()> {
    check_exp_(input, Some(ast), None)
}

#[test]
fn test_record_1() {
    check_exp_(
        "ret [$s => 1]",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret [$s => 1]])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_record_2() {
    check_exp_(
        "ret [$s => 1; $t => $two]",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret [$s => 1; $t => $two]])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_let_record() {
    check_exp_(
        "let name = ret $three; let val = ret 3; ret [name => val]",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret [$three => 3]])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_record_pattern() {
    check_exp_(
        "let [$secret => val] = ret [$secret => 42]; ret [$result => val]",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret [$result => 42]])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_assert_equal_success() {
    check_exp_(
        "assert 1 == 1",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret []])]\n]\n"),
    )
    .unwrap()
}

#[test]
fn test_assert_equal_failure() {
    check_exp_("assert 1 == 2", None, Some("fumola [\n  store = [];\n  procs = [% => error(assertionFailure(1 == 2), [trace = []; stack = []; bxes = []; vals = []; cont = 1 == 2])]\n]\n")).unwrap()
}

#[test]
fn test_assert_not_equal_success() {
    check_exp_(
        "assert 1 != 2",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret []])]\n]\n"),
    )
    .unwrap()
}

#[test]
fn test_assert_not_equal_failure() {
    check_exp_("assert 1 != 1", None, Some("fumola [\n  store = [];\n  procs = [% => error(assertionFailure(1 != 1), [trace = []; stack = []; bxes = []; vals = []; cont = 1 != 1])]\n]\n")).unwrap()
}

#[test]
fn test_ret() {
    check_exp_(
        "ret 1",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret 1])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_let_ret() {
    check_exp_(
        "let x = ret 1; ret x",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret 1])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_let_let_ret() {
    check_exp_(
        "let x = let y = ret 1; ret y; ret x",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret 1])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_let_nest_ret() {
    check_exp_(
        "let x = #$n { ret 1 }; ret x",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([#n {ret 1}; ret 1])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_put() {
    check_exp_(
        "$a := 1",
        None,
        Some("fumola [\n  store = [a => 1];\n  procs = [% => halted([put a <= 1])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_nest_put() {
    check_exp_(
        "#$n { $a := 1 }",
        None,
        Some("fumola [\n  store = [n/a => 1];\n  procs = [% => halted([#n {put n/a <= 1}])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_put_get() {
    check_exp(
        "@`($a := 1)",
        "Get(CallByValue(Put(Sym(Id(\"a\")), Num(1))))",
    )
    .unwrap();
}

#[test]
fn test_nest_put_get() {
    check_exp_(
        "let x = #$n{ $a := 3 }; @x",
        None,
        Some("fumola [\n  store = [n/a => 3];\n  procs = [% => halted([#n {put n/a <= 3}; get n/a => 3])]\n]\n")).unwrap();
}

#[test]
fn test_get_undef() {
    check_exp_(
        "@$s",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => error(notAPointer($s), [trace = []; stack = []; bxes = []; vals = []; cont = @$s])]\n]\n")).unwrap();
}

#[test]
fn test_let_put_get() {
    check_exp(
        "let x = $a := 1; @x",
        "Let(Var(\"x\"), Put(Sym(Id(\"a\")), Num(1)), Get(Var(\"x\")))",
    )
    .unwrap();
}

#[test]
fn test_nest() {
    check_exp_(
        "# $311 { ret 311 }",
        Some("Nest(Sym(Num(311)), Ret(Num(311)))"),
        Some("fumola [\n  store = [];\n  procs = [% => halted([#311 {ret 311}])]\n]\n"),
    )
    .unwrap();
}

#[test]
fn test_switch() {
    check_exp("switch #$apple(1) { #$apple(x){ret x}; #$banana(x){ret x} }",
              "Switch(Variant(Sym(Id(\"apple\")), Num(1)), Gather(Case(Case { label: Sym(Id(\"apple\")), pattern: Var(\"x\"), body: Ret(Var(\"x\")) }), Case(Case { label: Sym(Id(\"banana\")), pattern: Var(\"x\"), body: Ret(Var(\"x\")) })))").unwrap();
}

#[test]
fn test_branches_1() {
    check_exp(
        "{ $apple => ret 1 }",
        "Branches(Branch(Branch { label: Sym(Id(\"apple\")), body: Ret(Num(1)) }))",
    )
    .unwrap();
}

#[test]
fn test_branches_2() {
    check_exp("{ $apple => ret 1; $banana => \\x => ret x }", 
              "Branches(Gather(Branch(Branch { label: Sym(Id(\"apple\")), body: Ret(Num(1)) }), Branch(Branch { label: Sym(Id(\"banana\")), body: Lambda(Var(\"x\"), Ret(Var(\"x\"))) })))").unwrap();
}

#[test]
fn test_project_branches() {
    check_exp_(
	      "{ $apple => ret 1; $banana => \\x => x := x } <= $apple",
	      Some("Project(Branches(Gather(Branch(Branch { label: Sym(Id(\"apple\")), body: Ret(Num(1)) }), Branch(Branch { label: Sym(Id(\"banana\")), body: Lambda(Var(\"x\"), Put(Var(\"x\"), Var(\"x\"))) }))), Sym(Id(\"apple\")))"),
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret 1])]\n]\n")
    ).unwrap();
}

#[test]
fn test_let_switch() {
    check_exp_(
        "let a = ret $apple; switch #a(1) { #a(x){ret x}; #$banana(x){ret x} }",
        Some("Let(Var(\"a\"), Ret(Sym(Id(\"apple\"))), Switch(Variant(Var(\"a\"), Num(1)), Gather(Case(Case { label: Var(\"a\"), pattern: Var(\"x\"), body: Ret(Var(\"x\")) }), Case(Case { label: Sym(Id(\"banana\")), pattern: Var(\"x\"), body: Ret(Var(\"x\")) }))))"),
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret 1])]\n]\n")
    ).unwrap();
}

#[test]
fn test_syms() {
    check_exp(
        "let _ = ret $1; let _ = ret $a; ret 0",
        "Let(Ignore, Ret(Sym(Num(1))), Let(Ignore, Ret(Sym(Id(\"a\"))), Ret(Num(0))))",
    )
    .unwrap();

    check_exp("let _ = ret $a-1; let _ = ret $a.1; ret 0",
              "Let(Ignore, Ret(Sym(Tri(Id(\"a\"), Dash, Num(1)))), Let(Ignore, Ret(Sym(Tri(Id(\"a\"), Dot, Num(1)))), Ret(Num(0))))").unwrap();

    check_exp("let _ = ret $a_1-b_2.c; ret 0",
              "Let(Ignore, Ret(Sym(Tri(Id(\"a_1\"), Dash, Tri(Id(\"b_2\"), Dot, Id(\"c\"))))), Ret(Num(0)))").unwrap();
}

#[test]
fn test_let_box_syntax() {
    let ast = "LetBx(Var(\"f\"), Ret(Bx(BxVal { bxes: BxesEnv({}), name: None, code: Lambda(Var(\"x\"), Lambda(Var(\"y\"), Put(Var(\"x\"), Var(\"y\")))) })), App(App(Extract(Var(\"f\")), Sym(Id(\"a\"))), Num(1)))";

    // 0. most verbose, with least special syntax.
    check_exp("let box f = ret {\\x => \\y => x := y}; f $a 1", ast).unwrap();

    // 1. the "ret" keyword is optional when we give a literal box value.
    check_exp("let box f = {\\x => \\y => x := y}; f $a 1", ast).unwrap();

    // 2. the "let" keyword (and '=') is also optional when we give a literal box value.
    check_exp("box f {\\x => \\y => x := y}; f $a 1", ast).unwrap();
}

#[test]
fn test_rec_box_syntax() {
    check_exp_(
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

    check_exp_(
        "let box put_ = ret {\\x => \\y => x := y}; #$n { put_ $a 1 }",
        None,
        Some(result),
    )
    .unwrap();

    // shorter syntax.
    check_exp_(
        "box put_ {\\x => \\y => x := y}; #$n { put_ $a 1 }",
        None,
        Some(result),
    )
    .unwrap();
}

#[test]
fn test_put_link() {
    check_exp_("let _ = $s := 42; &$s",
               None,
               Some("fumola [\n  store = [s => 42];\n  procs = [% => halted([put s <= 42; link $s => !s])]\n]\n")).unwrap()
}

#[test]
fn test_put_link_get() {
    check_exp_("let _ = $s := 42; @`(&$s)",
               None,
               Some("fumola [\n  store = [s => 42];\n  procs = [% => halted([put s <= 42; link $s => !s; get s => 42])]\n]\n")).unwrap()
}

#[test]
fn test_link_waiting_for_ptr() {
    check_exp_("&$s",
               None,
               Some("fumola [\n  store = [];\n  procs = [% => waitingForPtr([trace = []; stack = []; bxes = []; vals = []; cont = &$s], s)]\n]\n")).unwrap()
}

#[test]
fn test_link_invalid_proc() {
    check_exp_("&~s",
               None,
               Some("fumola [\n  store = [];\n  procs = [% => error(invalidProc(s), [trace = []; stack = []; bxes = []; vals = []; cont = &~s])]\n]\n")).unwrap()
}

#[test]
fn test_link_wait_for_halt() {
    check_exp_("let p = ~$p { ret 42 }; &p",
               None,
               Some("fumola [\n  store = [p => ~p];\n  procs = [% => halted([link ~p => 42]); p => halted([ret 42])]\n]\n")).unwrap()
}

#[test]
fn test_spawn() {
    check_exp_("~$x { ret 1 }", None, None).unwrap()
}

#[test]
fn test_nest_spawn() {
    check_exp_("#$n{ ~$x { ret 1 } }", None, None).unwrap()
}

#[test]
fn test_let_spawn() {
    check_exp_("let r = ret 1 ; ~$x { ret r }", None, None).unwrap()
}

#[test]
fn test_cbpv_convert() {
    check_exp_(
        "box id3 {\\x => \\y => \\z => ret x}; box one {ret 1}; box two {ret 2}; box three {ret 3}; id3 `(one) `(two) `(three)",
        None,
        Some("fumola [\n  store = [];\n  procs = [% => halted([ret 1])]\n]\n")).unwrap();
}

/// Fumola tools
#[derive(StructOpt, Debug, Clone)]
#[structopt(
    name = "fumola",
    setting = clap::AppSettings::DeriveDisplayOrder
)]
pub struct CliOpt {
    /// Trace-level logging (most verbose)
    #[structopt(short = "t", long = "trace-log")]
    pub log_trace: bool,
    /// Debug-level logging (medium verbose)
    #[structopt(short = "d", long = "debug-log")]
    pub log_debug: bool,
    /// Coarse logging information (not verbose)
    #[structopt(short = "L", long = "log")]
    pub log_info: bool,

    #[structopt(subcommand)]
    pub command: CliCommand,
}

#[derive(StructOpt, Debug, Clone)]
pub enum CliCommand {
    #[structopt(
        name = "completions",
        about = "Generate shell scripts for auto-completions."
    )]
    Completions {
        shell: Shell,
    },
    Check {
        input: String,
    },
}

fn init_log(level_filter: log::LevelFilter) {
    use env_logger::{Builder, WriteStyle};
    let mut builder = Builder::new();
    builder
        .filter(None, level_filter)
        .write_style(WriteStyle::Always)
        .init();
}

fn main() -> OurResult<()> {
    info!("Starting...");
    let cli_opt = CliOpt::from_args();
    info!("Init log...");
    init_log(
        match (cli_opt.log_trace, cli_opt.log_debug, cli_opt.log_info) {
            (true, _, _) => log::LevelFilter::Trace,
            (_, true, _) => log::LevelFilter::Debug,
            (_, _, true) => log::LevelFilter::Info,
            (_, _, _) => log::LevelFilter::Warn,
        },
    );
    info!("Evaluating CLI command: {:?} ...", &cli_opt.command);
    let () = match cli_opt.command {
        CliCommand::Check { input: i } => {
            check_exp_(i.as_str(), None, None).unwrap();
        }
        CliCommand::Completions { shell: s } => {
            // see also: https://clap.rs/effortless-auto-completion/
            CliOpt::clap().gen_completions_to("caniput", s, &mut io::stdout());
            info!("done");
        }
    };
    Ok(())
}
