
pub enum EnviromentType {
    Function(String, fn(&[i32]) -> i32),
    Data,
}