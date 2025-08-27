use std::fs;

use shields::BadgeStyle;
use shields::builder::Badge;
use simpleicons_rs::{
    SIC, SICMAKE, SICPLUSPLUS, SICSS, SIDART, SIFLUTTER, SIGIT, SIGITHUB, SIGITLAB, SIHTML5,
    SIJAVASCRIPT, SILUA, SINUSHELL, SIPYTHON, SIRUST, SISASS, SITYPESCRIPT,
};

fn invert_color(input: &str) -> String {
    let mut input = String::from(input);
    if input.starts_with("#") {
        input = input.strip_prefix('#').unwrap().to_string();
    }
    if input.len() == 3 {
        let mut tmp = String::new();
        for ch in input.chars() {
            tmp.push(ch);
            tmp.push(ch);
        }
        input = tmp;
    }
    if input.len() != 6 {
        panic!("color is not exactly 6 hexadecimal characters");
    }

    let (r, g, b) = (
        u8::from_str_radix(&input[0..2], 16).unwrap(),
        u8::from_str_radix(&input[2..4], 16).unwrap(),
        u8::from_str_radix(&input[4..6], 16).unwrap(),
    );
    if (((r as f32) * 0.299) + ((g as f32) * 0.587) + ((b as f32) * 0.114)) > 150.0 {
        return String::from("000000");
    }
    String::from("FFFFFF")
}

/// the executable binary's main entry point.
fn main() {
    if fs::exists("assets").is_ok_and(|v| !v) {
        fs::create_dir("assets").unwrap();
    }
    let brands = [
        SIC,
        SICPLUSPLUS,
        SICSS,
        SIDART,
        SINUSHELL,
        SICMAKE,
        SIFLUTTER,
        SIHTML5,
        SIGIT,
        SIJAVASCRIPT,
        SILUA,
        SIPYTHON,
        SIRUST,
        SISASS,
        SITYPESCRIPT,
    ];
    for brand in brands {
        let inverted_color = invert_color(brand.hex);
        let logo = brand.svg.replace(
            " xmlns=",
            format!(" fill=\"#{inverted_color}\" xmlns=").as_str(),
        );
        let svg = Badge::style(BadgeStyle::Plastic)
            .message(brand.title)
            .label_color(brand.hex)
            .message_color("grey")
            .logo(&logo)
            .build();
        fs::write(format!("assets/{}.svg", brand.slug), svg).unwrap();
        println!("Generated badge for {}", brand.slug);
    }

    let message = "Bash";
    let msg_lower = message.to_lowercase();
    let svg = Badge::style(BadgeStyle::Plastic)
        .message_color("grey")
        .message(message)
        .label("$")
        .label_color("232323")
        .build();
    fs::write(format!("assets/{}.svg", msg_lower), svg).unwrap();
    println!("Generated badge for {}", msg_lower);

    let (message, icon) = ("GitHub Actions", SIGITHUB);
    let slug = message.to_lowercase();
    let logo = icon.svg.replace(
        " xmlns=",
        format!(" fill=\"#{}\" xmlns=", invert_color(icon.hex)).as_str(),
    );
    let svg = Badge::style(BadgeStyle::Plastic)
        .message_color("grey")
        .message(message)
        .label_color(icon.hex)
        .logo(&logo)
        .build();
    fs::write(format!("assets/{}.svg", slug), svg).unwrap();
    println!("Generated badge for {}", slug);

    let (message, icon) = ("GitLab Pipelines", SIGITLAB);
    let slug = message.to_lowercase();
    let logo = icon.svg.replace(
        " xmlns=",
        format!(" fill=\"#{}\" xmlns=", invert_color(icon.hex)).as_str(),
    );
    let svg = Badge::style(BadgeStyle::Plastic)
        .message_color("grey")
        .message(message)
        .label_color(icon.hex)
        .logo(&logo)
        .build();
    fs::write(format!("assets/{}.svg", slug), svg).unwrap();
    println!("Generated badge for {}", slug);
}
