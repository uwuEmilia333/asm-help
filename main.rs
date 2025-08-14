#![feature(if_let_guard)]

use std::process::exit;
use std::collections::HashMap;
use std::cmp::Ordering;

macro_rules! read {
	() => {{
		use std::io::BufRead;
		use std::io::stdin;
		let mut stdin = stdin().lock();
		let mut buf = String::new();
		if let Err(e) = stdin.read_line(&mut buf) {
			Err(e)
		}
		else {
			Ok(buf)
		}
	}};
	($msg:expr) => {{
		use std::io::BufRead;
		use std::io::Write;
		use std::io::stdin;
		use std::io::stdout;
		print!("{}", $msg);
		if let Err(e) = stdout().flush() {
			Err(e)
		}
		else {
			let mut stdin = stdin().lock();
			let mut buf = String::new();
			if let Err(e) = stdin.read_line(&mut buf) {
				Err(e)
			}
			else {
				Ok(buf)
			}
		}
	}}
}

static REGISTERS: &[&str] = &[
	"RAX",
	"RCX",
	"RDX",
	"RSI",
	"RDI",
	"RBX",
	"RBP",
	"RSP",
	"R8",
	"R9",
	"R10",
	"R11",
	"R12",
	"R13",
	"R14",
	"R15"
];

fn strncmp(a: &str, b: &str, n: usize) -> Ordering {
    a.chars().take(n).cmp(b.chars().take(n))
}

fn match_register<'a>(input: &'a str) -> Option<(&'static str, &'a str)> {
    for register in REGISTERS {
        if strncmp(input.to_ascii_uppercase().as_str(), register, 3) == Ordering::Equal {
            let rest = &input[3..];
            return Some((register, rest));
        }
    }
    None
}

fn display_info(reg: &str) {
	print!("{reg} : ");
	match reg {
		"RAX" => println!("caller-saved, return value"),
		"RCX" => println!("caller-saved, argument 4, syscall return address"),
		"RDX" => println!("caller-saved, argument 3"),
		"RSI" => println!("caller-saved, argument 2"),
		"RDI" => println!("caller-saved, argument 1"),
		"R8" => println!("caller-saved, argument 5"),
		"R9" => println!("caller-saved, argument 6"),
		"R10" => println!("caller-saved, argument 7"),
		"R11" => println!("caller-saved, argument 8, syscall rflags"),
		"RBX" => println!("callee-saved"),
		"RBP" => println!("frame pointer"),
		"R12" => println!("callee-saved"),
		"R13" => println!("callee-saved"),
		"R14" => println!("callee-saved"),
		"R15" => println!("callee-saved"),
		"RSP" => println!("stack pointer"),
		_ => eprintln!("Error: no such register: `{reg}`")
	}
}

fn main() {
	let mut scopes = HashMap::new();
	let mut purposes = HashMap::new();
	loop {
		let input = read!("> ").unwrap_or_else(|err| {
			eprintln!("Error: {err}");
			exit(1);
		});
		match input.trim() {
			s if let Some((reg, furthermore)) = match_register(s) => {
				if furthermore.trim().is_empty() {
					if let Some(purpose) = purposes.get(reg) {
						println!("{reg} : {purpose}");
					}
					else {
						println!("{reg} is unused");
					}
				}
				else if furthermore.trim() == "clear" {
					purposes.remove(reg);
					println!("{reg} is unused");
				}
				else {
					if let Some(old_purpose) = purposes.insert(reg, furthermore.trim().to_string()) {
						println!("{reg} : {old_purpose} -> {}", furthermore.trim());
					}
					else {
						println!("{reg} : {}", furthermore.trim());
					}
				}
			}
			s if let Some(name) = s.strip_prefix("end ") => {
				scopes.insert(name.to_string(), purposes);
				purposes = HashMap::new();
			}
			s if let Some(name) = s.strip_prefix("load ") => {
				if let Some(loaded) = scopes.remove(name) {
					purposes = loaded;
				}
				else {
					eprintln!("Error: no such scope");
				}
			}
			s if let Some(reg) = s.strip_prefix("about ") => {
				display_info(reg.to_ascii_uppercase().as_str());
			}
			"dump" => {
				for (register, purpose) in purposes.iter() {
					println!("{register} : {purpose}");
				}
			}
			"reset" => {
				scopes = HashMap::new();
				purposes = HashMap::new();
				println!("cleared all saved scopes and registers");
			}
			"exit" => {
				exit(0);
			}
			cmd => {
				eprintln!("Error: unknown command `{cmd}`");
			}
		}
	}
}
