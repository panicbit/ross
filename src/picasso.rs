
pub enum ConstExpr {
    Float(f32),
    FVec(Vec<f32>)
}

pub enum Register {
    Input,
    Scratch,
    FloatConst,
}

struct Ref {
    register: Register,
}

struct Builder {

}
