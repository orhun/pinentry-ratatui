#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Option {
    Ttyname(String),
}

impl TryFrom<String> for Option {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.splitn(2, '=');
        match parts.next() {
            Some("ttyname") => Ok(Self::Ttyname(parts.next().unwrap_or("").to_string())),
            _ => Err(anyhow::anyhow!("Invalid option")),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command {
    Option(Option),
    SetDesc(String),
    SetPrompt(String),
    GetPin,
}

impl TryFrom<String> for Command {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.splitn(2, ' ');
        match parts.next() {
            Some("OPTION") => Ok(Self::Option(Option::try_from(
                parts.next().unwrap_or("").to_string(),
            )?)),
            Some("SETDESC") => Ok(Self::SetDesc(parts.next().unwrap_or("").to_string())),
            Some("SETPROMPT") => Ok(Self::SetPrompt(parts.next().unwrap_or("").to_string())),
            Some("GETPIN") => Ok(Self::GetPin),
            _ => Err(anyhow::anyhow!("Invalid command")),
        }
    }
}
