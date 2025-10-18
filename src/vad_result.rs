#[derive(Debug, PartialEq)]
pub enum VadStatus {
    Speech,
    Silence,
    Unknown,
}
pub struct VadResult {
    pub prob: f32,
}

impl VadResult {
    pub fn status(&mut self) -> VadStatus {
        if self.prob > 0.5 {
            return VadStatus::Speech;
        }
        if self.prob < 0.35 {
            return VadStatus::Silence;
        }
        VadStatus::Unknown
    }
}
