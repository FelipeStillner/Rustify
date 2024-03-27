pub enum PlayInput {
    SETVOLUME(f32),
    NEXT,
    PAUSEPLAY,
    HELP,
    QUIT,
    INVALID(String),
}

pub enum LoginInput {
    QUIT,
    GUEST,
    NEW(String),
    OTHER(String),
}

pub enum HomeInput {
    QUIT,
    PLAYMUSIC(String),
    PLAYPLAYLIST(String),
    INVALID,
}

pub fn manipulate_play_input(input: String) -> PlayInput {
    let input: Vec<&str> = input.split_ascii_whitespace().collect();
    match input[0].trim().to_ascii_lowercase().as_str() {
        "p" => PlayInput::PAUSEPLAY,
        "n" => PlayInput::NEXT,
        "v" => {
            let value = input[1].parse::<f32>();
            match value {
                Ok(v) => PlayInput::SETVOLUME(v),
                Err(_) => PlayInput::INVALID("Invalid Value for volume".to_string()),
            }
        }
        "h" => PlayInput::HELP,
        "q" => PlayInput::QUIT,
        _ => PlayInput::INVALID("Invalid Input".to_string()),
    }
}

pub fn manipulate_login_input(input: String) -> LoginInput {
    let input: Vec<&str> = input.split_ascii_whitespace().collect();
    match input[0].trim().to_ascii_lowercase().as_str() {
        "n" => LoginInput::NEW(input[1].trim().to_ascii_lowercase()),
        "g" => LoginInput::GUEST,
        "q" => LoginInput::QUIT,
        input => LoginInput::OTHER(input.to_string()),
    }
}

pub fn manipulate_home_input(input: String) -> HomeInput {
    let input: Vec<&str> = input.split_ascii_whitespace().collect();
    match input[0].trim().to_ascii_lowercase().as_str() {
        "pp" => {
            if input.len() > 1 {
                HomeInput::PLAYPLAYLIST(input[1].trim().to_ascii_lowercase())
            } else {
                HomeInput::INVALID
            }
        }
        "pm" => {
            if input.len() > 1 {
                HomeInput::PLAYMUSIC(input[1].trim().to_ascii_lowercase())
            } else {
                HomeInput::INVALID
            }
        }
        "q" => HomeInput::QUIT,
        _ => HomeInput::INVALID,
    }
}
