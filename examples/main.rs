use drgn_knight::*;

enum Token {
    Member(String),
    Access,
    Deref,
}

/* FIXME: This is an ugly lexer for the C structure experssion :( */
struct Lexer {
    s: String,
    pos: usize,
    len: usize,
}

impl Lexer {
    pub fn new(s: String) -> Self {
        let l = s.len();
        Lexer {
            s: s,
            pos: 0,
            len: l,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let s = self.s.as_bytes();

        while self.pos < self.len {
            let c = s[self.pos] as u8;
            self.pos += 1;
            match c {
                b'.' => return Some(Token::Access),
                b'-' => {
                    if self.pos >= self.len || s[self.pos] != b'>' {
                        return None;
                    }
                    self.pos += 1;
                    return Some(Token::Deref);
                }
                _ => {
                    let start = self.pos - 1;

                    while self.pos < self.len {
                        let c = s[self.pos];
                        if c == b'.' || c == b'-' {
                            break;
                        }
                        self.pos += 1;
                    }

                    return Some(Token::Member(self.s[start..self.pos].to_string()));
                }
            }
        }

        None
    }
}

fn find_member(obj: &Object, path: &str) -> Option<Object> {
    let mut lexer = Lexer::new(path.to_string());

    /* The First token should be Token::Member */
    let mut cur_obj = Object::default();
    if let Some(token) = lexer.next_token() {
        match token {
            Token::Member(member) => {
                cur_obj = obj.deref_member(&member)?;
            }
            _ => return None,
        }
    }

    let mut prev_token = 0;
    while let Some(token) = lexer.next_token() {
        match token {
            Token::Member(member) => {
                if prev_token == 0 {
                    return None;
                }

                if prev_token == 1 {
                    cur_obj = cur_obj.member(&member)?;
                } else {
                    cur_obj = cur_obj.deref_member(&member)?;
                }

                prev_token = 0;
            }
            Token::Access => {
                if prev_token != 0 {
                    return None;
                }
                prev_token = 1;
            }
            Token::Deref => {
                if prev_token != 0 {
                    return None;
                }
                prev_token = 2;
            }
        }
    }

    Some(cur_obj)
}

fn print_obj(obj: &Object, path: String) {
    if let Some(obj) = find_member(obj, &path) {
        println!("{path}: {:x?}", obj.to_num());
    } else {
        println!("{path} is invalid");
    }
}

fn main() {
    let prog = Program::new();

    let pid = 1;
    let task = prog.find_task(pid);
    if task.is_err() {
        println!("Can't find task with pid {pid}");
        return;
    }

    let task = task.unwrap();
    let addr = task.to_num().unwrap();
    println!("Get task@{addr:x}");

    print_obj(&task, "on_cpu".to_string());
    print_obj(&task, "pid".to_string());
    print_obj(&task, "se".to_string());
    print_obj(&task, "se.vruntime".to_string());
}
