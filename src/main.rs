mod pinentry;
mod tui;

use pinentry::Command;
use std::path::Path;
use tui::Tui;

fn main() -> anyhow::Result<()> {
    println!("OK Please go ahead");

    let mut tui = Tui::new();

    let path = Path::new("/home/orhun/gh/pinentry-ratatui/test.txt");

    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;

        match Command::try_from(buffer.trim().to_string()) {
            Ok(Command::SetDesc(desc)) => {
                tui.data.desc = Some(desc.clone());
            }
            Ok(Command::SetPrompt(prompt)) => {
                tui.data.prompt = Some(prompt.clone());
            }
            Ok(Command::Option(option)) => match option {
                pinentry::Option::Ttyname(ttyname) => {
                    tui.data.ttyname = Some(ttyname.clone());
                    std::fs::write(
                        path,
                        format!("{:?} {}", std::fs::metadata(&ttyname), ttyname),
                    )?;
                }
            },
            Ok(Command::GetPin) => {
                let passphrase = match tui.get_pin() {
                    Ok(passphrase) => passphrase,
                    Err(err) => {
                        std::fs::write(
                            path,
                            format!(
                                "{}\n{:?} -> {}",
                                std::fs::read_to_string(path)?,
                                tui.data.ttyname,
                                err.to_string()
                            ),
                        )?;
                        return Err(err);
                    }
                };
                println!("D {passphrase}");
                return Ok(());
            }
            _ => {
                eprintln!("Invalid command");
            }
        }

        std::fs::write(
            path,
            format!("{}\n{buffer}", std::fs::read_to_string(path)?),
        )?;

        println!("OK");
    }
}
