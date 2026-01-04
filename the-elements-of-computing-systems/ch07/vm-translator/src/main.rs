use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::{map, map_res};
use nom::sequence::preceded;
use nom::{Finish, IResult, Parser};

enum Command {
    Add,
    Eq,
    Gt,
    Sub,
    Neg,
    Not,
    Pop(Pop),
    Push(Push),
}

impl Command {
    fn to_assembly(&self, unique_label_suffix: &mut usize) -> String {
        match self {
            Command::Pop(cmd) => pop(cmd),
            Command::Push(cmd) => push(cmd),
            Command::Add => add(),
            Command::Eq => eq(unique_label_suffix),
            Command::Gt => gt(unique_label_suffix),
            Command::Sub => sub(),
            Command::Neg => neg(),
            Command::Not => not(),
        }
    }
}

struct Pop {
    base_address_register: MemorySegment,
    offset: u8,
}

struct Push {
    from: PushContent,
}

enum PushContent {
    Constant(u16),
    // i.e. which memory segment and at what offset
    Memory(MemorySegment, u8),
}

enum MemorySegment {
    // TODO - add all of them
    LCL,
    THIS,
}

fn main() {
    let foo = translate("push constant 40 \n push constant 2 \n gt");

    if let Ok(output) = foo.clone() {
        print!("{output}")
    }
}

fn make_unique_label(label: &str, _unique_label_suffix: &mut usize) -> String {
    let label = format!("{label}_{}", *_unique_label_suffix);

    *_unique_label_suffix += 1;

    label
}

fn translate(vm_code: &str) -> Result<String, String> {
    let mut unique_label_suffix = 0;

    Ok(vm_code
        .lines()
        .map(parse_one_line)
        .collect::<Result<Vec<Command>, String>>()?
        .iter()
        .map(|c| c.to_assembly(&mut unique_label_suffix))
        .collect())
}

fn parse_one_line(line: &str) -> Result<Command, String> {
    alt((
        parse_push, parse_pop, parse_add, parse_eq, parse_gt, parse_sub, parse_neg, parse_not,
    ))
    .parse(line)
    .finish()
    .map(|(_, command)| command)
    .map_err(|_| format!("Parsing failed for line: {}", line))
}

fn parse_memory_segment(input: &str) -> IResult<&str, MemorySegment> {
    let (input, _) = space0(input)?;
    let (input, memory_segment) = alt((
        map(tag("local"), |_| MemorySegment::LCL),
        map(tag("this"), |_| MemorySegment::THIS),
    ))
    .parse(input)?;

    Ok((input, memory_segment))
}

fn parse_push(line: &str) -> IResult<&str, Command> {
    let (line, _) = space0(line)?;
    let (line, _) = tag("push")(line)?;
    let (line, push_content) = parse_push_content(line)?;

    Ok((line, Command::Push(Push { from: push_content })))
}

fn parse_constant(input: &str) -> IResult<&str, PushContent> {
    map(
        preceded(
            space0,
            preceded(
                tag("constant "),
                map_res(digit1, |digits: &str| digits.parse::<u16>()),
            ),
        ),
        |value| PushContent::Constant(value),
    )
    .parse(input)
}

fn parse_push_content(input: &str) -> IResult<&str, PushContent> {
    let (input, _) = space0(input)?;
    let (input, push_content) = alt((
        // TODO - actually parse the offset (maybe we have a VirtualMemoryTarget type?)
        map(parse_memory_segment, |segment| {
            PushContent::Memory(segment, 8)
        }),
        parse_constant,
    ))
    .parse(input)?;

    Ok((input, push_content))
}

fn parse_pop(line: &str) -> IResult<&str, Command> {
    let (line, _) = space0(line)?;
    let (line, _) = tag("pop")(line)?;
    let (line, memory_segment) = parse_memory_segment(line)?;
    let (line, _) = space0(line)?;
    let (_, offset) = map_res(digit1, |digits: &str| digits.parse::<u8>()).parse(line)?;

    Ok((
        line,
        Command::Pop(Pop {
            base_address_register: memory_segment,
            offset,
        }),
    ))
}

fn parse_add(line: &str) -> IResult<&str, Command> {
    let (line, _) = space0(line)?;
    let (_, _) = tag("add")(line)?;

    Ok((line, Command::Add))
}

fn parse_eq(line: &str) -> IResult<&str, Command> {
    let (line, _) = space0(line)?;
    let (_, _) = tag("eq")(line)?;

    Ok((line, Command::Eq))
}

fn parse_gt(line: &str) -> IResult<&str, Command> {
    let (line, _) = space0(line)?;
    let (_, _) = tag("gt")(line)?;

    Ok((line, Command::Gt))
}

fn parse_sub(line: &str) -> IResult<&str, Command> {
    let (line, _) = space0(line)?;
    let (_, _) = tag("sub")(line)?;

    Ok((line, Command::Sub))
}

