/*
--- Day 19: Aplenty ---
The Elves of Gear Island are thankful for your help and send you on your way. They even have a hang glider that someone stole from Desert Island; since you're already going that direction, it would help them a lot if you would use it to get down there and return it to them.

As you reach the bottom of the relentless avalanche of machine parts, you discover that they're already forming a formidable heap. Don't worry, though - a group of Elves is already here organizing the parts, and they have a system.

To start, each part is rated in each of four categories:

x: Extremely cool looking
m: Musical (it makes a noise when you hit it)
a: Aerodynamic
s: Shiny
Then, each part is sent through a series of workflows that will ultimately accept or reject the part. Each workflow has a name and contains a list of rules; each rule specifies a condition and where to send the part if the condition is true. The first rule that matches the part being considered is applied immediately, and the part moves on to the destination described by the rule. (The last rule in each workflow has no condition and always applies if reached.)

Consider the workflow ex{x>10:one,m<20:two,a>30:R,A}. This workflow is named ex and contains four rules. If workflow ex were considering a specific part, it would perform the following steps in order:

Rule "x>10:one": If the part's x is more than 10, send the part to the workflow named one.
Rule "m<20:two": Otherwise, if the part's m is less than 20, send the part to the workflow named two.
Rule "a>30:R": Otherwise, if the part's a is more than 30, the part is immediately rejected (R).
Rule "A": Otherwise, because no other rules matched the part, the part is immediately accepted (A).
If a part is sent to another workflow, it immediately switches to the start of that workflow instead and never returns. If a part is accepted (sent to A) or rejected (sent to R), the part immediately stops any further processing.

The system works, but it's not keeping up with the torrent of weird metal shapes. The Elves ask if you can help sort a few parts and give you the list of workflows and some part ratings (your puzzle input). For example:

px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
The workflows are listed first, followed by a blank line, then the ratings of the parts the Elves would like you to sort. All parts begin in the workflow named in. In this example, the five listed parts go through the following workflows:

{x=787,m=2655,a=1222,s=2876}: in -> qqz -> qs -> lnx -> A
{x=1679,m=44,a=2067,s=496}: in -> px -> rfg -> gd -> R
{x=2036,m=264,a=79,s=2244}: in -> qqz -> hdj -> pv -> A
{x=2461,m=1339,a=466,s=291}: in -> px -> qkq -> crn -> R
{x=2127,m=1623,a=2188,s=1013}: in -> px -> rfg -> A
Ultimately, three parts are accepted. Adding up the x, m, a, and s rating for each of the accepted parts gives 7540 for the part with x=787, 4623 for the part with x=2036, and 6951 for the part with x=2127. Adding all of the ratings for all of the accepted parts gives the sum total of 19114.

Sort through all of the parts you've been given; what do you get if you add together all of the rating numbers for all of the parts that ultimately get accepted?
*/

use std::fs::read_to_string;
use std::collections::HashMap;

/* ========== lexer and scanner =========== */

#[derive(Debug, Clone)]
enum Expr {
    Return(bool),
    Literal(i64),
    Variable(String),
    Assign(Box<Expr>, Box<Expr>),
    If(Box<Expr>,Box<Expr>),
    GreaterThan(Box<Expr>,Box<Expr>),
    LowerThan(Box<Expr>,Box<Expr>),
    FnCall(String),
    Fn(String,Vec<Expr>),
    Closure(Vec<Expr>)
}

type ParseResult = Result<Expr,String>;

fn parse_return(s: &str) -> ParseResult {
    match s {
        "A" => Ok(Expr::Return(true)),
        "R" => Ok(Expr::Return(false)),
        _ => Err("not a return statement".to_string())
    }
}

fn parse_literal(s: &str) -> ParseResult {
    let s = s.parse::<i64>();
    let s = s.map_err(|_| "not a literal".to_string());
    s.map(|l| Expr::Literal(l))
}

fn parse_variable(s: &str) -> ParseResult {
    if s.len() > 1 || !"xmas".contains(s.chars().nth(0).unwrap()) {
        return Err("not a variable".to_string());
    }
    Ok(Expr::Variable(s.to_string()))
}

fn parse_assign(s: &str) -> ParseResult {
    let s: Vec<&str> = s.split("=").collect();
    if s.len() == 1 {
        return Err("not an assignment".to_string());
    }

    let v = parse_variable(s[0]);
    if !v.is_ok() {
        return Err("left side isn't a variable".to_string());
    }

    let right = parse_expr(s[1]);
    if !right.is_ok() {
        return Err("right side isn't an expression".to_string());
    }
    

    Ok(Expr::Assign(Box::new(v.unwrap()), Box::new(right.unwrap())))    
}

