use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, digit1, space0};
use nom::combinator::map_res;
use nom::error::Error;
use nom::error::ErrorKind::Tag;
use nom::sequence::{delimited, preceded, terminated};
use nom::{Finish, IResult, Parser};
use std::collections::BTreeMap;

fn main() {
    // Note: I'm not going to work on the file reading part as that's not very interesting for me
    // let source_file = vec![];
}

// TODO - make the error better typed? Well, first actually fail if the input is not valid assembly
fn do_it(assembly: &str) -> Result<String, String> {
    let mut env: Environment = Environment {
        inner: BTreeMap::default(),
    };

    Ok(assembly
        .lines()
        .filter_map(|raw_line| {
            parse(raw_line, &mut env).map(|inst| inst.to_binary_instruction(&env))
        })
        .collect::<Vec<String>>()
        .join(""))
}

enum Line {
    Comment,
    Instruction(Instruction),
    Label(String),
}

enum Instruction {
    SetTheARegister(Value),
    Compute(String),
}

impl Instruction {
    fn to_binary_instruction(self, env: &Environment) -> String {
        match self {
            Instruction::SetTheARegister(Value::Literal(int)) => format!("{:016b}", int),
            Instruction::SetTheARegister(Value::Variable(variable)) => {
                // TODO - would be better to return an Err instead of panicking
                env.get(&variable).expect("program is invalid if variable is not present at this point").clone()
            }
            Instruction::Compute(c) => c,
        }
    }
}

enum Value {
    Literal(u16),
    Variable(String),
}

struct Environment {
    inner: BTreeMap<String, String>,
}

impl Environment {
    // TODO - I think we should disallow overwriting a variable.
    fn set(&mut self, entry: (String, String)) -> () {
        self.inner.insert(entry.0, entry.1);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.inner.get(key)
    }
}

fn parse_comment(raw_line: &str) -> IResult<&str, Line> {
    let (raw_line, _) = space0(raw_line)?;
    let (raw_line, _) = tag("//")(raw_line)?;

    Ok((raw_line, Line::Comment))
}

fn parse_label(raw_line: &str) -> IResult<&str, Line> {
    let (raw_line, _) = space0(raw_line)?;

    let (raw_line, label) = delimited(tag("("), alphanumeric1, tag(")")).parse(raw_line)?;

    Ok((raw_line, Line::Label(String::from(label))))
}

fn parse_literal(raw_line: &str) -> IResult<&str, Value> {
    let (raw_line, int) = map_res(digit1, |digits: &str| digits.parse::<u16>()).parse(raw_line)?;

    Ok((raw_line, Value::Literal(int)))
}

fn parse_variable(raw_line: &str) -> IResult<&str, Value> {
    let (raw_line, variable) = alpha1.parse(raw_line)?;

    Ok((raw_line, Value::Variable(String::from(variable))))
}

fn parse_the_a_register_instruction(raw_line: &str) -> IResult<&str, Line> {
    let (raw_line, _) = tag("@")(raw_line)?;
    let (raw_line, value) = alt((parse_literal, parse_variable)).parse(raw_line)?;

    Ok((
        raw_line,
        Line::Instruction(Instruction::SetTheARegister(value)),
    ))
}

fn parse_dest(raw_line: &str) -> IResult<&str, String> {
    let (raw_line, dest) = terminated(alpha1, tag("=")).parse(raw_line)?;

    // TODO - what's the right way to do this? If this is it, then Tag is probably wrong
    // ALTERNATIVELY - maybe there's a way to specify what characters are to be parsed, so
    let err = nom::Err::Error(Error::new(raw_line, Tag));

    // mmm maybe there's a clever way to do this whereby I just parse a list of characters and
    // flip the appropriate bit if the character is present? That way I'd allow any permutation and
    // not have to 'spell' all of them out in the match statement
    match dest {
        "D" => Ok((raw_line, "010".to_string())),
        "AMD" => Ok((raw_line, "111".to_string())),
        _ => Err(err),
    }
}

