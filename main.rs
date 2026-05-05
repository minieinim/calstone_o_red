use std::env::args;
use std::process::exit;
use std::fs::read_to_string;

#[derive(Debug)]
enum ConstType {
 Str(String),
 Int(i32),
}

#[derive(Debug)]
enum Ast {
 Const(ConstType),
 Expr(Vec<Ast>),
 Var(String),
 Assign(String,Vec<Ast>),
 Call(String,Vec<Ast>),
}

struct Parser {
 tokens:Vec<Token>,
 len:usize,
 pos:usize,
 ct:Token,
}

impl Parser {
 fn init(tokens:Vec<Token>)->Parser {
  Parser{tokens:tokens.clone(),len:tokens.len(),pos:0,ct:tokens[0].clone()}
 }

 fn advance(&mut self) {
  self.pos+=1;
  if self.pos<self.len {
   self.ct=self.tokens[self.pos].clone();
  }
  return;
 }

 fn next(&mut self)->Option<Token> {
  if self.pos+1<self.len {
   return Some(self.tokens[self.pos+1].clone());
  }
  None
 }

 fn parse(&mut self,state:i32)->Vec<Ast> {
  let mut res=Vec::<Ast>::with_capacity(self.len);
  while self.pos<self.len {
   match self.ct.clone() {
    Token::Const(s,i) => {
     if s.is_some() {
      res.push(Ast::Const(ConstType::Str(s.clone().unwrap())));
     } else if i.is_some() {
      res.push(Ast::Const(ConstType::Int(i.unwrap())));
     } else {
      println!("Undefined constant");
      exit(1);
     }
    },
    Token::Var(s) => {
     if state!=1 && self.next().is_some() {
      let nt=self.next();
      match nt.unwrap() {
       Token::Sep => (),
       _ => {
        let mut name=String::from(s);
        self.advance();
        res.push(Ast::Assign(name.clone(),self.parse(1)));
       }
      }
     } else {
      res.push(Ast::Var(s));
     }
    },
    Token::Func(s) => {
     let mut name=String::from(s);
     self.advance();
     res.push(Ast::Call(name.clone(),self.parse(1)));
     name.clear();
    },
    Token::Paren(c) => {
     if state==2 && c==')' { res.shrink_to_fit(); return res; }
     else if c=='(' {
      self.advance();
      res.push(Ast::Expr(self.parse(2)));
     }
    },
    Token::Sep => {
     if state==1 { res.shrink_to_fit(); return res; }
    },
   }
   self.advance();
  }
  res.shrink_to_fit();
  res
 }
}

#[derive(Debug,Clone)]
enum Token {
 Const(Option<String>,Option<i32>),
 Var(String),
 Func(String),
 Paren(char),
 Sep,
}

struct Lexer {
 code:String,
 len:usize,
 pos:usize,
 cc:char,
}

impl Lexer {
 fn init(code:String)->Lexer {
  Lexer{code:code.clone(),len:code.len(),pos:0,cc:code.chars().nth(0).unwrap()}
 }

 fn advance(&mut self) {
  self.pos+=1;
  if self.pos<self.code.len() {
   self.cc=self.code.chars().nth(self.pos).unwrap();
  }
  return;
 }

 fn lex(&mut self)->Vec<Token> {
  let parens=String::from("()");
  let mut word=String::with_capacity(self.code.len());
  let mut res=Vec::<Token>::with_capacity(self.code.len());
  let mut a:i32=0;
  while self.pos<self.len {
   if self.cc=='@' {
    self.advance();
    while self.pos<self.len && (self.cc.is_alphabetic() || self.cc=='_') {
     word.push(self.cc);
     self.advance();
    }
    res.push(Token::Func(word.clone()));
    word.clear();
   } else if self.cc=='$' {
    self.advance();
    while self.pos<self.len && (self.cc.is_alphabetic() || self.cc=='_') {
     word.push(self.cc);
     self.advance();
    }
    res.push(Token::Var(word.clone()));
    word.clear();
   } else if self.cc.is_ascii_digit() {
    while self.pos<self.len && self.cc.is_ascii_digit() {
     word.push(self.cc);
     self.advance();
    }
    res.push(Token::Const(None,Some(word.parse::<i32>().unwrap())));
    word.clear();
   } else if self.cc=='"' {
    self.advance();
    a+=1;
    while self.pos<self.len {
     if self.cc=='"' { a-=1; }
     if a==0 { break; }
     word.push(self.cc);
     self.advance();
    }
    res.push(Token::Const(Some(word.clone()),None));
    word.clear();
   }
   if self.pos<self.len {
    if parens.find(self.cc).is_some() {
     res.push(Token::Paren(self.cc));
    } else if self.cc==';' {
     res.push(Token::Sep);
    }
   }
   self.advance();
  }
  res.shrink_to_fit();
  res
 }
}

fn main() {
 let argv:Vec<String>=args().collect();
 if argv.len()<2 {
  println!("Expected a file");
  exit(1);
 }
 if !argv[1].ends_with(".mtknfkktr") {
  println!("Expected extension \".mtknfkktr\"");
  exit(1)
 }
 let code=read_to_string(argv[1].clone()).unwrap();
 let mut lexer=Lexer::init(code);
 let tokens=lexer.lex();
 let mut parser=Parser::init(tokens);
 let ast=parser.parse(0);
 for i in ast {
  println!("{:?}",i);
 }
 ()
}
