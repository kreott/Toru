use crate::parser::Program;
use crate::parser::{Expr, Stmt};

pub fn tokens_to_asm(program: &Program) -> String {
    let mut output = String::new();
    output.push_str("global _start\n_start:\n");

    for stmt in &program.stmts {
        match stmt {
            Stmt::Return(expr) => {
                match expr {
                    Expr::IntLit(val) => {
                        output.push_str("    mov rax, 60\n");
                        output.push_str(&format!("    mov rdi, {}\n", val));
                        output.push_str("    syscall\n");
                    }
                }
            }
        }
    }

    output
}