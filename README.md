# I/O adapters

This crate provides adapters to compose writeable traits in the standard library. The following
conversions are available:

- `fmt::Write` -> `io::Write`
- `io::Write` -> `hash::Hasher`

## Use case

Suppose you are writing a function which emits human-readable data in a zero-alloc way. The best
interface looks something like this:

```rust,ignore
fn foo<Out: fmt::Write>(mut output: Out, ...) {
    // Do stuff
    writeln!(output, "My computation: {result}").unwrap();
}
```

Notice the use of `fmt::Write`: using this trait provides a type-system guarantee that the data
written is valid UTF-8, hence why it should be preferred over `io::Write`.

Now users of this API can gather data into a `String`, provide their own `fmt::Write`
implementation, etc. The problem you'll run into is if you'd like to send the output of this
function to stdout: there is no built-in way to do so! That's where this crate comes in by providing
an adapter, so you can write the following:

```rust,ignore
foo(&mut io::stdout().write_adapter());
```
