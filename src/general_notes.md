# Gantt chart app notes

[Crust of Rust: std::collections](https://www.youtube.com/watch?v=EF3Z4jdD1EQ)

## General

Rust Lang comes with Three main crates, core, alloc and std. Core requires 'nothing', alloc requires a memory allocator, std requires operating system abstractions.

## What are lifetimes [`'a`]

Named regions of code that a reference must be valid for.

```rust
fn example(){
  let r; 
  {
    let x = 42;
    r = &x;    
  }
  println!("{}",*r);

    //                  |let r;               |     R is uninitialised therefore has no reference 
    //                  | {                   |    
    //-------+--'x      |   let x = 42;       |     X is initialised and is limited to a scope Q
    //       |          |                     |        
    //-------+--+-- 'r  | r = &x;             |     R now has a value, which is limited to a scope Q
    //       |  |       |                     |    
    //-------+  |       | }                   |     Scope Q collapsed
    //          |       |                     |
    //----------+       | println!("{}",*r);  |     Calling 
    //
    //
```

## What are vectors [`Vec`]

Collections in rust are part of std's sequence sup-types (Vec, vecdeque and linked lists)

`std::collections::vec` are composed of a raw `vec<T,A>` type and a `usize`. forming a chunk of contiguous memory of a certain size.

### Creating Vectors

Calling the basic `Vec<T>::new()` func estimates the minimum amount of capacity required for the given `T` type (Smaller data types will be used more frequently therefore require more capcity).

- *Capacity:* Total amount of storage in the current vector.
- *Length:*   Amount of allocated elements.

Given that vecs are dynamic storage types, when length reaches capacity and a new element is added to the vec, a new portion of memory is allocated (with the new capacity) and all the data is copied over.

### Growing Vectors

When growth does occur the `::grow_amoritized(...)` is called, it insures the current capacity is either doubled, or will contain the amount of space required to push new elements (if specified) (will also ensure the capcity is greater than the non_zero cap).

If there is a capacity estimate, its worth while to construct new vecs via `Vec<T>::with_capacity(capacity:usize)` to save on having to copy memory.

Advanced options with `::try_reserver(self, additional:usize)` returns a `Result<(),TryReserveError>` type based on whether there is sufficient additional contiguous memory to allocate new elements.

### Deleting Elements

Removals from vectors can be very expensive.

### `remove()`

Vecs are not allowed gaps between their elements. Deleting the second element in a million length vec will require 999,998 mem copies to complete.

### `swap_remove()`

If order is arbritrary, `swap_remove(&mut self, index:usize)` will remove the selected element, allocaing the element in the last position of the vec in ints place. Trading off ordering creates O(1) time complexity.

### `retain ()`

This method will optimise mass deletion via a specific predicate. Only retaining elements which return true vec.retain(|&x| x % 2 == 0)`.

## Utility

### `leak()`

This method creates a static mutable slice. This is useful for configuration data or anything else that lives for the entirity of the program. **Note there is no way to clear the leak until the program exits.**

## What are Vectors [`VecDeque`]

Defined as a double ended queue with a growable ring buffer. Rather than a capacity = length check. The ring data structure compares start and end pointers (off by one equality) to detect vacancy.

Being a set of pointers, the queue can operate as both a stack (LIFO) and a queue (FIFO).

### Growing a Queue

#### Standard Operations

|`pop()`|`push()`|
|   -   |   -   |
|`pop_front()`|`push_front()`|
|`pop_back()`|`push_front()`|

#### `make_contiguous()`, `as_slices()`,`as_mut_slices()`

Unlike a vec, queue doesn't exist as contiguous memory.

`make_contiguous()` can perform a mem copy to assert contiguity, this function also returns a reference to this contiguous memory valid for this point in time(dropping the reference/forming a ring buffer breaks the contiguity).

`as_slices()` and `as_mute_slices()` differ as they both return a reference to possibly two slices, the first from start to length or end of the buffer, and the second (only in the case of wrap around), from the start of the buffer. to the length of the queue. Naturally they differ between eachother in only mutability.

