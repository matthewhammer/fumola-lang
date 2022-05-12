use structopt::StructOpt;

use log::info;
use structopt::{clap, clap::Shell};

use std::io;

use fumola::error::OurResult;

#[test]
fn test_put_get() {
    let expr = fumola::parser::ExpParser::new()
        .parse("@`($a := 1)")
        .unwrap();
    assert_eq!(
        &format!("{:?}", expr),
        "Get(CallByValue(Put(Sym(Id(\"a\")), Num(1))))"
    );
}

#[test]
fn test_let_put_get() {
    let expr = fumola::parser::ExpParser::new()
        .parse("let x = $a := 1; @x")
        .unwrap();
    assert_eq!(
        &format!("{:?}", expr),
        "Let(Id(\"x\"), Put(Sym(Id(\"a\")), Num(1)), Get(Var(\"x\")))"
    );
}

#[test]
fn test_nest() {
    let expr = fumola::parser::ExpParser::new()
        .parse("# $311 { ret 311 }")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "Nest(Sym(Num(311)), Ret(Num(311)))");
}

#[test]
fn test_syms() {
    let expr = fumola::parser::ExpParser::new()
        .parse("let _ = ret $1; let _ = ret $a; let _ = ret $a-1; let _ = ret $a.1; let _ = ret $a_1-b_2.c; ret 0")
        .unwrap();
    assert_eq!(&format!("{:?}", expr),
               "Let(Ignore, Ret(Sym(Num(1))), Let(Ignore, Ret(Sym(Id(\"a\"))), Let(Ignore, Ret(Sym(Tri(Id(\"a\"), Dash, Num(1)))), Let(Ignore, Ret(Sym(Tri(Id(\"a\"), Dot, Num(1)))), Let(Ignore, Ret(Sym(Tri(Id(\"a_1\"), Dash, Tri(Id(\"b_2\"), Dot, Id(\"c\"))))), Ret(Num(0)))))))");
}

#[test]
fn test_let_box() {
    // box f contains code that, when given a symbol and a value, puts the value at that symbol.
    let expr = fumola::parser::ExpParser::new()
        .parse("let box f = {\\x => \\y => x := y}; f $a 1")
        .unwrap();
    assert_eq!(&format!("{:?}", expr),
               "LetBx(Id(\"f\"), Bx(Lambda(Id(\"x\"), Lambda(Id(\"y\"), Put(Var(\"x\"), Var(\"y\"))))), App(App(Extract(Var(\"f\")), Sym(Id(\"a\"))), Num(1)))");
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
