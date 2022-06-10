use structopt::StructOpt;

use log::info;
use std::collections::HashMap;
use std::io;
use structopt::{clap, clap::Shell};

use fumola::{
    ast::{
        step::{Proc, System},
        Exp, Sym,
    },
    error::OurResult,
};

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
        store: HashMap::new(),
        trace: vec![],
        procs,
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
    println!("final system:\n{:?}", &sys);
    match final_sys {
        None => (),
        Some(s) => {
            assert_eq!(&format!("{:?}", &sys), s);
        }
    };
    Ok(())
}

fn check_exp(input: &str, ast: &str) -> Result<(), ()> {
    check_exp_(input, Some(ast), None)
}

fn check_net(initial: &str, halted: &str) {
    let initial = fumola::parser::NetParser::new().parse(initial).unwrap();
    let halted = fumola::parser::TraceNetParser::new().parse(halted).unwrap();
    // to do -- run ast and check final config against halted
    println!("initial = {:?}", initial);
    println!("halted = {:?}", halted);
}

#[test]
fn test_put() {
    check_exp_(
        "$a := 1",
        None,
        Some("System { store: {Id(\"a\"): Num(1)}, trace: [], procs: {None: Halted(Halted { retval: Sym(Id(\"a\")) })} }")
    ).unwrap();
}

#[test]
fn test_nest_put() {
    check_exp_(
        "#$n { $a := 1 }",
        None,
        Some("System { store: {Nest(Id(\"n\"), Id(\"a\")): Num(1)}, trace: [], procs: {None: Halted(Halted { retval: Sym(Nest(Id(\"n\"), Id(\"a\"))) })} }")
    ).unwrap();
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
        Some("System { store: {Nest(Id(\"n\"), Id(\"a\")): Num(3)}, trace: [], procs: {None: Halted(Halted { retval: Num(3) })} }")).unwrap();
}

#[test]
fn test_get_undef() {
    check_exp_(
        "@$s",
        None,
        Some("System { store: {}, trace: [], procs: {None: Error(Running { env: Env { vals: {}, bxes: {} }, stack: [], cont: Get(Sym(Id(\"s\"))), trace: [] }, Undefined(Id(\"s\")))} }")).unwrap();
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
        Some("System { store: {}, trace: [], procs: {None: Halted(Halted { retval: Num(311) })} }"),
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
        Some("System { store: {}, trace: [], procs: {None: Halted(Halted { retval: Num(1) })} }")
    ).unwrap();
}

#[test]
fn test_let_switch() {
    check_exp_(
        "let a = ret $apple; switch #a(1) { #a(x){ret x}; #$banana(x){ret x} }",
        Some("Let(Var(\"a\"), Ret(Sym(Id(\"apple\"))), Switch(Variant(Var(\"a\"), Num(1)), Gather(Case(Case { label: Var(\"a\"), pattern: Var(\"x\"), body: Ret(Var(\"x\")) }), Case(Case { label: Sym(Id(\"banana\")), pattern: Var(\"x\"), body: Ret(Var(\"x\")) }))))"),
        Some("System { store: {}, trace: [], procs: {None: Halted(Halted { retval: Num(1) })} }")
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
fn test_let_box() {
    let ast = "LetBx(Var(\"f\"), Ret(Bx(BxVal { bxes: {}, name: None, code: Lambda(Var(\"x\"), Lambda(Var(\"y\"), Put(Var(\"x\"), Var(\"y\")))) })), App(App(Extract(Var(\"f\")), Sym(Id(\"a\"))), Num(1)))";

    // box f contains code that, when given a symbol and a value, puts the value at that symbol.
    check_exp("let box f = ret {\\x => \\y => x := y}; f $a 1", ast).unwrap();

    // the "ret" keyword is optional when we give a literal box value
    check_exp("let box f = {\\x => \\y => x := y}; f $a 1", ast).unwrap();
}

#[test]
fn test_put_link() {
    check_exp_("let _ = $s := 42; &$s",
               None,
               Some("System { store: {Id(\"s\"): Num(42)}, trace: [], procs: {None: Halted(Halted { retval: Sym(Id(\"s\")) })} }")).unwrap()
}

#[test]
fn test_open_link() {
    check_exp_("&$s",
               None,
               Some("System { store: {}, trace: [], procs: {None: Waiting(Running { env: Env { vals: {}, bxes: {} }, stack: [], cont: Link(Sym(Id(\"s\"))), trace: [] }, Id(\"s\"))} }")).unwrap()
}

#[test]
fn test_net_put_link_get() {
    // By linking, doing b awaits the final result of first doing a.
    // doing a produces an address !a-x written with 137, which doing b
    // reads and returns as its result.

    // not sure about the "!" syntax for raw, global addresses.
    check_net(
        "doing a { $x := 137 } || doing b { @`(@`(&$a)) }",
        r##"
        proc a { put a-x <= 137 };
        proc b { link $a => ~a;
                 get a => !a-x;
                 get a-x => 137 }
        ;;
         being a { !a-x }
      || being b { 137 }
        "##,
    )
}

#[test]
fn test_cbpv_convert() {
    check_exp_(
        "box id3 {\\x => \\y => \\z => ret x}; box one {ret 1}; box two {ret 2}; box three {ret 3}; id3 `(one) `(two) `(three)",
        None,
        Some("System { store: {}, trace: [], procs: {None: Halted(Halted { retval: Num(1) })} }")).unwrap();
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
