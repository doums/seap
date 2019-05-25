pub mod parser {

    #[derive(Debug)]
    pub struct Arg<T>(&'static str, &'static str, T)
        where T: Fn(&mut Config, &mut std::env::Args);

    #[derive(Debug)]
    pub enum Mod {
        Normal,
        Word,
        Regex
    }

    #[derive(Debug)]
    pub struct Config {
        pub query: Option<String>,
        pub replace: Option<String>,
        pub search_mod: Mod,
        pub case_sensitive: bool,
        pub confirm: bool,
        pub excluded: Option<Vec<String>>,
        pub file_mask: Option<Vec<String>>,
        pub root: Option<String>,
        pub recursive: bool
    }

    #[derive(Debug)]
    pub enum TokenType {
        IgnoreCase,
        WordMod,
        RegexMod,
        Confirm,
        Excluded,
        FileMask,
        File,
        Recursive,
        Arg
    }

    impl Copy for TokenType { }
    impl Clone for TokenType {
        fn clone(&self) -> TokenType {
            *self
        }
    }

    pub struct OptData(TokenType, &'static str, &'static str, bool);
    const OPTIONS: [OptData; 8] = [
        OptData(TokenType::IgnoreCase, "i", "ignore-case", false),
        OptData(TokenType::WordMod, "w", "word", false),
        OptData(TokenType::RegexMod, "x", "regex", false),
        OptData(TokenType::Confirm, "c", "confirm", false),
        OptData(TokenType::Excluded, "e", "excluded", true),
        OptData(TokenType::FileMask, "m", "mask", true),
        OptData(TokenType::File, "f", "file", true),
        OptData(TokenType::Recursive, "r", "recursive", false)
    ];

    #[derive(Debug)]
    pub struct Token(TokenType, Option<String>);

    fn find_option(short: Option<&str>, long: Option<&str>, accept_arg: bool) -> Result<TokenType, &'static str> {
        for option in &OPTIONS {
            if let Some(i) = short {
                if i == option.1 && accept_arg == option.3 {
                    return Ok(option.0)
                }
            }
            if let Some(i) = long {
                if i == option.2 && accept_arg == option.3 {
                    return Ok(option.0)
                }
            }
        }
        Err("unknown option")
    }

    fn parse_long_opt(arg: &str, tokens: &mut Vec<Token>) -> Result<(), &'static str> {
        let mut current_arg = String::from(arg);
        current_arg.drain(..2);
        match current_arg.find("=") {
            None => {
                let token_type = find_option(None, Some(&current_arg), false)?;
                tokens.push(Token(token_type, None));
            },
            Some(i) => {
                current_arg.remove(i);
                let (first, last) = current_arg.split_at(i);
                println!("first {}, last {}", first, last);
                if first.len() == 0 {
                    return Err("bad synthax for option")
                }
                let token_type = find_option(None, Some(first), true)?;
                tokens.push(Token(token_type, Some(String::from(last))))
            }
        }
        Ok(())
    }

    fn tokenize(args: &mut std::env::Args, config: &mut Config) -> Result<(), &'static str> {
        let mut tokens: Vec<Token> = vec![];
        let mut accept_opt = true;
        args.next(); //skip the binary
        while let Some(arg) = args.next() {
            if arg.len() == 2 && &arg[..2] == "--" {
                println!("terminate arg");
                accept_opt = false;
            } else if arg.len() > 2 && &arg[..2] == "--" && accept_opt == true {
                println!("long option");
                parse_long_opt(&arg, &mut tokens)?;
            } else if arg.len() > 1 && &arg[..1] == "-" && accept_opt == true {
                println!("option");
                // option_handler(&arg, &mut config, &mut args)?;
            } else {
                println!("arg");
                tokens.push(Token(TokenType::Arg, Some(String::from(arg))));
                // arg_handler(&arg, &mut config, &mut args)?;
            }
        }
        dbg!(tokens);
        Ok(())
    }

    impl Config {
        pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
            if args.len() < 1 {
                return Err("not enough arguments");
            }
            let mut config = Config {
                query: None,
                replace: None,
                search_mod: Mod::Normal,
                case_sensitive: false,
                confirm: true,
                excluded: None,
                file_mask: None,
                root: None,
                recursive: false
            };
            tokenize(&mut args, &mut config)?;
            Ok(config)
        }
    }
}


// const ARGS: [Arg<fn(&mut Config, &mut std::env::Args)>; 2] = [
// Arg("i", "ignore-case", |config, _args| {
// config.case_sensitive = true;
// }),
// Arg("c", "confirm", |config, _args| {
// config.confirm = true;
// })
// ];
//
// fn arg_handler(arg: &String, config: &mut Config, args: &mut env::Args) -> Result<(), &'static str> {
// if config.query == None {
// config.query = Some(arg.to_string());
// return Ok(())
// }
// if config.replace == None {
// config.replace = Some(arg.to_string());
// return Ok(())
// }
// Err("bad usage")
// }
//
// fn parse_long_opt(arg: &String, config: &mut Config, args: &mut env::Args) -> Result<(), &'static str> {
// return match ARGS.iter().find(|&valid| valid.1 == arg) {
// None => Err("option not found"),
// Some(elem) => {
// elem.2(config, args);
// Ok(())
// }
// }
// }
//
// fn option_handler(arg: &String, config: &mut Config, args: &mut env::Args) -> Result<(), &'static str> {
// let vec_args: Vec<String> = env::args().skip(1).collect();
// println!("{:#?}", vec_args);
// if &arg[..2] == "--" {
// parse_long_opt(&arg, config, args)?;
// return Ok(())
// } else {
//
// }
// Ok(())
// }
//
// impl Config {
// pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
// args.next(); //skip the binary
// if args.len() < 1 {
// return Err("not enough arguments");
// }
// let mut config = Config {
// query: None,
// replace: None,
// search_mod: Mod::Normal,
// case_sensitive: false,
// confirm: true,
// excluded: None,
// file_mask: None,
// root: None,
// recursive: false
// };
// let mut accept_opt = true;
// while let Some(arg) = args.next() {
// // println!("{:#?}", ARGS[0].2(&mut config, &mut args));
// if arg.len() == 2 && &arg[..2] == "--" {
// println!("terminate arg");
// accept_opt = false;
// } else if &arg[..1] == "-" && accept_opt == true {
// println!("option");
// option_handler(&arg, &mut config, &mut args)?;
// } else {
// println!("arg");
// arg_handler(&arg, &mut config, &mut args)?;
// }
// }
// Ok(config)
// }
//
// }