fn parse_if(s: &str) -> ParseResult {
    let s: Vec<&str> = s.split(":").collect();
    if s.len() == 1 {
        return Err("not an if statement".to_string());
    }

    let left = parse_expr(s[0]);
    if !left.is_ok() {
        return Err("left side isn't an expression".to_string());
    }
    
    let right = parse_expr(s[1]);
    if !right.is_ok() {
        return Err("right side isn't an expression".to_string());
    }
    

    Ok(Expr::If(Box::new(left.unwrap()), Box::new(right.unwrap())))    
}

fn parse_gt(s: &str) -> ParseResult {
    let s: Vec<&str> = s.split(">").collect();
    if s.len() == 1 {
        return Err("not a greaterthan statement".to_string());
    }

    let left = parse_expr(s[0]);
    if !left.is_ok() {
        return Err("left side isn't an expression".to_string());
    }
    
    let right = parse_expr(s[1]);
    if !right.is_ok() {
        return Err("right side isn't an expression".to_string());
    }
    
    Ok(Expr::GreaterThan(Box::new(left.unwrap()), Box::new(right.unwrap())))   
}


fn parse_lt(s: &str) -> ParseResult {
    let s: Vec<&str> = s.split("<").collect();
    if s.len() == 1 {
        return Err("not a greaterthan statement".to_string());
    }

    let left = parse_expr(s[0]);
    if !left.is_ok() {
        return Err("left side isn't an expression".to_string());
    }
    
    let right = parse_expr(s[1]);
    if !right.is_ok() {
        return Err("right side isn't an expression".to_string());
    }
    
    Ok(Expr::LowerThan(Box::new(left.unwrap()), Box::new(right.unwrap())))   
}


fn parse_fn_call(s: &str) -> ParseResult {
    if !s.chars().all(|c| c.is_alphanumeric() && c.is_lowercase()) {
        return Err("not an fn function name".to_string());
    }

    Ok(Expr::FnCall(s.to_string()))
}

fn parse_fn(s: &str) -> ParseResult {
    let s: Vec<&str> = s.split("{").collect();
    if s.len() == 1 {
        return Err("not an fn statement, missing a '{'".to_string());
    }

    let fn_name = s[0];
    let s: Vec<&str> = s[1].split("}").collect();
    if s.len() == 1 {
        return Err("not an fn statement, missing a '}'".to_string());
    }

    let fn_body = s[0];
    if fn_name.len() == 0 {
        Ok(Expr::Closure(fn_body.split(",").map(|s| parse_expr(s).unwrap()).collect::<Vec<Expr>>()))
    } else {
        Ok(Expr::Fn(fn_name.to_string(), fn_body.split(",").map(|s| parse_expr(s).unwrap()).collect::<Vec<Expr>>()))
    }
}

fn parse_expr(s: &str) -> ParseResult {
    let functions: Vec<fn(&str) -> ParseResult> = vec![parse_return, parse_literal, parse_variable, parse_assign, parse_if, parse_gt, parse_lt, parse_fn_call, parse_fn];
    for func in functions {
        match func(s) {
            Ok(expr) => return Ok(expr),
            Err(_) => {}
        }
    }
    Err("syntax error".to_string())
}


fn parse(input: &str) -> Vec<Expr> {
    read_to_string(input).unwrap().split("\n").filter(|s| s.len() > 0).map(|s| {
        parse_expr(s).unwrap()
    }).collect()
}


/* ========== interpreter =========== */

struct VM {
    fns: HashMap<String, Vec<Expr>>,
    fvars: HashMap<String,i64>,
    done: bool,
    debug: bool,
    sum: i64
}

type InterpretResult = Result<i64, String>;

impl VM {
    fn eval_return(&mut self, b: bool) -> InterpretResult {
        if self.debug {println!("return: {}",b)}
        self.done = true;
        match b {
            true =>  Ok(0),
            false => Ok(1)
        }
    }

    fn eval_literal(&mut self, l: i64) -> InterpretResult {
        if self.debug {println!("literal: {}",l)}
        Ok(l)    
    }

    fn eval_variable(&mut self, s: String) -> InterpretResult {
        if self.debug {println!("var: {} ({})",s,self.fvars[&s])}
        Ok(self.fvars[&s])
    }

