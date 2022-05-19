use structopt::StructOpt;

use log::info;
use structopt::{clap, clap::Shell};

use std::io;

use fumola::error::OurResult;

fn check_exp(input: &str, ast: &str) {
    let expr = fumola::parser::ExpParser::new().parse(input).unwrap();
    assert_eq!(&format!("{:?}", expr), ast);
}

fn check_net(initial: &str, halted: &str) {
    let initial = fumola::parser::NetParser::new().parse(initial).unwrap();
    let halted = fumola::parser::TraceNetParser::new().parse(halted).unwrap();
    // to do -- run ast and check final config against halted
    println!("initial = {:?}", initial);
    println!("halted = {:?}", halted);
}

#[test]
fn test_put_get() {
    check_exp(
        "@`($a := 1)",
        "Get(CallByValue(Put(Sym(Id(\"a\")), Num(1))))",
    );
}

#[test]
fn test_let_put_get() {
    check_exp(
        "let x = $a := 1; @x",
        "Let(Id(\"x\"), Put(Sym(Id(\"a\")), Num(1)), Get(Var(\"x\")))",
    );
}

#[test]
fn test_nest() {
    check_exp("# $311 { ret 311 }", "Nest(Sym(Num(311)), Ret(Num(311)))");
}

#[test]
fn test_syms() {
    check_exp(
        "let _ = ret $1; let _ = ret $a; ret 0",
        "Let(Ignore, Ret(Sym(Num(1))), Let(Ignore, Ret(Sym(Id(\"a\"))), Ret(Num(0))))",
    );

    check_exp("let _ = ret $a-1; let _ = ret $a.1; ret 0",
                    "Let(Ignore, Ret(Sym(Tri(Id(\"a\"), Dash, Num(1)))), Let(Ignore, Ret(Sym(Tri(Id(\"a\"), Dot, Num(1)))), Ret(Num(0))))");

    check_exp("let _ = ret $a_1-b_2.c; ret 0",
                    "Let(Ignore, Ret(Sym(Tri(Id(\"a_1\"), Dash, Tri(Id(\"b_2\"), Dot, Id(\"c\"))))), Ret(Num(0)))");
}

#[test]
fn test_let_box() {
    // box f contains code that, when given a symbol and a value, puts the value at that symbol.
    check_exp("let box f = {\\x => \\y => x := y}; f $a 1",
                    "LetBx(Id(\"f\"), Bx(Lambda(Id(\"x\"), Lambda(Id(\"y\"), Put(Var(\"x\"), Var(\"y\"))))), App(App(Extract(Var(\"f\")), Sym(Id(\"a\"))), Num(1)))");
}

#[test]
fn test_net_put_link_get() {
    // By linking, doing b awaits the final result of first doing a.
    // doing a produces an address !a-x written with 137, which doing b
    // reads and returns as its result.

    // not sure about the "!" syntax for raw, global addresses.
    check_net(
        "doing a { $x := 137 } | doing b { @`(@`(&$a)) }",
        r##"
        proc a { put a-x <= 137 };
        proc b { link $a => ~a;
                 get a => !a-x;
                 get a-x => 137 }
        ;;
        being a { !a-x }
      | being b { 137 }
        "##,
    );
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
    Completions { shell: Shell },
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
        CliCommand::Completions { shell: s } => {
            // see also: https://clap.rs/effortless-auto-completion/
            CliOpt::clap().gen_completions_to("caniput", s, &mut io::stdout());
            info!("done");
        }
    };
    Ok(())
}