fn parse_neg(line: &str) -> IResult<&str, Command> {
    let (line, _) = space0(line)?;
    let (_, _) = tag("neg")(line)?;

    Ok((line, Command::Neg))
}

fn parse_not(line: &str) -> IResult<&str, Command> {
    let (line, _) = space0(line)?;
    let (_, _) = tag("not")(line)?;

    Ok((line, Command::Not))
}

const TRUE: &str = "-1";
const FALSE: &str = "0";

// push
// increment the stack pointer
// if constant, skip, else read from the appropriate virtual segment into a temp reg
// write the value at the temp register into the memory pointed at by stack pointer
fn push(command: &Push) -> String {
    match command {
        Push {
            from: PushContent::Constant(constant),
        } => {
            format!(
                "
//// START PUSH (CONSTANT)
@SP
M=M+1
@{}
D=A
@SP
A=M
M=D",
                constant
            )
        }
        Push {
            from: PushContent::Memory(segment, offset),
        } => {
            // TODO - extract this thing into MemorySegment::to_assembly_string
            let target_register = match segment {
                MemorySegment::LCL => "LCL",
                MemorySegment::THIS => "THIS",
            };

            format!(
                "
//// START PUSH (SEGMENT)
@{}
D=M
@{}
D=D+A
A=D
D=M
@SP
A=M
M=D",
                target_register, offset
            )
        }
    }
}

//////
// read the value at the current stack pointer into D and update the stack pointer to SP--
const POP_INTO_D: &str = "
//// START POP_INTERNAL (stores popped value in D and decrements the stack pointer)
@SP
D=M
A=D
D=M
@SP
M=M-1
//// END POP_INTERNAL
";

fn pop(command: &Pop) -> String {
    let target_register = match command.base_address_register {
        MemorySegment::LCL => "LCL",
        MemorySegment::THIS => "THIS",
    };

    let offset = command.offset.to_string();

    // pop into D and store it in R13
    // figure out where we are writing to by reading the base address of segment + offset
    // store the target address in R14
    // set D to the popped value
    // set A to the target address
    // store the popped value into the taget address (M=D)
    format!(
        "//// START POP (R13-popped value; R14-target memory address)
         {POP_INTO_D}
         @R13
         M=D
         @{target_register}
         D=M
         @{offset}
         D=D+A
         @R14
         M=D
         @R13
         D=M
         @R14
         A=M
         M=D
         //// END POP"
    )
}

fn pop_two_arguments() -> String {
    format!(
        "
//// POP TWO ARGUMENTS (R13-first argument, D-second argument)
{POP_INTO_D}
// move first argument into @R13
@R13
M=D
{POP_INTO_D}
"
    )
}

const PUSH_D_INTO_THE_STACK: &str = "
// PUSH D into the stack
@SP
M=M+1
A=M
M=D
";

fn add() -> String {
    let pop_two = pop_two_arguments();

    format!(
        "
//// START ADD
{pop_two}
  // Add D and R13
@R13
D=D+M
{PUSH_D_INTO_THE_STACK}
"
    )
}

fn sub() -> String {
    let pop_two = pop_two_arguments();

    format!(
        "
//// START SUB
{pop_two}
// Subtract D and R13
@R13
D=D-M
{PUSH_D_INTO_THE_STACK}",
    )
}

fn neg() -> String {
    format!(
        "
//// START NEG
{POP_INTO_D}
D=-D
{PUSH_D_INTO_THE_STACK}"
    )
}

fn eq(unique_label_suffix: &mut usize) -> String {
    let pop_two = pop_two_arguments();

    let is_equal_label = make_unique_label("IS_EQUAL", unique_label_suffix);
    let end_eq_label = make_unique_label("END_EQ", unique_label_suffix);

    format!(
        "
//// START EQ
{pop_two}
// Subtract D and R13
  @R13
  D=D-M
  @{is_equal_label}
  D;JNE
// DID NOT JUMP, SO IT IS EQUAL
  @{end_eq_label}
  D={TRUE};JMP
({is_equal_label})
  D={FALSE}
({end_eq_label})
  {PUSH_D_INTO_THE_STACK}",
    )
}

fn gt(unique_label_suffix: &mut usize) -> String {
    let pop_two = pop_two_arguments();

    let is_greater_than_label = make_unique_label("IS_GREATER_THAN", unique_label_suffix);
    let end_gt_label = make_unique_label("END_GT", unique_label_suffix);

    format!(
        "
//// START GT
{pop_two}
// Subtract D and R13
  @R13
  D=D-M
  @{is_greater_than_label}
  D;JGT
// DID NOT JUMP, SO IT IS NOT GREATER THAN
  @{end_gt_label}
  D={FALSE};JMP
({is_greater_than_label})
  D={TRUE}
({end_gt_label})
  {PUSH_D_INTO_THE_STACK}",
    )
}

fn not() -> String {
    format!(
        "
//// START NOT
{POP_INTO_D}
D=!D
{PUSH_D_INTO_THE_STACK}"
    )
}
