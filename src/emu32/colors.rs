

pub struct Colors {
    pub black:String,
    pub red:String,
    pub green:String,
    pub orange:String,
    pub blue:String,
    pub purple:String,
    pub cyan:String,
    pub light_gray:String,
    pub dark_gray:String,
    pub light_red:String,
    pub light_green:String,
    pub yellow:String,
    pub light_blue:String,
    pub light_purple:String,
    pub light_cyan:String,
    pub white:String,
    pub nc:String, // no_color
}

impl Colors {
    pub fn new() -> Colors {
        Colors{
            black:"\x1b[0;30m".to_string(),
            red:"\x1b[0;31m".to_string(),
            green:"\x1b[0;32m".to_string(),
            orange:"\x1b[0;33m".to_string(),
            blue:"\x1b[0;34m".to_string(),
            purple:"\x1b[0;35m".to_string(),
            cyan:"\x1b[0;36m".to_string(),
            light_gray:"\x1b[0;37m".to_string(),
            dark_gray:"\x1b[1;30m".to_string(),
            light_red:"\x1b[1;31m".to_string(),
            light_green:"\x1b[1;32m".to_string(),
            yellow:"\x1b[1;33m".to_string(),
            light_blue:"\x1b[1;34m".to_string(),
            light_purple:"\x1b[1;35m".to_string(),
            light_cyan:"\x1b[1;36m".to_string(),
            white:"\x1b[1;37m".to_string(),
            nc:"\x1b[0m".to_string(), // no_color
        }
    }
}