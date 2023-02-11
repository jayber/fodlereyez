# Folder-Size

This is a personal exercise in

1. File and folder listing and iterating in rust
2. Which became an exercise in mocking external libraries for unit tests in Rust
3. And then a learning how to handle references and borrowing exercise.

I took the approach of creating proxy traits for any

- External functions
- Types used by these functions that I could not instantiate

Thereby decoupling my code from the external libraries (except where I could instantiate and return some library types
in my mocks).

I then used trait objects in my client code that would be either

* production types that run external library code (the only code that uses the external libraries), or
* [mockall](https://github.com/asomers/mockall) mocks during test

I would be interested in finding out if there are better ways of achieving the same for unit tests that are quicker and
less verbose. It took ***way*** longer to set up the mocks and write the tests than the actual logic code. And they take
up way more lines. This doesn't mean they are bad, but it reinforces the idea that unit testing can be an expensive
hassle.

BTW *Integration* tests for this particular problem would have made more sense, but don't affect the value of the
exercise
as it is. But remember:

**_Unit tests are useful; Integration tests tell you if it actually works._**