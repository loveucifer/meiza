use mieza::{parse_and_render, SvgStyle, SvgTheme};

fn main() {
    println!("Hello, meiza!");

    let cdl = r#"
          R1 resistor 1k
          R1.2 -> R2.1
      "#;

    let svg = parse_and_render(cdl, SvgTheme::Light, SvgStyle::Ieee);
    match svg {
        Ok(svg_content) => println!("{}", svg_content),
        Err(e) => eprintln!("Error: {}", e),
    }
}
