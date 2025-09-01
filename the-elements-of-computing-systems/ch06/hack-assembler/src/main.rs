use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{alpha1, alphanumeric1, digit1, space0};
use nom::combinator::{eof, map_res};
use nom::error::Error;
use nom::error::ErrorKind::Tag;
use nom::multi::many1;
use nom::sequence::{delimited, preceded, terminated};
use nom::{Finish, IResult, Parser};
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

fn main() {
    // Note: I'm not going to work on the file reading part as that's not very interesting for me
    // let source_file = vec![];
    let assembly =
        std::fs::read_to_string("./test-files/Pong.asm").expect("File needs to be there");

    let compiled = do_it(assembly.as_str()).unwrap();

    std::fs::write("./test-files/Pong.compiled", compiled).unwrap();
}

// TODO - make the error better typed? Well, first actually fail if the input is not valid assembly
fn do_it(assembly: &str) -> Result<String, String> {
    let mut env = Environment::new();

    let parsed_lines: Result<Vec<Line>, _> = assembly
        .lines()
        .map(|raw_line| {
            parse_line(raw_line).map(|line| {
                env.process_line(&line);
                line
            })
        })
        .collect();

    let r: Result<String, _> = parsed_lines.map(|lines| {
        lines
            .into_iter()
            .filter_map(|line| line.to_binary_instruction(&mut env))
            .collect::<Vec<String>>()
            .join("")
    });

    r.map_err(|e| e.to_string())
}

enum Line {
    Comment,
    EmptyLine,
    Instruction(Instruction),
    Label(String),
}

impl Line {
    fn to_binary_instruction(&self, env: &mut Environment) -> Option<String> {
        match self {
            Line::Instruction(instruction) => Some(instruction.to_binary_instruction(env)),

            Line::Comment | Line::Label(_) | Line::EmptyLine => None,
        }
    }
}

enum Instruction {
    SetTheARegister(Value),
    Compute(String),
}

impl Instruction {
    fn to_binary_instruction(&self, env: &mut Environment) -> String {
        match self {
            Instruction::SetTheARegister(Value::Literal(int)) => format!("{:016b}", int),
            Instruction::SetTheARegister(Value::Variable(variable)) => env.get2(&variable).clone(),
            Instruction::Compute(c) => c.clone(),
        }
    }
}

enum Value {
    Literal(u16),
    Variable(String),
}

#[derive(Debug)]
struct Environment {
    current_instruction_address: u16,
    inner: BTreeMap<String, u16>,
    next_variable_address: u16,
}

impl Environment {
    fn new() -> Self {
        let mut inner = BTreeMap::new();

        inner.insert(String::from("R0"), 0);
        inner.insert(String::from("SP"), 0);
        inner.insert(String::from("R1"), 1);
        inner.insert(String::from("LCL"), 1);
        inner.insert(String::from("R2"), 2);
        inner.insert(String::from("ARG"), 2);
        inner.insert(String::from("R3"), 3);
        inner.insert(String::from("THIS"), 3);
        inner.insert(String::from("R4"), 4);
        inner.insert(String::from("THAT"), 4);
        inner.insert(String::from("R5"), 5);
        inner.insert(String::from("R6"), 6);
        inner.insert(String::from("R7"), 7);
        inner.insert(String::from("R8"), 8);
        inner.insert(String::from("R9"), 9);
        inner.insert(String::from("R10"), 10);
        inner.insert(String::from("R11"), 11);
        inner.insert(String::from("R12"), 12);
        inner.insert(String::from("R13"), 13);
        inner.insert(String::from("R14"), 14);
        inner.insert(String::from("R15"), 15);

        inner.insert(String::from("SCREEN"), 16384);
        inner.insert(String::from("KBD"), 24576);

        Environment {
            current_instruction_address: 0,
            inner,
            next_variable_address: 16,
        }
    }

    // TODO - I think we should disallow overwriting a variable.
    fn set(&mut self, entry: (String, u16)) -> () {
        self.inner.insert(entry.0, entry.1);
    }

    fn get2(&mut self, key: &str) -> String {
        match self.inner.entry(key.to_string()) {
            Entry::Vacant(vacant) => {
                let next_address = self.next_variable_address;
                vacant.insert(next_address);
                self.next_variable_address += 1;
                format!("{:016b}", next_address)
            }
            Entry::Occupied(occ) => {
                format!("{:016b}", occ.get())
            }
        }
    }