    fn eval_assign(&mut self, var: &Expr, e: &Expr) -> InterpretResult {
        let Expr::Variable(var_name) = var else { panic!("expected variable on left side of assignement") };
        let Ok(l) = self.eval(&e) else { panic!("right side evaluation of the assignement failed") };
        if self.debug {println!("assign: {} <= {}",var_name,l)}
        self.fvars.insert(var_name.clone(),l);
        Ok(l)
    }

    fn eval_if(&mut self, cond: &Expr, then: &Expr) -> InterpretResult {
        if self.debug {println!("if: {:?} <= {:?}",cond,then)}
        match self.eval(cond) {
            Ok(0) => self.eval(then),
            Ok(_) => Ok(1),
            Err(s) => Err("could not evaluate if condition: ".to_string()+&s)
        }
    }

    fn eval_gt(&mut self, left: &Expr, right: &Expr) -> InterpretResult {
        if self.debug {println!("gt: {:?} <= {:?}",left,right)}
        let Ok(l) = self.eval(left) else { panic!("cannot evaluate left side of the operator") };
        let Ok(r) = self.eval(right) else { panic!("cannot evaluate right side of the operator") };
        if l > r {
            Ok(0)
        } else {
            Ok(1)
        }
    }


    fn eval_lt(&mut self, left: &Expr, right: &Expr) -> InterpretResult {
        if self.debug {println!("lt: {:?} <= {:?}",left,right)}
        let Ok(l) = self.eval(left) else { panic!("cannot evaluate left side of the operator") };
        let Ok(r) = self.eval(right) else { panic!("cannot evaluate right side of the operator") };
        if l < r {
            Ok(0)
        } else {
            Ok(1)
        }
    }

    fn eval_fn(&mut self, func_name: String, body: Vec<Expr>) -> InterpretResult {
        self.fns.insert(func_name.clone(), body.clone());
        Ok(0)
    }

    fn eval_fn_call(&mut self, func_name: String) -> InterpretResult {
        if self.debug {println!("calling {}",func_name)};
        let body = self.fns[&func_name].clone();
        let mut last: InterpretResult = Ok(0);
        for e in body.iter() {
            last = self.eval(e);
            if self.done {
                return last
            }
        }
        return last;
    }

    fn eval_closure(&mut self, body: &Vec<Expr>) -> InterpretResult {
        for e in body.iter() {
            self.eval(e);
        }
        
        self.done = false;
        match self.eval_fn_call("in".to_string()) {
            Ok(0) => {
                //println!("x={},m={},a={},s={} -> accepted",self.fvars["x"], self.fvars["m"], self.fvars["a"], self.fvars["s"])
                self.sum += self.fvars["x"] + self.fvars["m"] + self.fvars["a"] + self.fvars["s"];
            },
            Ok(_) => {
                //println!("x={},m={},a={},s={} -> rejected",self.fvars["x"], self.fvars["m"], self.fvars["a"], self.fvars["s"])
            },
            Err(e) => println!("x={},m={},a={},s={} -> error {}",self.fvars["x"], self.fvars["m"], self.fvars["a"], self.fvars["s"],e)
        }
        Ok(0)
    }

    fn eval(&mut self, expr: &Expr) -> InterpretResult {
        let e = expr.clone();
        match e {
            Expr::Return(ret) => self.eval_return(ret),
            Expr::Literal(i) => self.eval_literal(i),
            Expr::Variable(str) => self.eval_variable(str),
            Expr::Assign(l, r) => self.eval_assign(l.as_ref(),r.as_ref()),
            Expr::If(cond,then) => self.eval_if(cond.as_ref(),then.as_ref()),
            Expr::GreaterThan(left,right) => self.eval_gt(left.as_ref(), right.as_ref()),
            Expr::LowerThan(left,right) => self.eval_lt(left.as_ref(), right.as_ref()),
            Expr::FnCall(func_name) => self.eval_fn_call(func_name),
            Expr::Fn(func_name, body) => self.eval_fn(func_name, body),
            Expr::Closure(body) => self.eval_closure(&body),
        }
    }
}

fn main() {
    let input = "day19/assets/input";    
    let instructions = parse(input);

    let mut vm = VM {
        fns: HashMap::new(),
        fvars: HashMap::new(),
        done: false,
        debug: false,
        sum: 0
    };
    instructions.iter().for_each(|e| {
        match vm.eval(e) {
            Err(e) => panic!("there was an error evaluating: {}",e),
            _ => {}
        }
    });
    println!("sum: {}",vm.sum);
}