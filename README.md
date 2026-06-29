# ABAL (A Boring-Ass Language)

## How to build and run

1. Install rust and deps
- llvm-22
- clang-22
- libpolly-22
- libzstd

2. Build and run with
```bash
cargo build
./compile_and_run SOURCE
```

## Syntax

Syntax is fairly c-like

```
main() {
    let x = 6;
    print_int(x);
}
```

Return type is declared with a rust-like syntax, but the idea is to infer it whenever possible once we get to that point.

```
test_function() -> Int {
    return 34;
}
```

The point of the language is to avoid doing anything super fancy to get to the hard parts as fast as possible.
Here are some points where I couldn't help but do something different.

### Callee expressions with dot syntax

I consider javascript-style callee expressions quite unreadable
```js
(some_function_expression)(args)
f1(args1)(args2)(args3)
```

I still wanted to include some sort of callee-expressions, so I decided to require a dot between the callee and the args.
```
(some_function_expression).(args)
f1(args1).(args2).(args3)
```

### Type declarations and type casts look identical

```
let x = Int 6;
```

## e2e tests with lit

1. Create venv (if first time)
```bash
python3 -m venv lit_tests/venv
```

2. Activate venv
```bash
source lit_tests/venv/bin/activate
```

3. Install deps (if first time)
```bash
pip install -r lit_tests/requirements.txt
```

4. Run tests
```bash
cargo test
lit lit_tests
```
