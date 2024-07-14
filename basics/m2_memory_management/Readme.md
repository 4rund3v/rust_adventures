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

#### Return Values and Scope
---
### References and Borrowing
#### Mutable References
#### Dangling References
#### Rules of References
#### Slice Type
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