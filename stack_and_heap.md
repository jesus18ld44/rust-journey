# Stack and the Heap

The stack follows a LIFO structure. All data stored on the stack must have a known, fixed size

Data with unknown size at compile time or a size that might change must be stored on the heap  instead

When putting data on the heap, you request a certain amount of spac. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location

When the code calls a function, the values passed into the function (including potentially, pointers to data on the heap) and the function's local variables get pushed onto the stack. 
When the function is over, those values get popped off the stack


