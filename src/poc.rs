#![allow(unused)]

use colored::*;
use regex::Regex;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub bold: bool,
}

pub struct Theme {
    pub blue_smurf: Color,
    pub blue_coffee_bold: Color,
    pub blue_smurf_bold: Color,
    pub cream: Color,
    pub cream_bold: Color,
    pub charcoal_cream: Color,
    pub green_eggs: Color,
    pub green_eggs_bold: Color,
    pub grey_mouse: Color,
    pub orange_sherbet: Color,
    pub red: Color,
    pub red_bold: Color,
    pub yellow_canary_bold: Color,
    pub white: Color,
    pub white_bold: Color,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            blue_smurf: Color {
                r: 0,
                g: 128,
                b: 255,
                bold: false,
            },
            blue_coffee_bold: Color {
                r: 0,
                g: 192,
                b: 255,
                bold: true,
            },
            blue_smurf_bold: Color {
                r: 0,
                g: 128,
                b: 255,
                bold: true,
            },
            cream: Color {
                r: 250,
                g: 246,
                b: 228,
                bold: false,
            },
            cream_bold: Color {
                r: 250,
                g: 246,
                b: 228,
                bold: true,
            },
            charcoal_cream: Color {
                r: 102,
                g: 102,
                b: 102,
                bold: false,
            },
            green_eggs: Color {
                r: 135,
                g: 255,
                b: 175,
                bold: false,
            },
            green_eggs_bold: Color {
                r: 135,
                g: 255,
                b: 175,
                bold: true,
            },
            grey_mouse: Color {
                r: 115,
                g: 115,
                b: 115,
                bold: false,
            },
            orange_sherbet: Color {
                r: 239,
                g: 157,
                b: 110,
                bold: false,
            },
            red: Color {
                r: 241,
                g: 95,
                b: 73,
                bold: false,
            },
            red_bold: Color {
                r: 241,
                g: 95,
                b: 73,
                bold: true,
            },
            yellow_canary_bold: Color {
                r: 255,
                g: 252,
                b: 103,
                bold: true,
            },
            white: Color {
                r: 255,
                g: 255,
                b: 255,
                bold: false,
            },
            white_bold: Color {
                r: 255,
                g: 255,
                b: 255,
                bold: true,
            },
        }
    }

    pub fn color_rgb(&self, message: &str, color: &Color) -> ColoredString {
        if !color.bold {
            message.truecolor(color.r, color.g, color.b)
        }
        else {
            message.truecolor(color.r, color.g, color.b).bold()
        }
    }

    pub fn color_blank(&self, _message: &str) -> ColoredString {
        "".truecolor(0, 0, 0)
    }

}

pub fn highlight(output_str: &str, highlight_term: &str, color: &Color) -> String {
    /* find the highlight term in the output string and format the output
        * string to emphasize the highlight term in the output string
        */

    let tmp: String = output_str.to_string();
    let elements: Vec<&str> = tmp.split(&highlight_term).collect::<Vec<&str>>();

    // construct highlighted output
    let mut o: String = String::new();
    let theme = Theme::new();
    for i in 0..elements.len() {
        if i < (elements.len() - 1) {
            o += &format!(
                "{}{}",
                theme.color_rgb(elements[i], &theme.grey_mouse),
                theme.color_rgb(highlight_term, color),
            );
        }
        else {
            o += &format!(
                "{}",
                theme.color_rgb(elements[i], &theme.grey_mouse),
            );
        }
    }

    o
}

pub fn highlight_filename(output_str: &str, color: &Color) -> String {
    /* highlight everything following the last "/" */

    let re: Regex = Regex::new(r"/([^/]+)$").unwrap();

    let filename: String = match re.captures(output_str) {
        Some(n) => n[1].to_string(),
        None => "".to_string(),
    };

    highlight(output_str, &filename, color)
}