fn parse_comp(raw_line: &str) -> IResult<&str, String> {
    let (raw_line, comp) = alphanumeric1(raw_line)?;

    // TODO - what's the right way to do this? If this is it, then Tag is probably wrong
    // ALTERNATIVELY - maybe there's a way to specify what characters are to be parsed, so
    let err = nom::Err::Error(Error::new(raw_line, Tag));

    match comp {
        "0" => Ok((raw_line, "0101010".to_string())),
        "1" => Ok((raw_line, "0111111".to_string())),
        _ => Err(err),
    }
}

fn parse_jump(raw_line: &str) -> IResult<&str, String> {
    let (raw_line, jump) = preceded(tag(";"), alpha1).parse(raw_line)?;

    // TODO - what's the right way to do this? If this is it, then Tag is probably wrong
    // ALTERNATIVELY - maybe there's a way to specify what characters are to be parsed, so
    let err = nom::Err::Error(Error::new(raw_line, Tag));

    match jump {
        "JGT" => Ok((raw_line, "001".to_string())),
        "JEQ" => Ok((raw_line, "010".to_string())),
        "JGE" => Ok((raw_line, "011".to_string())),
        "JLT" => Ok((raw_line, "100".to_string())),
        "JNE" => Ok((raw_line, "101".to_string())),
        "JLE" => Ok((raw_line, "110".to_string())),
        "JMP" => Ok((raw_line, "111".to_string())),
        _ => Err(err),
    }
}

fn parse_c_instruction(raw_line: &str) -> IResult<&str, Line> {
    println!("parse_c_instruction {}", raw_line);

    let (raw_line, dest) = parse_dest(raw_line).unwrap_or((raw_line, "000".to_string()));
    let (raw_line, comp) = parse_comp(raw_line)?;
    let (raw_line, jump) = parse_jump(raw_line).unwrap_or((raw_line, "000".to_string()));

    let line = Line::Instruction(Instruction::Compute(format!("111{}{}{}", comp, dest, jump)));

    Ok((raw_line, line))
}

fn parse_instruction(raw_line: &str) -> IResult<&str, Line> {
    let (raw_line, _) = space0(raw_line)?;
    alt((parse_the_a_register_instruction, parse_c_instruction)).parse(raw_line)
}

fn parse_line(raw_line: &str) -> IResult<&str, Line> {
    println!("raw in parse {}", raw_line);
    alt((parse_comment, parse_label, parse_instruction)).parse(raw_line)
}

fn parse(raw_line: &str, env: &mut Environment) -> Option<Instruction> {
    parse_line(raw_line)
        .finish()
        .ok()
        .and_then(|(_, instruction)| match instruction {
            Line::Comment => None,
            Line::Label(label) => {
                env.set((label, "0000".to_string()));
                None
            }
            Line::Instruction(inst) => Some(inst),
        })
}

#[cfg(test)]
mod tests {
    use crate::do_it;

    #[test]
    fn just_comments() {
        let source = "// This assembly program \
                      // does absolutely nothing \
                      // but is a valid program";

        assert_eq!(do_it(source), Ok("".to_string()))
    }

    #[test]
    fn set_the_a_register_to_a_literal_value() {
        let source = "@42";

        assert_eq!(do_it(source), Ok("0000000000101010".to_string()))
    }

    #[test]
    fn set_the_d_register_to_zero() {
        let source = "D=0";

        assert_eq!(do_it(source), Ok("1110101010010000".to_string()))
    }

    #[test]
    fn set_everything_to_one_then_jump() {
        let source = "AMD=1;JMP";

        assert_eq!(do_it(source), Ok("1110111111111111".to_string()))
    }

    // #[test]
    // fn set_and_use_a_label() {
    //     let source = "(start)
    //                     D=0
    //                     @start";
    // 
    //     let expected = "1110101010010000\
    //                     0000000000000001"
    //         .to_string();
    // 
    //     assert_eq!(do_it(source), Ok(expected))
    // }

    // TODO - disallow integer literals bigger than whatever 15bit allows
}
