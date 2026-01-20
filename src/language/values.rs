#[derive(Debug, Clone)]
pub enum Value {
	Int(i32),
	Float(f32),
	Bool(bool),
	String(String),
}