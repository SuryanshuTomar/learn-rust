// => Why RUST ?
// 1. It has High level language feature without performance penalties.
// 2. We can enforce program behaviors at the compile time itself which will result in better program reliability.
// 3. It has Built-in dependency management, similar to npm.
// 4. Friendly and welcoming developer community with growing ecosystem of libraries.

// => Features of RUST -
// 1. It is first-class support for multithreading.
// 2. Rust is a strongly type and statically typed language, so variable and expression types are determined and checked at compile time, which helps enhance memory safety and error detection, resulting in more reliable builds.
// 3. It supports module system that makes code seperation simple.
// 4. We can add dependency using only a single line in the config file.
// 5. It also comes with other tools for generating documentation, linting and formatting code. 

// --------------------------------------------------------------------------
// => DATA TYPES -

// 1. Memory only stores binary data and anything + everything is represented in the form of binary.
// 2. The program that we write determins what that binary will be.
// 3. Basic types that are univesally useful are provided by the Rust language are - 
//   a. Boolean - true, false
//   b. Numbers - 
//       i. Integer - 1, 2, 40
//       ii. Double / Float - 1.1, 2.3, 4.0006
//   d. Charachter - 'A', 'z', '3', '%'
//   e. String - "Hello", "This"


// --------------------------------------------------------------------------
// => VARIABLES -

// 1. Assign data to a temporary memory location and allows the programmer to easily work with memory.
// 2. Can be set to any value with its type declared.
// 3. Immutable by default, but can become mutable.
//   a. Immutable means the value that we have stored in the variable cannot be changed.
//   b. Mutable means that value that we have stored in our variable can be replaced with another value but with the same type as before.

// -> Syntax - 
// 1. Immutable - 
//    let(keyword) [variable_name] = [variable_value / literal] 
// 2. Mutable - 
//    let(keyword) mut(keyword) [variable_name] = [variable_value / literal]

// examples - 
// 1. let two = 2;
// 2. let hello = "hello";
// 3. let charachter = 'j';
// 4. let myHalf = 0.5;
// 5. let closeCode = false;
// 6. let yourHalf = myHalf;
// 7. let mut myName = "Imposter"
// 8. myName = "Imposter Dev"


// --------------------------------------------------------------------------
// => DECLARING TYPES -

// 1. If we declare a variable normally, like -> let varName = "value", then, the compiler will infer its type automatically. But we can also declare the variable type.

// -> Syntax -
// 1. Immutable - 
//    let(keyword) [variable_name]:[data_type] = [variable_value / literal] 
// 2. Mutable - 
//    let(keyword) mut(keyword) [variable_name]:[data_type] = [variable_value / literal]

// -> Number Data Types -
// 1. Integer Data Types - 
//   a. Signed Integers - These can be negatives and positive integers (LMB is used to store the sign of the integer)
//       8bit    -   i8     - Range(2^7)   -128 to 127.
//       16bit   -   i16    - Range(2^15)  -32768 to 32767.
//       32bit   -   i32    - Range(2^31)  -2147483647 to 214748364
//       64bit   -   i64    - Range(2^63)
//       128bit  -   i128   - Range(2^127)

//   a. Unsigned Integers - These can are always positive integers 
//       8bit    -   u8     - Range(2^128)  0 to 255
//       16bit   -   u16    - Range(2^128)  0 to 65536
//       32bit   -   u32    - Range(2^128)  0 to 4294967296
//       64bit   -   u64    - Range(2^128)
//       128bit  -   u128   - Range(2^128)

// -> How to write Number literals in Rust
// Number         literals
// Decimal	      98_222
// Hex	          0xff
// Octal	        0o77
// Binary	        0b1111_0000
// Byte(u8 only)	b'A'

// 2. Integer Data Types - 
// a. Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. 
// b. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision.
// c. All floating-point types are signed.
// e. Processors does not read 1 byte at a time but 1 word at a time from the memory. In 32bit processor, it can access 4bytes(1word) at a time(32 bits) and In 64bit processor, it can access 8bytes(1word) at a time(64 bits).

// Note: By default, the type of integer is i32 and for float it is f64

// Note: We cannot assign value from one data type to another data type, if we have already declared the data type of the variable.

// Note: 
// We can also annotate a type of a value on the value itself.
// For example - 
// let mut z = 10_u8;

// --------------------------------------------------------------------------
// => FUNCTIONS -

// 1. Functions are a way to encapsulate a program functionality so that we dont have to write that piece of code multiple times in our program.
// 2. It can optionally accepts data.
// 3. It can optionally returns data.
// 4. It is used to writing more readable code.

