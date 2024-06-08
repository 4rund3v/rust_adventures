### Common Programmign Concepts
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
       - 
