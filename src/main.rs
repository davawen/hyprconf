peg::parser!{
  grammar hyprconf() for str {
    rule ws() = " " / "\t"
    rule _() = ws()*
    rule __() = ws()+

    rule slice_string<T>(x: rule<T>) -> String
        = s:$(x()) { s.into() }

    rule to_string<T: Into<String>>(x: rule<T>) -> String
        = s:x() { s.into() }

    rule hex() -> char
        = ['0'..='9' | 'a'..='f' | 'A'..='F']

    rule int() -> String
      = slice_string(<['0'..='9']+>)

    rule boolean() -> String
        = slice_string(<"true" / "false">)

    rule float() -> String
        = slice_string(<['0'..='9']+ "." ['0'..='9']+>)

    rule color() -> String
        = "#" color:(
            rgba:$(hex()*<8>) { format!("rgba({rgba})") } /
            rgb:$(hex()*<6>) { format!("rgb({rgb})") }
        ) { color }

    rule vec2() -> String
        = "[" _ a:float() _ "," _ b:float() _ "]" { format!("{a} {b}") }

    rule hyprstr() -> String
        = slice_string(<"\"" (!"\"" [_])* "\"">)
        / slice_string(<['a'..='z' | 'A'..='Z' | '_']+>)

    // #ff0000..#00ff00..#0000ff(10deg)

    rule gradient() -> String
        = colors:((_ c:color() _ { c }) **<2,> "..") _ "(" _ a:int() _ "deg" _ ")" { format!("{} {a}deg", colors.join(" ")) }

    rule value() -> String
        = gradient() / color() / vec2() / float() / int() / boolean() / hyprstr()

    rule empty() -> String
        = "_" { String::new() }

    rule ident() -> String
        = slice_string(<['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '.' | ':']+>)

    rule set_variable() -> String
        = i:ident() _ "=" _ v:value() { format!("{i} = {v}") }

    rule keyword(separator: char) -> String
        = k:ident() __ params:((_ v:(empty() / value()) _ { v }) ** ",") {
            format!("{k}{separator}{}", params.join(","))
        }

    // exec swaylock
    // exec = swaylock

    // CTRL+SHIFT+Q

    rule modifier() -> &'input str
        = $(['A'..='Z']+)

    pub rule bind() -> String
        = "bind" flags:$(['l' | 'r' | 'e' | 'n' | 'm' | 't']*) __
            modifiers:(m:modifier() _ "+" _ { m })* key:ident() __
            actions:((_ k:keyword(',') _ { k }) ++ ";") { 
            actions.into_iter().map(|action|
                format!("bind{flags} = {},{key},{action}", modifiers.join("+"))
            ).collect::<Vec<_>>().join("\n")
        }

    // rule number() -> u32
    //
    // pub rule list() -> Vec<u32>
    //   = "[" l:(number() ** ",") "]" { l }
  }
}

fn main() {
    let p = hyprconf::bind("bind SUPER+mouse_down workspace e-1");

    match p {
        Ok(x) => println!("{x}"),
        Err(e) => println!("{e}")
    }
}
