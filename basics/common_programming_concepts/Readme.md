## Common Programmign Concepts
We learn about the variables, basic types, functions, comments and the control flow.

Variables are immutable by default
 - When trying to reassign a variable with an value, we get the error ```^^^^^^ cannot assign twice to immutable variable```
 - constants are values bound to a name & are not allowed to change
   - defined with the `const` keyword and the `type` annotation
   - defined in any scope, global too
   - the constant should be set only to a constant expression, not a result computed at runtime
   ``` const TIME_TO_LIVE: u32 = 60*30;   ```
   - Concept of SHADOWING
     - we can declare a new variable with the same name sa the previous variable
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
    Additional types, isize and usize indicate the size depending on the computer architecture, 32bit or 64bit  (based on the architecture)
      variants :
       i8,i16,i32,i64,i128,isize
      Integer Literals: 
        Number literals that can be multiple numerica types allow a type suffix such as 76u16, also the literals can use _ , such as 1000 or 1_0000
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
  ---
  Boolean Type:
    true and false, 1 byte in size.
  ---
  Character Type:
    chinese, etc. The character type is 4 bytes in size and represents a unicode scalar value.
  Character types are defined in single-quote.
  ---

#### Compound Types
  Compound types can group multiple values into one type. The primitive compound types: tuples and arrays
    - The tuple type
    - The array type 