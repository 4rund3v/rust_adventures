### Memory Management
In rust the memory management is done by the system of ownership with a set of rules that the compiler checks.
To understand this better, firstly the 
#### Stack and heap
  Stack stores the data in the order it gets them and removes them in the opposite order.
	i.e. its stacked on top of each other.
	Last In / First out
	adding data onto the stack is called as pushing onto
	removing data is called as popping off
	ALL data stored on the stack must have a known, fixed size on the time of compilation,
	Data with dynamic/changing size must be stored on the heap.
  Heap is less organized, for data on the heap an request is made for a certain amount of space.
	The memory allocator finds an empty slot in the heap that is big enough, marks it as being used and returns an pointer which is the address to that location.
	This process is called the allocation on the heap.
	Because the pointer to the heap is a known fixed size value, you can store the pointer on the stack,
	When the data is requested, the pointer from the stack is followed on until the data is fetched.
  Comparisons
    -  Pushing onto the stack is always faster than allocating on the heap. Because the allocator never has to search for a place to store new data. The location is always at the top of the stack.
    - Accessing data on the heap is slower than accessing data on the stack because you have to follow a pointer to fetch the data.

  Flow
    When the code calls a function, the values passed into the function ( including potentially, pointers to data on the heap) and the functions local variables get pushed onto the stack, when the function is executed and over, those values are popped of the stack.
    Ownership tries to address the following ( main purpose is to manage the heap data)
      1. Keeping track of what parts of code are using what data on the heap
      2. minimizing the amount of duplicate data on the heap
      3. cleaing up unused data on the heap
  
#### Ownership rules
  ** Rules of Ownership  **
  	1. Each value in Rust has an owner
  	2. There can only be one and only one owner for a value at a time.
  	3. When the owner goes out of scope, the value gets dropped.

#### Memory Allocation and Scope
  The pattern of deallocating resources at the end of an items lifetime is called as Resource Acquistion is Initialization. **RAII**. The drop function will be the same concept.
  When a variable goes out of scope Rust calls a special function for us. The function is called as *drop* , when invoked will return the memory back to the allocator.
  Usually the process is as follows.
	1. Request for allocation
	2. allocate()
	3. the memory is allocated and utilized
	4. The task is done, the memory is to be cleaned up 
	5. free() is invoked.
  ** In rust the memory is automatically returned once the variable that owns it goes out of scope.**
  so when the following is executed
  ```
  let s1 = String::from("Whoa this string is awesome!");
  let s2 = s1;
  ```
  To be able to use the s1 post assignment to s2, we would need to manually specify that the s2 needs to clone s2.
  ```
  let s1 = String::from("Whoa this string is awesome!");
  let s2 = s1.clone();
  ```
  By doing so the s1 can be then used, the clone() method then duplicates the heap data and updates the pointer accordingly.

  What happens here is that the s1 is no longer a valid variable, itan invalidated reference
  Its like a shallow copy and then does a move. s1 was moved into s2
  Rust will never automatically create a deep copy of the data, its only data in the stack that gets copied.
  Under the hood when the line `let x = 5; let y = x;` is executed, for the simple scalar data types the underlying, Copy trait, is invoked for the integer type and another copy is made on the stack for the type.
  The *Copy Trait* can only be applied to static/simple data types and not to those which invoke the allocate() method.

### References and Borrowing
  The references to a variable are indicated via the & ( ampersands, representing the refernces)
  It creates a pointer to the object. They allow you to refer to some value without taking ownership of it.
  The concept of creating the references via the & is called as borrowing.
  Since like all variables, the references are also immutable, to have the references as immutable,
#### Mutable References
  we add the `&mut` prefix to it, indicating that its a mutable pointer, a pointer to a mutable memory location.
  Mutable refernces have one big restriction, you can have only 1 mutable reference to a value at a time.
  The first mutable borrow and its last use must be before a second mutable reference is made and used.
  Read only references are allowed to be made in parallel/within same scope.
  Mutable references, only one valid within its scope.

  The benifit of this is that rust can prevent data races at compile time, a data race is similar to a race condition and happens when these three behavior occur.
    - Two or more pointers access the same data at the same time.
    - At least one of the pointers is being used to write to the data.
    - There is no mechanism being used to synchronize access to the data.
  This ensures that runtime data race conditions are avoided
  to create multiple mutable references to the same variable, create scopes {} and use them

  Another limitation with immutable references is that it cannot exist in prallel with an mutable reference.
  The SCOPE OF A REFERENCE IS FROM THE POINT IT IS INTRODUCED AND CONTINUES THROUGH TO THE LAST TIME THAT REFERENCE IS USED. POST WHICH THE REFERENCES IS DE_SCOPED.
#### Dangling References
  The scenario where there is a reference to an memory and that memory gets cleared up, but the reference still exists - dangling references, this scenario does not occur since in rust while compiling it checks if the owner of the data is in scope of the reference of the data. If not, compilation fails.
  Hence this scenario would not occur.
#### Rules of References
  At any given time, you can have either one mutable reference or any number of immutable references
  References must always be valid.

#### Slice Type
  Another kind of reference : Slices
  Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
  Slice is a kind of reference, it does not have ownership.
  In the case of String the pointer datastrcuture holds the following,
   1. ptr: The address of the location where the memory is located
   2. len : The length of the pointer, how many slots in the mem is used
   3. capacity : The max length (in terms of bytes) allocated by the memory-allocator 
  The  slice reference pointer under the hood has the following
   1. ptr - the address of the first byte of the starting point for the data
   2. len - number of blocks/length of the pointer.
  String slice range indices must occur at valid UTF-8 character boundaries If you attempt to create a string slice in the middle of a multibyte character your program will exit with an error.
  
##### String Slices
##### String Literals as Slices
##### String Literals as Parameters
----
### Structs, Enums and Pattern Matching
##### Instantiating Structs
##### Defining Enum
##### Match Control Flow Construct
---
### Managing Packages, Crates and Modules
##### Packages and Crates
##### Modules for scope/privacy.
---
### Common Collections
#### Vectors
#### Strings
#### HashMaps
---
### Error Handling
#### Unrecoverable Errors and panic
#### Panic/Recoverable errors
---
### Automated Tests
#### Tests
#### Test Control
#### Test Organization
---
### IO
#### CLI Program
#### Read a file