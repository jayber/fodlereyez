
# Folder-Size

This is an exercise in

1. File and folder listing and iterating in Rust
2. Which became an exercise in mocking external libraries for unit tests in Rust
3. And then a learning how to handle references and borrowing exercise.

Using a [cursive](https://github.com/gyscos/cursive) tui frontend for 1.

For 2. I took the approach of creating proxy traits for any

- External functions
- Types used by these functions that I could not instantiate

Thereby decoupling my code from the external libraries (except where I could instantiate and return
some library types
in my mocks).

I then used trait objects in my client code that would be either

* production types that run external library code (the only code that uses the external libraries),
  or
* [mockall](https://github.com/asomers/mockall) mocks during test

I would be interested in finding out if there are better ways of achieving the same for unit tests
that are quicker and
less verbose. It took ***way*** longer to set up the mocks and write the tests than the actual logic
code. And they take
up way more lines. This doesn't mean they are bad, but it reinforces the idea that unit testing can
be an expensive
hassle.

BTW *Integration* tests for this particular problem would have made more sense, but don't affect the
value of the
exercise
as it is. But remember:

**_Unit tests are useful; Integration tests tell you if it actually works._**

The work of 3. is still ongoing. Cursive's user_data allows you to avoid using references
(cursive owns the data you want to display), which ended up solving all the lifetime problems I
had. I still think this is hiding important knowledge on how to _actually_ use borrows and
lifetime annotations correctly.

I started another repo [james_vs_borrow_checker](https://github.com/jayber/james_vs_borrow_checker)
to illustrate the compilation problems I encounter and their solutions (hopefully)