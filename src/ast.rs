use std::ops::Deref;
use std::fmt;

#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Ident(String);

impl Ident {
    pub fn new<S: Into<String>>(ident: S) -> Self {
        let ident = ident.into();
        assert!(!ident.is_empty());
        assert!(ident.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'));
        assert!(ident.chars().next().unwrap().is_ascii_alphabetic());

        Ident(ident)
    }
}

impl Deref for Ident {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug,PartialEq)]
pub struct Fn {
    pub ident: Ident,
    pub args: Vec<FnArg>,
    pub body: Vec<Stmt>,
}

#[derive(Debug,PartialEq)]
pub struct FnArg {
    pub ident: Ident,
    pub typ: Type,
}

#[derive(Debug,PartialEq)]
pub struct Call {
    pub ident: Ident,
    pub args: Vec<Expr>,
}

#[derive(Debug,PartialEq)]
pub struct Var {
    pub ident: Ident,
    pub mask: Option<Vec<Component>>,
}

#[derive(Debug,PartialEq)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Float(f32),
    Call(Call),
    Array(Vec<Expr>),
    Var(Var),
}

#[derive(Debug,PartialEq)]
pub enum Stmt {
    Let { ident: Ident, typ: Option<Type>, expr: Expr },
    Return { expr: Expr },
    Assign { ident: Ident, expr: Expr },
    AssignAdd { ident: Ident, expr: Expr },
    AssignSub { ident: Ident, expr: Expr },
    AssignMul { ident: Ident, expr: Expr },
    AssignDiv { ident: Ident, expr: Expr },
    AssignMod { ident: Ident, expr: Expr },
    Expr(Expr),
}

#[derive(Debug,PartialEq)]
pub struct Output {
    pub ident: Ident,
    pub typ: Ident,
}

#[derive(Debug,PartialEq)]
pub struct Uniform {
    pub ident: Ident,
    pub len: usize,
}

#[derive(Debug,PartialEq)]
pub struct Input {
    pub ident: Ident,
}

#[derive(Debug,PartialEq,Default)]
pub struct Module {
    pub uniforms: Vec<Uniform>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub functions: Vec<Fn>,
}

#[derive(Debug,PartialEq)]
pub enum Item {
    Fn(Fn),
    Uniforms(Vec<Uniform>),
    Inputs(Vec<Ident>),
    Outputs(Vec<Output>),
}

impl Item {
    pub fn declared_idents<'a>(&'a self) -> Box<Iterator<Item=&'a Ident> + 'a> {
        use std::iter::once;

        match *self {
            Item::Fn(ref fun) => Box::new(once(&fun.ident)),
            Item::Uniforms(ref uniforms) => Box::new(uniforms.iter().map(|uniform| &uniform.ident)),
            Item::Inputs(ref inputs) => Box::new(inputs.iter()),
            Item::Outputs(ref outputs) => Box::new(outputs.iter().map(|output| &output.ident)),
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum Component {
    R,
    G,
    B,
    A
}

impl Component {
    pub fn from_char(ch: char) -> Option<Self> {
        Some(match ch {
            'r' | 'x' => Component::R,
            'g' | 'y' => Component::G,
            'b' | 'z' => Component::B,
            'a' | 'w' => Component::A,
            _ => return None,
        })
    }
}

#[derive(PartialEq)]
pub enum Type {
    Ident(Ident),
    Array { typ: Box<Type>, len: usize },
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Ident(ref ident) => fmt::Display::fmt(ident, f),
            Type::Array { ref typ, len } => write!(f, "[{:?}; {}]", typ, len),
        }
    }
}
