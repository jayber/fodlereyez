# Folder-Size

This is a personal exercise in

1. File and folder listing and iterating in rust
2. Then an exercise in mocking external libraries for unit tests in Rust

I took the approach of creating proxy traits for any   
- External functions
- Types that I could not instantiate

And then used trait objects in my client code which would be either 
* production types that run external library code, or
* [mockall](https://github.com/asomers/mockall) mocks during test

I would be interested in finding out if there are better ways of achieving the same for unit tests that are quicker and 
less verbose. It took ***way*** longer to set up the mocks and write the tests than the actual logic code. And they take 
up way more code. This doesn't mean they are bad, but it reinforces the idea that unit testing can be a massive hassle and
expensive.

Also, the current version is only tested by virtue of the mocks receiving their expectations, rather than actual functionality.
This is a fault really. To be meaningful they should verify return values.

BTW *Integration* tests for this particular problem would have made more sense, but don't affect the value of the exercise
as it is. But remember:

**_Unit tests are useful; Integration tests are essential._**