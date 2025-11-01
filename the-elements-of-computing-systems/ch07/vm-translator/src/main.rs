use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0};
use nom::combinator::{map, map_res};
use nom::{Finish, IResult, Parser};

struct Pop {
    base_address_register: MemorySegment,
    offset: u8,
}

enum MemorySegment {
    // TODO - add all of them
    LCL,
    THIS,
}

fn main() {
    println!("Hello, world!");
}

fn translate(vm_code: &str) -> String {
    let (_, parsed_pop) = parse_pop(vm_code)
        .finish()
        .expect("need to make translate return a result");
    pop(parsed_pop)
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

fn parse_pop(line: &str) -> IResult<&str, Pop> {
    let (line, _) = space0(line)?;
    let (line, _) = tag("pop")(line)?;
    let (line, memory_segment) = parse_memory_segment(line)?;
    let (line, _) = space0(line)?;
    let (_, offset) = map_res(digit1, |digits: &str| digits.parse::<u8>()).parse(line)?;

    Ok((
        line,
        Pop {
            base_address_register: memory_segment,
            offset,
        },
    ))
}

fn pop(command: Pop) -> String {
    let target_register = match command.base_address_register {
        MemorySegment::LCL => "LCL",
        MemorySegment::THIS => "THIS",
    };

    let offset = command.offset.to_string();

    // pop(internal)
    //////
    // read the value at the current stack pointer into D
    // put it in a temp register
    // update the stack pointer to SP--

    // pop <segment> <offset>
    // pop_internal
    // figure out where we are writing to by reading the base address of segment + offset
    //  store the target address into a temp register
    // set D to the popped value
    // set A to the target address
    // M=D
    format!(
        "//// START POP (stores target memory address in R14)
              //// START POP_INTERNAL (stores popped value in R13)
              @SP
              D=M
              A=D
              D=M
              @R13
              M=D
              @SP
              M=M-1
              //// END POP_INTERNAL
          @{}
          D=M
          @{}
          D=D+A
          @R14
          M=D
          @R13
          D=M
          @R14
          A=M
          M=D
          //// END POP",
        target_register, offset
    )
}

#[cfg(test)]
mod tests {
    use crate::translate;

    #[test]
    fn pop() {
        let vm_code = "pop local 2";

        let expected = "\
          //// START POP (stores target memory address in R14)
              //// START POP_INTERNAL (stores popped value in R13)
              @SP
              D=M
              A=D
              D=M
              @R13
              M=D
              @SP
              M=M-1
              //// END POP_INTERNAL
          @LCL
          D=M
          @2
          D=D+A
          @R14
          M=D
          @R13
          D=M
          @R14
          A=M
          M=D
          //// END POP";

        assert_eq!(translate(vm_code), expected);
    }
}
