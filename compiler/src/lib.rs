use serde::{Deserialize, Serialize};

pub mod codegen;

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub theme: String,
    pub entry: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Screen {
    pub name: String,
    pub states: Vec<State>,
    pub layout: Layout,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    pub name: String,
    pub type_name: String,
    pub initial_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layout {
    pub component: String,
    pub children: Vec<Component>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    pub text: Option<String>,
    pub modifiers: Vec<String>,
}

pub fn parse_aefl(input: &str) -> Result<Vec<Screen>, String> {
    let mut screens = Vec::new();
    let lines: Vec<&str> = input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();
    
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("screen") {
            let name = line.split_whitespace().nth(1).unwrap_or("Unknown").replace("{", "");
            let mut states = Vec::new();
            let mut children = Vec::new();
            i += 1;
            while i < lines.len() && !lines[i].starts_with("}") {
                let inner_line = lines[i];
                if inner_line.starts_with("state") {
                    let parts: Vec<&str> = inner_line.split(|c| c == ':' || c == '=' || c == ' ').filter(|s| !s.is_empty()).collect();
                    if parts.len() >= 4 {
                        states.push(State {
                            name: parts[1].to_string(),
                            type_name: parts[2].to_string(),
                            initial_value: parts[3].to_string(),
                        });
                    }
                } else if inner_line.contains("Text") || inner_line.contains("Button") {
                    children.push(Component {
                        name: inner_line.split('(').next().unwrap_or("Unknown").to_string(),
                        text: Some("...".to_string()),
                        modifiers: vec![],
                    });
                }
                i += 1;
            }
            screens.push(Screen {
                name,
                states,
                layout: Layout {
                    component: "Column".to_string(),
                    children,
                },
            });
        }
        i += 1;
    }
    
    Ok(screens)
}
