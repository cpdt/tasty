# TASTY

> Tom's Amazingly Simple Templates, Yeah!

**TASTY** is a template format for defining just dang simple templates.

**Note:** This (semi-)specification and library currently only exist for some internal purposes in [Axiom](https://github.com/monadgroup/axiom) - beyond that unfortunately I don't have the time to maintain or add features to it. However if this is useful to you, more contributors are always welcome!

### Show me some examples

Yes sir.

You can substitute variables:

```rust
fn main() {
    {{FUNC_NAME}}();
}
// with `FUNC_NAME` = "hello_world", becomes:
fn main() {
    hello_world();
}
```

You can substitute variables in variables:

```rust
fn main() {
   {{{{TO_CALL}}_NAME}}();
}
// with `TO_CALL` = "THING", and `THING_NAME` = "nice_one", becomes:
fn main() {
    nice_one();
}
```

You can loop:

```rust
fn main() {
    {%LOOP {{LOOP_TIMES}}%}
    {{FUNC_NAME_{{LOOP_INDEX}}}}();
    {%END%}
}
// With:
//  - LOOP_TIMES = "3"
//  - FUNC_NAME_0 = "first_func"
//  - FUNC_NAME_1 = "second_func"
//  - FUNC_NAME_2 = "third_func"
// becomes:
fn main() {
    first_func();
    second_func();
    third_func();
}
```

You can branch:

```rust
fn main() {
    {%LOOP {{LOOP_TIMES}}%}
    {%IF {%NOT {{LOOP_INDEX}}%}%}
    called_first_iteration();
    {%END%}
    {%IF {{LOOP_INDEX}}%}
    called_every_other_iteration();
    {%END%}
    {%END%}
}
// With `LOOP_TIMES` = "3", becomes:
fn main() {
    called_first_iteration();
    called_every_other_iteration();
    called_every_other_iteration();
}
```

And you can set scoped variables:

```rust
fn main() {
    {%WITH NAMESPACE={{AUTHOR}}_{{TOOLKIT}}, FUNC_NAME={{NAMESPACE}}_{{FUNC_NAME}}%}
    {{FUNC_NAME}}();
    {%END%}
    {{FUNC_NAME}}();
}
// With `AUTHOR` = "me", `TOOLKIT` = "stuff" and `FUNC_NAME` = "the_func", becomes:
fn main() {
    me_stuff_the_func();
    the_func();
}
```

And that's literally it! You can do a lot with these primitives, and even more by putting the complex logic in the underlying controller, where it lives.


### Things that would be nice to have (in order)

 - Proper documentation
 - Tests
 - Ability to put in a `{{` or `{%` without it being interpreted as the start of some statement (i.e escaping it)
 - Cleaner code (everything's just a quickly written hack at the moment, but it works ¯\\\_(ツ)_/¯)
 - Put it on crates.io

### License

MIT, yo. Check the LICENSE file for more details.