    fn process_line(&mut self, line: &Line) -> () {
        match line {
            Line::Instruction(_) => {
                self.current_instruction_address += 1;
            }

            Line::Label(label) => self.set((label.clone(), self.current_instruction_address)),

            Line::Comment | Line::EmptyLine => {}
        }
    }
}

fn parse_comment(raw_line: &str) -> IResult<&str, Line> {
    let (raw_line, _) = space0(raw_line)?;
    let (raw_line, _) = tag("//")(raw_line)?;

    Ok((raw_line, Line::Comment))
}

fn parse_empty_line(raw_line: &str) -> IResult<&str, Line> {
    let (raw_line, _) = terminated(space0, eof).parse(raw_line)?;

    Ok((raw_line, Line::EmptyLine))
}

fn parse_label(raw_line: &str) -> IResult<&str, Line> {
    let (raw_line, _) = space0(raw_line)?;

    let (raw_line, label) = delimited(
        tag("("),
        // TODO extract this as it's reused elsewhere
        take_while1(|char| (char != ' ') && (char != ')')),
        tag(")"),
    )
    .parse(raw_line)?;

    Ok((raw_line, Line::Label(String::from(label))))
}

fn parse_literal(raw_line: &str) -> IResult<&str, Value> {
    let (raw_line, int) = map_res(digit1, |digits: &str| digits.parse::<u16>()).parse(raw_line)?;

    Ok((raw_line, Value::Literal(int)))
}

fn parse_variable(raw_line: &str) -> IResult<&str, Value> {
    let (raw_line, variable) =
        take_while1(|char| (char != ' ') && (char != ')')).parse(raw_line)?;

    Ok((raw_line, Value::Variable(String::from(variable))))
}

fn parse_the_a_register_instruction(raw_line: &str) -> IResult<&str, Line> {
    let (raw_line, _) = tag("@")(raw_line)?;
    // Note - the order in the alt is important: a literal is just digits, whereas a variable
    // will parse on anything, including digits
    let (raw_line, value) = alt((parse_literal, parse_variable)).parse(raw_line)?;

    Ok((
        raw_line,
        Line::Instruction(Instruction::SetTheARegister(value)),
    ))
}

fn parse_dest(raw_line: &str) -> IResult<&str, String> {
    let (raw_line, dest) = terminated(alpha1, tag("=")).parse(raw_line)?;

    // TODO - what's the right way to do this? If this is it, then Tag is probably wrong
    let err = nom::Err::Error(Error::new(raw_line, Tag));

    // mmm maybe there's a clever way to do this whereby I just parse a list of characters and
    // flip the appropriate bit if the character is present? That way I'd allow any permutation and
    // not have to 'spell' all of them out in the match statement
    match dest {
        "M" => Ok((raw_line, "001".to_string())),
        "D" => Ok((raw_line, "010".to_string())),
        "MD" => Ok((raw_line, "011".to_string())),
        "A" => Ok((raw_line, "100".to_string())),
        "AM" => Ok((raw_line, "101".to_string())),
        "AD" => Ok((raw_line, "110".to_string())),
        "AMD" => Ok((raw_line, "111".to_string())),
        _ => Err(err),
    }
}

