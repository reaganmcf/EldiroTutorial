use miette::Result;
use parser::{parse, source::Source};
use interpreter::{interpret, env::Env};
use reedline::{DefaultPrompt, Reedline, Signal};

fn main() -> Result<()> {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    let mut env = Env::new();

    let args: Vec<String> = std::env::args().collect();
    let show_parse = args.contains(&String::from("-p"));

    loop {
        let sig = line_editor.read_line(&prompt).unwrap();
        match sig {
            Signal::Success(buffer) => {
                let source = Source::new(&buffer, "stdin".to_string());
                let parse = parse(source)?;

                if show_parse {
                    println!("{:#?}", parse);
                }

                interpret(parse, Some(&mut env))?;
            }
            Signal::CtrlD | Signal::CtrlC => {
                println!("\nAborted!");
                break Ok(());
            }
        }
    }
}
