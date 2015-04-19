
/// Utility functions to output css strings.
pub struct CssFormatter;

impl CssFormatter {
    /// Output the value in pixels.
    pub fn px(x:i32) -> String {
        format!("{}{}", x, if x!=0 {"px"} else {""})
    }

    /// Output the position.
    pub fn position(x:i32 , y:i32) -> String {
        format!("{} {}", CssFormatter::px(x), CssFormatter::px(y) )
    }

}