fn parse_comp(raw_line: &str) -> IResult<&str, String> {
    let (raw_line, comp) = many1(alt((
        alphanumeric1,
        tag("+"),
        tag("-"),
        tag("!"),
        tag("&"),
        tag("|"),
    )))
    .parse(raw_line)?;

    // TODO - what's the right way to do this? If this is it, then Tag is probably wrong
    let err = nom::Err::Error(Error::new(raw_line, Tag));

    // mmm here as well the clever way is probably to figure out if there's a recipe, perhaps
    // you start with a sequence of bits, and then if you encounter a specific character you flip
    // a certain bit. For example, if you see `M`, then just change the first bit to 1
    match comp.concat().as_str() {
        "0" => Ok((raw_line, "0101010".to_string())),
        "1" => Ok((raw_line, "0111111".to_string())),
        "-1" => Ok((raw_line, "0111010".to_string())),
        "D" => Ok((raw_line, "0001100".to_string())),
        "A" => Ok((raw_line, "0110000".to_string())),
        "M" => Ok((raw_line, "1110000".to_string())),
        "!D" => Ok((raw_line, "0001101".to_string())),
        "!A" => Ok((raw_line, "0110001".to_string())),
        "!M" => Ok((raw_line, "1110001".to_string())),
        "-D" => Ok((raw_line, "0001111".to_string())),
        "-A" => Ok((raw_line, "0110011".to_string())),
        "-M" => Ok((raw_line, "1110011".to_string())),
        "D+1" => Ok((raw_line, "0011111".to_string())),
        "A+1" => Ok((raw_line, "0110111".to_string())),
        "M+1" => Ok((raw_line, "1110111".to_string())),
        "D-1" => Ok((raw_line, "0001110".to_string())),
        "A-1" => Ok((raw_line, "0110010".to_string())),
        "M-1" => Ok((raw_line, "1110010".to_string())),
        "D+A" => Ok((raw_line, "0000010".to_string())),
        "D+M" => Ok((raw_line, "1000010".to_string())),
        "D-A" => Ok((raw_line, "0010011".to_string())),
        "D-M" => Ok((raw_line, "1010011".to_string())),
        "A-D" => Ok((raw_line, "0000111".to_string())),
        "M-D" => Ok((raw_line, "1000111".to_string())),
        "D&A" => Ok((raw_line, "0000000".to_string())),
        "D&M" => Ok((raw_line, "1000000".to_string())),
        "D|A" => Ok((raw_line, "0010101".to_string())),
        "D|M" => Ok((raw_line, "1010101".to_string())),
        _ => Err(err),
    }
}

fn parse_jump(raw_line: &str) -> IResult<&str, String> {
    let (raw_line, jump) = preceded(tag(";"), alpha1).parse(raw_line)?;

    // TODO - what's the right way to do this? If this is it, then Tag is probably wrong
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

fn parse_line(raw_line: &str) -> Result<Line, Error<&str>> {
    alt((
        parse_comment,
        parse_empty_line,
        parse_label,
        parse_instruction,
    ))
    .parse(raw_line)
    .finish()
    .map(|(_, line)| line)
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
    fn empty_line() {
        let source = "// This assembly program
                       
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

    #[test]
    fn set_m_to_m_plus_one() {
        let source = "M=M+1";

        assert_eq!(do_it(source), Ok("1111110111001000".to_string()))
    }

    #[test]
    fn set_and_use_a_label() {
        let source = "  D=0
                      (start)
                        D=0
                        @start";

        let expected = "1110101010010000\
                        1110101010010000\
                        0000000000000001"
            .to_string();

        assert_eq!(do_it(source), Ok(expected))
    }

    #[test]
    fn use_and_set_a_label() {
        let source = "  @END_EQ
                        D=0
                      (END_EQ)
                        D=0";

        let expected = "0000000000000010\
                        1110101010010000\
                        1110101010010000"
            .to_string();

        assert_eq!(do_it(source), Ok(expected))
    }

    #[test]
    fn use_a_variable() {
        let source = "@a
                      @b
                      @a";

        let expected = "0000000000010000\
                        0000000000010001\
                        0000000000010000"
            .to_string();

        assert_eq!(do_it(source), Ok(expected))
    }

    #[test]
    fn use_a_predefined_variable() {
        let source = "@R15";

        let expected = "0000000000001111".to_string();

        assert_eq!(do_it(source), Ok(expected))
    }

    #[test]
    fn example_from_book() {
        let source = "\
// Adds 1 + ... + 100
       @i
       M=1    // i=1
       @sum
       M=0    // sum=0
(LOOP)
       @i
       D=M    // D=i
       @100
       D=D-A  // D=i-100
       @END
       D;JGT  // if (i-100)>0 goto END
       @i
       D=M    // D=i
       @sum
       M=D+M  // sum=sum+i
       @i
       M=M+1  // i=i+1
       @LOOP
       0;JMP  // goto LOOP
 (END)
       @END
       0;JMP  // infinite loop";

        let expected = "0000000000010000\
                        1110111111001000\
                        0000000000010001\
                        1110101010001000\
                        0000000000010000\
                        1111110000010000\
                        0000000001100100\
                        1110010011010000\
                        0000000000010010\
                        1110001100000001\
                        0000000000010000\
                        1111110000010000\
                        0000000000010001\
                        1111000010001000\
                        0000000000010000\
                        1111110111001000\
                        0000000000000100\
                        1110101010000111\
                        0000000000010010\
                        1110101010000111"
            .to_string();

        assert_eq!(do_it(source), Ok(expected))
    }

    // TODO - disallow integer literals bigger than whatever 15bit allows
}
