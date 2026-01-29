use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::{map, map_res};
use nom::sequence::preceded;
use nom::{Finish, IResult, Parser};

#[derive(Copy, Clone)]
enum Command {
    Pop(Pop),
    Push(Push),
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl Command {
    fn to_assembly(&self, unique_label_suffix: &mut usize) -> String {
        match self {
            Command::Pop(cmd) => pop(cmd),
            Command::Push(cmd) => push(cmd),
            Command::Add => add(),
            Command::Sub => sub(),
            Command::Neg => neg(),
            Command::Eq => eq(unique_label_suffix),
            Command::Gt => gt(unique_label_suffix),
            Command::Lt => lt(unique_label_suffix),
            Command::Or => or(),
            Command::And => and(),
            Command::Not => not(),
        }
    }
}

#[derive(Copy, Clone)]
struct Pop {
    target: MemoryTarget,
}

#[derive(Copy, Clone)]
struct Push {
    from: PushContent,
}

#[derive(Copy, Clone)]
enum PushContent {
    Constant(u16),
    // i.e. which memory segment and at what offset
    Memory(MemoryTarget),
}

#[derive(Copy, Clone)]
struct MemoryTarget {
    segment: MemorySegment,
    index_within_segment: u8,
}

#[derive(Copy, Clone, PartialEq)]
enum MemorySegment {
    // TODO - add all of them
    ARG,
    LCL,
    THIS,
    THAT,
    TEMP,
    POINTER,
    STATIC,
}

impl MemorySegment {
    fn to_assembly_register_str(&self) -> &str {
        match self {
            MemorySegment::ARG => "ARG",
            MemorySegment::LCL => "LCL",
            MemorySegment::THIS => "THIS",
            MemorySegment::THAT => "THAT",
            MemorySegment::TEMP => "TEMP",
            // TODO - explain how if we had a better type, then this would not be required
            MemorySegment::POINTER => {
                unreachable!("explain how if we had a better type, then this would not be required")
            }
            // TODO - explain how if we had a better type, then this would not be required
            MemorySegment::STATIC => {
                unreachable!("explain how if we had a better type, then this would not be required")
            }
        }
    }
}

fn main() {
    let foo = translate(
        "push constant 111
push constant 333
push constant 888
pop static 8
pop static 3
pop static 1
push static 3
push static 1
sub
push static 8
add",
    );

    match foo {
        Ok(asm) => print!("{asm}"),
        Err(err) => print!("{err}"),
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
        parse_push,
        parse_pop,
        _parse_command("add", Command::Add),
        _parse_command("sub", Command::Sub),
        _parse_command("neg", Command::Neg),
        _parse_command("eq", Command::Eq),
        _parse_command("gt", Command::Gt),
        _parse_command("lt", Command::Lt),
        _parse_command("and", Command::And),
        _parse_command("or", Command::Or),
        _parse_command("not", Command::Not),
    ))
    .parse(line)
    .finish()
    .map(|(_, command)| command)
    .map_err(|_| format!("Parsing failed for line: {}", line))
}

fn parse_memory_target(input: &str) -> IResult<&str, MemoryTarget> {
    let (input, _) = space0(input)?;
    let (input, segment) = alt((
        map(tag("argument"), |_| MemorySegment::ARG),
        map(tag("local"), |_| MemorySegment::LCL),
        map(tag("this"), |_| MemorySegment::THIS),
        map(tag("that"), |_| MemorySegment::THAT),
        map(tag("temp"), |_| MemorySegment::TEMP),
        map(tag("pointer"), |_| MemorySegment::POINTER),
        map(tag("static"), |_| MemorySegment::STATIC),
    ))
    .parse(input)?;

    let (input, _) = space0(input)?;
    let (_, index_within_segment) =
        map_res(digit1, |digits: &str| digits.parse::<u8>()).parse(input)?;

    Ok((
        input,
        MemoryTarget {
            segment,
            index_within_segment,
        },
    ))
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
        map(parse_memory_target, PushContent::Memory),
        parse_constant,
    ))
    .parse(input)?;

    Ok((input, push_content))
}

fn parse_pop(line: &str) -> IResult<&str, Command> {
    let (line, _) = space0(line)?;
    let (line, _) = tag("pop")(line)?;
    let (line, memory_target) = parse_memory_target(line)?;

    Ok((
        line,
        Command::Pop(Pop {
            target: memory_target,
        }),
    ))
}

fn _parse_command<'a>(
    keyword: &'static str,
    command: Command,
) -> impl Fn(&'a str) -> IResult<&'a str, Command> {
    move |input: &'a str| {
        let (input, _) = space0(input)?;
        let (input, _) = tag(keyword)(input)?;
        Ok((input, command))
    }
}

const TRUE: &str = "-1";
const FALSE: &str = "0";

