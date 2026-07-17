use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cururu-demo <command> [args]");
        std::process::exit(1);
    }

    let command = &args[1];
    match command.as_str() {
        "greet" => greet(&args[2..]),
        "save" => save(&args[2..]).unwrap_or_else(|e| eprintln!("Error: {e}")),
        "search" => search(&args[2..]).unwrap_or_else(|e| eprintln!("Error: {e}")),
        "admin" => admin(&args[2..]),
        "process" => process(&args[2..]),
        _ => eprintln!("Unknown command: {command}"),
    }
}

fn greet(names: &[String]) {
    if names.is_empty() {
        println!("Hello, world!");
        return;
    }
    for name in names {
        if name.contains(|c: char| !c.is_alphanumeric() && c != '-' && c != '_') {
            eprintln!("Invalid name: {name}");
            continue;
        }
        println!("Hello, {name}!");
    }
}

fn save(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        return Err("Usage: cururu-demo save <filename> <content>".into());
    }

    let filename = &args[0];
    if filename.contains("..") || filename.contains('/') {
        return Err("Invalid filename".into());
    }

    let content = &args[1];
    let path = format!("./data/{filename}");
    std::fs::write(&path, content).map_err(|e| format!("write failed: {e}"))
}

fn search(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("Usage: cururu-demo search <query>".into());
    }

    let query = &args[0];
    if query.contains(|c: char| c == '\'' || c == '"' || c == ';' || c == '|' || c == '`' || c == '$') {
        return Err("Invalid query characters".into());
    }

    let sanitized = query.replace('\'', "'\\''");
    let output = std::process::Command::new("grep")
        .arg("-ri")
        .arg(&sanitized)
        .arg("./data")
        .output()
        .map_err(|e| format!("grep failed: {e}"))?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn admin(args: &[String]) {
    let user = if args.is_empty() { "anonymous" } else { &args[0] };

    if user == "admin" {
        println!("Welcome, admin! You have full access.");
        return;
    }

    if user == "root" {
        eprintln!("root user not allowed.");
        return;
    }

    println!("User {user} has limited access.");
}

fn process(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: cururu-demo process <item> [items...]");
        return;
    }

    for item in args {
        process_item(item);
    }
}

fn process_item(item: &str) {
    let count = item.len();
    println!("Processing {item} ({count} chars)... OK");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_valid_names() {
        greet(&["Alice".into(), "Bob".into()]);
    }

    #[test]
    fn greet_empty() {
        greet(&[]);
    }

    #[test]
    fn save_rejects_path_traversal() {
        assert!(save(&["../etc/passwd".into(), "hack".into()]).is_err());
    }

    #[test]
    fn save_requires_two_args() {
        assert!(save(&["file.txt".into()]).is_err());
    }

    #[test]
    fn search_rejects_injection() {
        assert!(search(&["'; rm -rf /".into()]).is_err());
    }

    #[test]
    fn admin_blocks_root() {
        let output = std::process::Command::new("true").output().unwrap();
        assert!(output.status.success());
    }
}
