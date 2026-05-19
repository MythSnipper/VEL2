#[derive(Default, Debug, Clone)]


pub struct Optimizer {}

impl Optimizer {
    pub fn new() -> Self {
        Self {}
    }
    pub fn run(&self, asm: &str) -> Result<String, String> {
        let mut current = asm.to_string();

        let mut cycle = 1;
        loop {
            println!("Optimizer cycle {cycle}");
            let (next, changed) = self.peephole_once(&current)?;

            if !changed {
                return Ok(next);
            }
            current = next;

            cycle += 1;
        }
    }
    fn peephole_once(&self, asm: &str) -> Result<(String, bool), String> {
        let lines: Vec<&str> = asm.lines().collect();
        let mut out: Vec<String> = Vec::new();

        let mut changed = false;
        let mut i = 0;

        while i < lines.len() {
            if i + 1 < lines.len() {
                let line1 = lines[i];
                let line2 = lines[i + 1];

                if let Some(src) = self.parse_unary_instr(line1, "push") {
                    if let Some(dst) = self.parse_unary_instr(line2, "pop") {
                        if self.is_gpr64(&src) && self.is_gpr64(&dst) {
                            if src == dst {
                                /*
                                out.push(format!(
                                    "    ; peephole removed push/pop {}",
                                    src
                                ));
                                */
                            }
                            else {
                                
                                out.push(format!(
                                    "    mov {}, {} ; peephole push/pop",
                                    dst,
                                    src
                                ));
                                
                            }

                            changed = true;
                            i += 2;
                            continue;
                        }

                        if self.is_simple_immediate_push_operand(&src) && self.is_gpr64(&dst) {
                            let imm = self.strip_push_size(&src);
                            
                            out.push(format!(
                                "    mov {}, {} ; peephole push-imm/pop",
                                dst,
                                imm
                            ));
                            
                            changed = true;
                            i += 2;
                            continue;
                        }
                    }
                }
            }

            out.push(lines[i].to_string());
            i += 1;
        }

        Ok((out.join("\n") + "\n", changed))
    }
    fn parse_unary_instr(&self, line: &str, mnemonic: &str) -> Option<String> {
        let code = match line.split_once(';') {
            Some((before_comment, _)) => before_comment.trim(),
            None => line.trim(),
        };

        if code.is_empty() {
            return None;
        }

        let mut parts = code.split_whitespace();

        let op = parts.next()?;

        if op != mnemonic {
            return None;
        }

        let operand = parts.collect::<Vec<&str>>().join(" ");

        if operand.is_empty() {
            return None;
        }

        Some(operand)
    }
    fn is_gpr64(&self, operand: &str) -> bool {
        matches!(
            operand,
            "rax"
                | "rbx"
                | "rcx"
                | "rdx"
                | "rsi"
                | "rdi"
                | "rbp"
                | "rsp"
                | "r8"
                | "r9"
                | "r10"
                | "r11"
                | "r12"
                | "r13"
                | "r14"
                | "r15"
        )
    }
    fn is_simple_immediate_push_operand(&self, operand: &str) -> bool {
        let operand = self.strip_push_size(operand);

        if operand.starts_with("0x") {
            return operand[2..].chars().all(|c| c.is_ascii_hexdigit());
        }

        if operand.starts_with("-") {
            return operand[1..].chars().all(|c| c.is_ascii_digit());
        }

        operand.chars().all(|c| c.is_ascii_digit())
    }
    fn strip_push_size<'a>(&self, operand: &'a str) -> &'a str {
        if let Some(rest) = operand.strip_prefix("qword ") {
            return rest.trim();
        }

        operand.trim()
    }
}