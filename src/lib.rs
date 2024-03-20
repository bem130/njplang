use wasm_bindgen::prelude::*;
use std::str;
use peg::{self, str::LineCol};
use serde::Serialize;

fn debug(text: String) {
    web_sys::console::log_1(&JsValue::from(text));
}

#[derive(Debug,Serialize)]
pub enum Expr {
    // literal type
    Int(i64),
    Float(f64),
    Str(String),
    // operation
    Opr(String,Box<Expr>,Box<Expr>),
    UOpr(String,Box<Expr>),
}
peg::parser! {
    grammar parser() for str {
        pub rule calc() -> Expr
            = precedence! {
                _ "+" _ r:@ { Expr::UOpr("+".to_string(), Box::new(r)) }
                _ "-" _ r:@ { Expr::UOpr("-".to_string(), Box::new(r)) }
                l:(@) _ "+" _ r:@ { Expr::Opr("+".to_string(),Box::new(l), Box::new(r)) }
                l:(@) _ "-" _ r:@ { Expr::Opr("-".to_string(),Box::new(l), Box::new(r)) }
                l:(@) _ "*" _ r:@ { Expr::Opr("*".to_string(),Box::new(l), Box::new(r)) }
                l:(@) _ "/" _ r:@ { Expr::Opr("/".to_string(),Box::new(l), Box::new(r)) }
                n:literal() { n }
                "(" _ c:calc() _ ")" { c }
            }
            rule _()
                = quiet!{ " "* }

        rule literal() -> Expr
            = n:$(['0'..='9']+"."['0'..='9']+) { (Expr::Float(n.parse().unwrap())) }
            / n:$(['0'..='9']+) { (Expr::Int(n.parse().unwrap())) }
            / "\"" s:$((("\\\"")/[^'\"'])*) "\"" {Expr::Str(s.to_string())}
    }
}
//'ParseError { location: LineCol { line: 1, column: 8, offset: 7 }, expected: ExpectedSet { expected: {"\\"*\\"", "\\"+\\"", "\\"-\\"", "\\"/\\"", "EOF"} } }'
#[wasm_bindgen]
pub fn parse(input: &str) -> JsValue {
    match parser::calc(input) {
        Ok(val) =>{
            return JsValue::from(serde_json::to_string(&val).unwrap());
        }
        Err(err) =>{
            return JsValue::from(&format!("{:?}", err));
        }
    }
}

#[wasm_bindgen]
pub fn parse2(input: &str) -> JsValue {
    match parser::calc(input) {
        Ok(val) =>{
            //debug(format!("Done {:?}", val));
            return JsValue::from(&format!("{:?}", val));
        }
        Err(err) =>{
            //debug(format!("Err  {:?}", err));
            return JsValue::from(&format!("{:?}", err));
        }
    }
}