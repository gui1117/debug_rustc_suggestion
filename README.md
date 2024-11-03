Issue about rustc suggestion.

* Go to dir `some-bin`
* run `cargo check`
* See the error message:
    ```
        Checking some-bin v0.1.0 (/home/gui/Developpement/debug_rustc_suggestion/some-bin)
    error[E0599]: no function or associated item named `bar` found for struct `Foo` in the current scope
     --> src/main.rs:3:15
      |
    3 |     let a = Foo::bar();
      |                  ^^^ function or associated item not found in `Foo`
      |
      = help: items from traits can only be used if the trait is in scope
    help: trait `Bar` which provides `bar` is implemented but not in scope; perhaps you want to import it
      |
    1 + use some_inner_lib::Bar;
      |

    For more information about this error, try `rustc --explain E0599`.
    error: could not compile `some-bin` (bin "some-bin") due to 1 previous error
    ```
* Now use suggestion
* rerun `cargo check`
* It didn't work indeed:
    ```
        Checking some-bin v0.1.0 (/home/gui/Developpement/debug_rustc_suggestion/some-bin)
    error[E0432]: unresolved import `some_inner_lib`
     --> src/main.rs:2:5
      |
    2 | use some_inner_lib::Bar;
      |     ^^^^^^^^^^^^^^ use of undeclared crate or module `some_inner_lib`

    error[E0599]: no function or associated item named `bar` found for struct `Foo` in the current scope
     --> src/main.rs:4:15
      |
    4 |     let a = Foo::bar();
      |                  ^^^ function or associated item not found in `Foo`
      |
      = help: items from traits can only be used if the trait is in scope
    help: trait `Bar` which provides `bar` is implemented but not in scope; perhaps you want to import it
      |
    1 + use some_inner_lib::Bar;
      |

    Some errors have detailed explanations: E0432, E0599.
    For more information about an error, try `rustc --explain E0432`.
    error: could not compile `some-bin` (bin "some-bin") due to 2 previous errors
    ```
* The real fix would be `use some_inner_lib_rename::Bar;`

I believe this wrong suggestion appears because we do some "renaming" in the cargo toml when we import the crate:
```toml
some-inner-lib-rename = { path = "../some-inner-lib", package = "some-inner-lib" }
```
With combination with usage of trait (If we use a struct like Foo, then the suggestion is correct).
