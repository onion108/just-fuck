use std::fmt::format;

fn get_uint_inside_10(number: u8) -> String {
    if number == 0 {
        String::from("+[]")
    } else {
        let mut result = String::new();
        for _i in 0..number {
            result.push_str("+!+[]");
        }
        return result;
    }
}

fn get_other_uint(number: u32) -> String {
    if number < 10 {
        return get_uint_inside_10(number as u8);
    }
    let digited_numbers = number.to_string().bytes().map(|x| (x - 0x30)).collect::<Vec<u8>>();
    let mut result = String::from("+[");

    for i in digited_numbers {
        result.push_str("[");
        result.push_str(&get_uint_inside_10(i));
        result.push_str("]+");
    }
    result.push_str("[]]");

    result
}

pub fn get_uint(number: u32) -> String {
    get_other_uint(number)
}

fn get_undefined() -> String {
    format!("[][{}]", get_uint(0))
}

fn get_nan() -> String {
    String::from("+[][[]]")
}

fn get_true() -> String {
    String::from("!![]")
}

fn get_false() -> String {
    String::from("![]")
}

fn convert_to_string(expr: &str) -> String {
    format!("({0})+[]", expr)
}

fn get_small_a() -> String {
    format!("({})[{}]", convert_to_string(&get_false()), get_uint(1))
}

fn get_small_f() -> String {
    format!("({})[{}]", convert_to_string(&get_false()), get_uint(0))
}

fn get_small_l() -> String {
    format!("({})[{}]", convert_to_string(&get_false()), get_uint(2))
}

fn get_small_s() -> String {
    format!("({})[{}]", convert_to_string(&get_false()), get_uint(3))
}

fn get_small_e() -> String {
    format!("({})[{}]", convert_to_string(&get_false()), get_uint(4))
}

fn get_small_u() -> String {
    format!("({})[{}]", convert_to_string(&get_undefined()), get_uint(0))
}

fn get_small_n() -> String {
    format!("({})[{}]", convert_to_string(&get_undefined()), get_uint(1))
}

fn get_small_d() -> String {
    format!("({})[{}]", convert_to_string(&get_undefined()), get_uint(2))
}

fn get_small_i() -> String {
    format!("({})[{}]", convert_to_string(&get_undefined()), get_uint(5))
}

fn get_small_r() -> String {
    format!("({})[{}]", convert_to_string(&get_true()), get_uint(1))
}

fn get_small_t() -> String {
    format!("({})[{}]", convert_to_string(&get_true()), get_uint(0))
}

pub fn get_native_code_of_function_find() -> String {
    format!("[][({})+({})+({})+({})]+[]", get_small_f(), get_small_i(), get_small_n(), get_small_d())
}

fn get_small_c() -> String {
    format!("({})[{}]", get_native_code_of_function_find(), get_uint(3))
}

fn get_small_o() -> String {
    format!("({})[{}]", get_native_code_of_function_find(), get_uint(6))
}

fn get_space() -> String {
    format!("({})[{}]", get_native_code_of_function_find(), get_uint(8))
}

fn get_left_par() -> String {
    format!("({})[{}]", get_native_code_of_function_find(), get_uint(13))
}

fn get_right_par() -> String {
    format!("({})[{}]", get_native_code_of_function_find(), get_uint(14))
}

fn get_left_brace() -> String {
    format!("({})[{}]", get_native_code_of_function_find(), get_uint(16))
}

fn get_left_bracket() -> String {
    format!("({})[{}]", get_native_code_of_function_find(), get_uint(18))
}

fn get_small_v() -> String {
    format!("({})[{}]", get_native_code_of_function_find(), get_uint(23))
}

fn get_right_bracket() -> String {
    format!("({})[{}]", get_native_code_of_function_find(), get_uint(30))
}

fn get_right_brace() -> String {
    format!("({})[{}]", get_native_code_of_function_find(), get_uint(32))
}

fn get_empty_string() -> String {
    format!("[]+[]")
}
fn get_string_constructor() -> String {
    format!(
        "({})[{}]",
        get_empty_string(),
        format!(
            "({})+({})+({})+({})+({})+({})+({})+({})+({})+({})+({})",
            get_small_c(),
            get_small_o(),
            get_small_n(),
            get_small_s(),
            get_small_t(),
            get_small_r(),
            get_small_u(),
            get_small_c(),
            get_small_t(),
            get_small_o(),
            get_small_r(),
        )
    )
}

