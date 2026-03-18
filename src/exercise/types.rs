use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ─── Exercise Types ─────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseType {
    FixCompilerError,
    DebugLogicBug,
    ImplementFromScratch,
    CodeTransformation,
    BossBattle,
    ReverseEngineeringFillIn,
    ReverseEngineeringMultipleChoice,
    Optimization,
}

impl ExerciseType {
    /// Derive the verification method from the exercise type.
    /// This is the single source of truth — no separate field in info.toml.
    pub fn verification_method(&self) -> VerificationMethod {
        match self {
            Self::FixCompilerError => VerificationMethod::CompileOnly,
            Self::DebugLogicBug
            | Self::ImplementFromScratch
            | Self::BossBattle
            | Self::ReverseEngineeringFillIn => VerificationMethod::CompileAndTest,
            Self::CodeTransformation => VerificationMethod::CompileTestClippy,
            Self::Optimization => VerificationMethod::CompileTestCustom,
            Self::ReverseEngineeringMultipleChoice => VerificationMethod::MultipleChoice,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationMethod {
    CompileOnly,
    CompileAndTest,
    CompileTestClippy,
    CompileTestCustom,
    MultipleChoice,
}

// ─── Tier / Difficulty / Status ─────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tier {
    Foundation,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseStatus {
    Locked,
    Available,
    InProgress,
    Completed,
}

impl Default for ExerciseStatus {
    fn default() -> Self {
        Self::Locked
    }
}

// ─── Custom Checks ──────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomCheck {
    pub check_type: CustomCheckType,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomCheckType {
    NoClone,
    NoUnwrap,
    NoCollect,
    NoBoxDyn,
    MaxLines,
}

// ─── Multiple Choice ────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCOption {
    pub label: String,
    pub text: String,
    pub correct: bool,
}

// ─── Exercise ───────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Exercise {
    pub id: String,
    pub name: String,
    pub module_id: String,
    pub exercise_type: ExerciseType,
    pub difficulty: Difficulty,
    pub base_xp: u32,
    pub file_path: PathBuf,
    pub solution_path: PathBuf,
    pub hints: Vec<String>,
    pub description: String,
    pub flavor_text: Option<String>,
    pub time_limit_secs: Option<u32>,
    pub custom_checks: Vec<CustomCheck>,
    pub multiple_choice_options: Vec<MCOption>,
    pub extra_files: Vec<PathBuf>,
    pub order: u32,
    pub ci: bool,
}

impl Exercise {
    pub fn verification_method(&self) -> VerificationMethod {
        self.exercise_type.verification_method()
    }

    pub fn is_time_trial(&self) -> bool {
        self.time_limit_secs.is_some()
    }
}

// ─── Module ─────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub theme_name: String,
    pub flavor_text: String,
    pub tier: Tier,
    pub order: u32,
    pub prerequisites: Vec<String>,
    // exercises list is computed by the catalog, not stored here
}