// figure out where we are writing to by EITHER
//   reading the base address of segment + offset
//   OR if it's in TEMP, then just 5 + offset
//   OR if it's POINTER, then it's either THIS or THAT, depending on the offset
// store the target address in R14
fn store_target_address_into_r14(target: &MemoryTarget) -> String {
    match target.segment {
        MemorySegment::TEMP => {
            let address = 5 + target.index_within_segment;
            format!(
                "@{address}
            D=A
            @R14
            M=D
            "
            )
        }
        MemorySegment::POINTER => {
            let address = if target.index_within_segment == 0 {
                "THIS"
            } else {
                "THAT"
            };
            format!(
                "@{address}
            D=A
            @R14
            M=D
            "
            )
        }
        MemorySegment::STATIC => {
            // TODO - filename should come from main or something
            let address = format!("FILENAME.{}", target.index_within_segment);
            format!(
                "@{address}
            D=A
            @R14
            M=D
            "
            )
        }
        _ => {
            let target_register = target.segment.to_assembly_register_str();
            let offset = target.index_within_segment.to_string();

            format!(
                "
         @{target_register}
         D=M
         @{offset}
         D=D+A
         @R14
         M=D "
            )
        }
    }
}

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
//////////////////////////////
//////////////////////////////
//// START PUSH (CONSTANT) ///
@{constant}
D=A
{PUSH_D_INTO_THE_STACK}"
            )
        }
        Push {
            from: PushContent::Memory(target),
        } => {
            let store_target_address_in_r14 = store_target_address_into_r14(target);

            format!(
                "
//////////////////////////////
//////////////////////////////
//// START PUSH (SEGMENT) ////
{store_target_address_in_r14}
// this assumes that the previous command leaves A = R14
A=M
D=M
{PUSH_D_INTO_THE_STACK}"
            )
        }
    }
}

//////
// update the stack pointer to SP-- and read the value at the current stack pointer into D
const POP_INTO_D: &str = "
//// START POP_INTERNAL (decrements the stack pointer and stores the popped value in D)
@SP
M=M-1
A=M
D=M
//// END POP_INTERNAL
";

fn pop(command: &Pop) -> String {
    let store_target_address_into_r14 = store_target_address_into_r14(&command.target);

    format!(
        "
        /////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////
        //// START POP (R13-popped value; R14-target memory address) ////
        /// 
        // pop into D and store it in R13
         {POP_INTO_D}
         @R13
         M=D
         {store_target_address_into_r14}
        // set D to the popped value
         @R13
         D=M
        // set A to the target address
         @R14
         A=M
        // store the popped value into the taget address (M=D)
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
A=M
M=D
@SP
M=M+1
";

fn binary_operation(operation: &str) -> String {
    let pop_two = pop_two_arguments();

    format!(
        "
//////////////////////
//////////////////////
//// START {operation}
{pop_two}
@R13
D=D{operation}M
{PUSH_D_INTO_THE_STACK}
"
    )
}

fn add() -> String {
    binary_operation("+")
}

fn sub() -> String {
    binary_operation("-")
}

fn or() -> String {
    binary_operation("|")
}

fn and() -> String {
    binary_operation("&")
}

fn neg() -> String {
    format!(
        "
///////////////////
///////////////////
//// START NEG ////
{POP_INTO_D}
D=-D
{PUSH_D_INTO_THE_STACK}"
    )
}

fn comparison_operation(
    operation_name: &str,
    comparison: &str,
    unique_label_suffix: &mut usize,
) -> String {
    let pop_two = pop_two_arguments();

    let successful_comparison_label = make_unique_label(operation_name, unique_label_suffix);
    let end_label = make_unique_label("END_COMPARISON", unique_label_suffix);

    format!(
        "
///////////////////////////////////
///////////////////////////////////
//// START {operation_name}
{pop_two}
// Subtract D and R13
  @R13
  // TODO - I think we can make the following three instruction into 2
  D=D-M
  @{successful_comparison_label}
  D;{comparison}
// DID NOT JUMP, SO IT IS NOT {operation_name}
  @{end_label}
  D={FALSE};JMP
({successful_comparison_label})
  D={TRUE}
({end_label})
  {PUSH_D_INTO_THE_STACK}",
    )
}

fn eq(unique_label_suffix: &mut usize) -> String {
    comparison_operation("IS_EQUAL_TO", "JEQ", unique_label_suffix)
}

fn gt(unique_label_suffix: &mut usize) -> String {
    comparison_operation("IS_GREATER_THAN", "JGT", unique_label_suffix)
}

fn lt(unique_label_suffix: &mut usize) -> String {
    comparison_operation("IS_LESS_THAN", "JLT", unique_label_suffix)
}

fn not() -> String {
    format!(
        "
/////////////////////
/////////////////////
//// START NOT //////
 {POP_INTO_D}
D=!D
{PUSH_D_INTO_THE_STACK}"
    )
}