fn get_big_s() -> String {
    format!("({})[{}]", convert_to_string(&get_string_constructor()), get_uint(9))
}

fn get_small_g() -> String {
    format!("({})[{}]", convert_to_string(&get_string_constructor()), get_uint(14))
}

fn encode_char_directly(c: char) -> Result<String, ()> {
    match c {
        'a' => Ok(get_small_a()),
        'd' => Ok(get_small_d()),
        'c' => Ok(get_small_c()),
        'e' => Ok(get_small_e()),
        'f' => Ok(get_small_f()),
        'g' => Ok(get_small_g()),
        'i' => Ok(get_small_i()),
        'l' => Ok(get_small_l()),
        'n' => Ok(get_small_n()),
        'o' => Ok(get_small_o()),
        'r' => Ok(get_small_r()),
        's' => Ok(get_small_s()),
        'S' => Ok(get_big_s()),
        't' => Ok(get_small_t()),
        'u' => Ok(get_small_u()),
        'v' => Ok(get_small_v()),
        ' ' => Ok(get_space()),
        '(' => Ok(get_left_par()),
        ')' => Ok(get_right_par()),
        '[' => Ok(get_left_bracket()),
        ']' => Ok(get_right_bracket()),
        '{' => Ok(get_left_brace()),
        '}' => Ok(get_right_brace()),
        _ => Err(())
    }
}

fn encode_small_letter(c: char) -> String {
    match encode_char_directly(c) {
        Ok(s) => s,
        Err(_) => {
            let number = 10 + ((c as u8) - b'a');
            format!(
                "({})[{}]({})", 
                get_uint(number as u32),
                format!(
                    "({})+({})+({})+({})+({})+({})+({})+({})",
                    get_small_t(),
                    get_small_o(),
                    get_big_s(),
                    get_small_t(),
                    get_small_r(),
                    get_small_i(),
                    get_small_n(),
                    get_small_g(),
                ),
                get_uint(36),
            )
        },
    }
}

fn encode_string_only_small_letter_and_space(s: &str) -> Result<String, ()> {
    let mut result = String::new();
    for i in s.chars() {
        match i {
            'a'..='z' => result.push_str(&format!("({})", encode_small_letter(i))),
            ' ' => result.push_str(&format!("({})", get_space())),
            _ => return Err(()),
        }
        result.push_str("+");
    }
    result.push_str("[]");
    Ok(result)
}

fn get_function_constructor() -> String {
    format!("[][{}][{}]", encode_string_only_small_letter_and_space("find").unwrap(), encode_string_only_small_letter_and_space("constructor").unwrap())
}

fn get_big_c() -> String {
    format!("({})({})()(({})[{}]())[{}]", get_function_constructor(), encode_string_only_small_letter_and_space("return escape").unwrap(), get_empty_string(), encode_string_only_small_letter_and_space("italics").unwrap(), get_uint(2))
}

fn get_from_char_code_method() -> String {
    format!(
        "({})[{}][{}]", 
        get_empty_string(),
        encode_string_only_small_letter_and_space("constructor").unwrap(),
        format!(
            "({})+({})+({})+({})+({})",
            encode_string_only_small_letter_and_space("from").unwrap(),
            get_big_c(),
            encode_string_only_small_letter_and_space("har").unwrap(),
            get_big_c(),
            encode_string_only_small_letter_and_space("ode").unwrap(),
        )
    )
}

fn encode_any_char(c: char) -> String {
    match c {
        ' ' => get_space(),
        'a'..='z' => encode_small_letter(c),
        'C' => get_big_c(),
        'S' => get_big_s(),
        '(' | ')' | '[' | ']' | '{' | '}' => encode_char_directly(c).unwrap(),
        '0'..='9' => convert_to_string(&get_uint((c as u32) - 0x30)),
        _ => format!(
            "({})({})",
            get_from_char_code_method(),
            get_uint(c as u32)
        )
    }
}

/**
 * Encode string to a jsfuck string.
 */
pub fn encode_string(s: &str) -> String {
    let mut result = String::new();
    for i in s.chars() {
        result.push_str(&format!("({})+", encode_any_char(i)));
    }
    result.push_str("[]");
    result
}

/**
 * Convert a string into jsfuck with eval.
 */
pub fn evaled_string(s: &str) -> String {
    format!("({})({})()", get_function_constructor(), encode_string(s))
}
