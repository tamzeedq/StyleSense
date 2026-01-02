// Basic style rules for C/C++ code
pub struct StyleRule {
    pub name: &'static str,
    pub description: &'static str,
}

pub const SPACING_RULES: [StyleRule; 2] = [
    StyleRule {
        name: "space_before_equals",
        description: "Require space before equals sign",
    },
    StyleRule {
        name: "space_after_equals",
        description: "Require space after equals sign",
    },
];