use crate::Screen;

pub fn generate_flutter(screens: &[Screen]) -> String {
    let mut code = String::from("import 'package:flutter/material.dart';\n\n");
    for screen in screens {
        code.push_str(&format!(
            "class {} extends StatelessWidget {{\n  @override\n  Widget build(BuildContext context) {{\n    return Scaffold(\n      body: Column(\n        children: [\n",
            screen.name
        ));
        for child in &screen.layout.children {
            code.push_str(&format!("          Text('{}'),\n", child.name));
        }
        code.push_str("        ],\n      ),\n    );\n  }\n}\n\n");
    }
    code
}

pub fn generate_kotlin(screens: &[Screen]) -> String {
    let mut code = String::from("import androidx.compose.runtime.*\nimport androidx.compose.foundation.layout.*\n\n");
    for screen in screens {
        code.push_str(&format!(
            "@Composable\nfun {}() {{\n  Column() {{\n",
            screen.name
        ));
        for child in &screen.layout.children {
            code.push_str(&format!("    Text(text = \"{}\")\n", child.name));
        }
        code.push_str("  }\n}\n\n");
    }
    code
}

pub fn generate_swift(screens: &[Screen]) -> String {
    let mut code = String::from("import SwiftUI\n\n");
    for screen in screens {
        code.push_str(&format!(
            "struct {}: View {{\n  var body: some View {{\n    VStack {{\n",
            screen.name
        ));
        for child in &screen.layout.children {
            code.push_str(&format!("      Text(\"{}\")\n", child.name));
        }
        code.push_str("    }\n  }\n}\n\n");
    }
    code
}
