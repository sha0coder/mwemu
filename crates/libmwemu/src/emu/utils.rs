#[macro_export]
macro_rules! color {
    ("Black") => {
        "\x1b[0;30m"
    };
    ("Red") => {
        "\x1b[0;31m"
    };
    ("Green") => {
        "\x1b[0;32m"
    };
    ("Orange") => {
        "\x1b[0;33m"
    };
    ("Blue") => {
        "\x1b[0;34m"
    };
    ("Purple") => {
        "\x1b[0;35m"
    };
    ("Cyan") => {
        "\x1b[0;36m"
    };
    ("LightGray") => {
        "\x1b[0;37m"
    };
    ("DarkGray") => {
        "\x1b[1;30m"
    };
    ("LightRed") => {
        "\x1b[1;31m"
    };
    ("LightGreen") => {
        "\x1b[1;32m"
    };
    ("Yellow") => {
        "\x1b[1;33m"
    };
    ("LightBlue") => {
        "\x1b[1;34m"
    };
    ("LightPurple") => {
        "\x1b[1;35m"
    };
    ("LightCyan") => {
        "\x1b[1;36m"
    };
    ("White") => {
        "\x1b[1;37m"
    };
    ("nc") => {
        "\x1b[0m"
    };
    ("ClearScreen") => {
        "\x1bc"
    };
    ($unknown:tt) => {
        compile_error!(concat!(
            "Unknown color name: '",
            $unknown,
            "'. Valid options are: \
            Black, Red, Green, Orange, Blue, Purple, Cyan, LightGray, \
            DarkGray, LightRed, LightGreen, Yellow, LightBlue, \
            LightPurple, LightCyan, White, nc, ClearScreen"
        ))
    };
}
