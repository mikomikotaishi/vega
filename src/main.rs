use crate::{
    _utils::cli,
    data::common::get_system_info,
    logo::logo::{Logo, get_logo},
};

mod _utils;
mod data;
mod logo;

fn main() {
    if cli::handle_clap() {
        return;
    }
    // Default: print logo and system info
    let mut logo: Logo = get_logo();
    let system_info = get_system_info();
    system_info.for_each(|info: String| {
        let content: Option<&'static str> = logo.content.next();
        if let Some(content) = content {
            print!("{}", content);
        } else {
            print!("{}", " ".repeat(logo.cols as usize));
        }
        println!("   {}", info)
    });
    // Finish logo
    loop {
        let content: Option<&'static str> = logo.content.next();
        if let Some(content) = content {
            print!("{}", content);
        } else {
            break;
        }
    }
    println!();
    cli::print_colors();
    println!();
}
