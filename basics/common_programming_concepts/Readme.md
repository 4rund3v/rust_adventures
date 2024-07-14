## Common Programming Concepts
We learn about the variables, basic types, functions, comments and the control flow.

Variables are immutable by default
 - When trying to reassign a variable with an value, we get the error ```^^^^^^ cannot assign twice to immutable variable```
 - constants are values bound to a name & are not allowed to change
   - defined with the `const` keyword and the `type` annotation
   - defined in any scope, global too
   - the constant should be set only to a constant expression, not a result computed at runtime
   ``` const TIME_TO_LIVE: u32 = 60*30;   ```
   - Concept of SHADOWING
     - we can declare a new variable with the same name as the previous variable
     - the first variable is shadowed by the second
     - this means that the second variable is the one that the compiler sees.
     - it ( second variable) uses the same variable until it is overshadowed or the scope ends
     - The difference between shadowing and mut
       - in shadowing you are  creating a new variable with the same name in that scope
       - the new variable can be of any type, whereas the mut indicates that the variable must be of the same type.

### Data types
Every value in rust is of a certain data type, within the category of scalar or compound
Since rust is a statically typed language, at the compile time each variable must be associated with its data type. If missed, the compiler throws an error, type annotations needed.

#### Scalar Types
The scalar type represents the single value, there are 4 such primary scalar types
integers, floating-point numbers, booleans, characters

  Integers :
    The number without the fractional part, this has 2 variants the signed integer and the unsigned integer, the signed integer has the support for the negative numbers whereas the unsigned are all positive.
    ```
       i8 - indicates [ -(2^7) to (2^7) - 1 ]  which is from -128 to 127
       u8 - indicates [ 2^8 - 1 ] which is from 0 to 255
    ```
    Additional types, isize(signed) and usize(unsigned) indicate the size depending on the computer architecture, 32bit or 64bit  (based on the architecture)
    Note while iterating through an array, the index variable is the usize or an variant of the usize variable.
      variants :
       i8,i16,i32,i64,i128,isize
      Integer Literals: 
        Number literals that can be multiple numerical types allow a type suffix such as 76u16, also the literals can use _ , such as 1000 or 1_000
          decimal literal  --- 2.431 2_431
          hex - 0xff
          octal - 0o77
          binary - 0b1111_0000
          byte (u8 only) - b'A'
        When the value of a integer exceeds it max possible value, then the rust program panics and terminates. in the release mode it will wrap around and return the value

    Floating Point Literals: 
      The default is f64 and all floating point numbers are by default signed values. f32 and f64 are options available.
      floating point numbers are represented according to ieee-754 standard, f32 is a single precision float and f64 has double precision.

    Numeric Operations:
      Rust supports the basic mathematical operations such as addition, subtraction, multiplication, division and remainder. Integer division truncates towards 0 to nearest integer ( +, - , * , / , % )
  
  Boolean Type:
    true and false, 1 byte in size.
  
  Character Type:
    chinese, etc. The character type is 4 bytes in size and represents a unicode scalar value. Character types are defined in single-quote.  
---

#### Compound Types
  Compound types can group multiple values into one type. The primitive compound types: tuples and arrays
    - The tuple type
        The Tuple is a general way of grouping togehter a number of values and variety of types into one compound type. Tuple have a fixed length once declared they cannot grow or shrink in size.
        to access the tuple by index, var.index
        to print it {:?} then the var ```println!("tuple : {:?}", tup_var);```
        The tuple without any values has a special name - unit,  the type and value of the unit is -- ()
        Expressions implicitly return the unit value if they don't return any other value.

    - The array type
        The array is a collection of multiple values, all values must have the same type
        Useful when we want the data to be allocated in the stack instead of the heap
        - Similar to the vector type: the vector is a collection provided by the standard library and is allowed to
          grow or shrink in size.
---

## Functions
  Functions are the main component of the rust programs. they are declared with the `fn` keyword
  Rust uses snake case as the convention style for the function and variable names.
  The order of defining the functions does not matter.
  the function declaration needs the arguments to have their types defined.
  Rust is an expression based language
   - Statements - are instructions that perform some action and do not return a value.
   - Expressions - evaluate to a resultant value. Expressions evaluate to a value and make up most of the code in rust.
    In functions while returning an expression, there is no `;` Expressions do not include trailing semicolons, if an `;` is added then its turned into an statement and will not return a value, it will return the unit value () instead of the expected one.
---

### Conditionals and Loops
Rust provides `if and else if` blocks for checking conditions that evaluate to a boolean value,
non boolean values are not automatically converted.
  Rust also provides the match clause to match the value, like a switch case statement.
  The if evaluates to an expression, can be used in a let statement, both arms to be having the same data type.
  For loops the rust language provides, `loop`, `while`, and `for`.
  loop:
    - continue: to skip over the flow below it
    - break: statement to stop the execution of the loop
    - loop returns the value; when the break expression is followed by the value to be returned.
    - the break and continue usually apply to the innermost loops within which they are present
    - with loop labels we can optionally specify a loop label on a loop, which when used on the label
      breaks/continues the corresponding loop.
    - 
