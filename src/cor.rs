use colored::*;

pub fn color_rgb(message: &str, r: u8, g: u8, b: u8) -> colored::ColoredString {
    message.truecolor(r, g, b)
}

pub fn color_rgb_bold(message: &str, r: u8, g: u8, b: u8) -> colored::ColoredString {
    message.truecolor(r, g, b).bold()
}

pub fn color_red_bold(message: &str) -> ColoredString {
    message.truecolor(241, 95, 78).bold()
}

pub fn _color_orange_sherbet_bold(message: &str) -> ColoredString {
    message.truecolor(239, 157, 110).bold()
}

pub fn color_yellow_canary_bold(message: &str) -> ColoredString {
    message.truecolor(255, 252, 103).bold()
}

pub fn color_green_eggs_bold(message: &str) -> ColoredString {
    message.truecolor(135, 255, 175).bold()
}

pub fn color_blue_smurf(message: &str) -> ColoredString {
    message.truecolor(0, 128, 255)
}

pub fn color_blue_smurf_bold(message: &str) -> ColoredString {
    message.truecolor(0, 128, 255).bold()
}

pub fn color_blue_coffee_bold(message: &str) -> ColoredString {
    message.truecolor(0, 192, 255).bold()
}

pub fn color_white_bold(message: &str) -> ColoredString {
    message.truecolor(249, 247, 236).bold()
}

pub fn color_white(message: &str) -> ColoredString {
    message.truecolor(249, 247, 236)
}

pub fn color_grey_mouse(message: &str) -> ColoredString {
    message.truecolor(155, 155, 155)
}

pub fn color_charcoal_cream(message: &str) -> ColoredString {
    message.truecolor(102, 102, 102)
}

pub fn color_blank(_message: &str) -> ColoredString {
    "".truecolor(0, 0, 0)
}
