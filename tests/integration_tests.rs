use lobe::{create_runtime, CellSize};

#[test]
fn test_parser_bracket_matching() {
    // Test valid bracket matching
    let src = "++[>+<-]";
    let result = create_runtime(src, CellSize::Bits8);
    assert!(result.is_ok());

    // Test unmatched opening bracket
    let src = "++[>+<-";
    let result = create_runtime(src, CellSize::Bits8);
    assert!(result.is_err());

    // Test unmatched closing bracket
    let src = "++>+<-]";
    let result = create_runtime(src, CellSize::Bits8);
    assert!(result.is_err());

    // Test nested brackets
    let src = "++[>+[<-]]";
    let result = create_runtime(src, CellSize::Bits8);
    assert!(result.is_ok());
}

#[test]
fn test_parser_comment_stripping() {
    // Test that non-BF characters are ignored
    let src = "This is a comment +>.< and more comment";
    let result = create_runtime(src, CellSize::Bits8);
    assert!(result.is_ok());
}

#[test]
fn test_hello_world() {
    // Classic BF "Hello World" program
    let hello_world = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    
    let mut runtime = create_runtime(hello_world, CellSize::Bits8).unwrap();
    let result = runtime.run();
    assert!(result.is_ok());
}

#[test]
fn test_simple_increment() {
    // Simple program: increment cell 0, output it
    let src = "+++++."; // Increment 5 times (5), then output
    
    let mut runtime = create_runtime(src, CellSize::Bits8).unwrap();
    let result = runtime.run();
    assert!(result.is_ok());
}

#[test]
fn test_data_pointer_movement() {
    // Move right, increment, move left, output
    let src = ">++<."; 
    
    let mut runtime = create_runtime(src, CellSize::Bits8).unwrap();
    let result = runtime.run();
    assert!(result.is_ok());
}

#[test]
fn test_empty_program() {
    let src = "";
    let mut runtime = create_runtime(src, CellSize::Bits8).unwrap();
    let result = runtime.run();
    assert!(result.is_ok());
}

#[test]
fn test_loop_execution() {
    // Simple loop: +[-] should set cell to 0
    let src = "+++++[-]"; // Set to 5, then loop until zero
    
    let mut runtime = create_runtime(src, CellSize::Bits8).unwrap();
    let result = runtime.run();
    assert!(result.is_ok());
}

#[test]
fn test_nested_loops() {
    // Test nested loops
    let src = "++[>++[>++<-]<-]";
    
    let mut runtime = create_runtime(src, CellSize::Bits8).unwrap();
    let result = runtime.run();
    assert!(result.is_ok());
}

#[test]
fn test_dynamic_tape_growth() {
    // Program that accesses high tape indices
    let src = ">>>>>>>>>++++++++++."; // Move right 9 times, then set to 10 and output
    
    let mut runtime = create_runtime(src, CellSize::Bits8).unwrap();
    let result = runtime.run();
    assert!(result.is_ok());
}

#[test]
fn test_pointer_wrapping() {
    // Test that pointer wraps around when decrementing from 0
    let src = "<<<+++."; // Move left (wraps), then increment and output
    
    let mut runtime = create_runtime(src, CellSize::Bits8).unwrap();
    let result = runtime.run();
    assert!(result.is_ok());
}

