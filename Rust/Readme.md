Solutions in this folder are configured to run against Rust 1.21.0 to match the version used by codeforces. Set up is easy using rustup, just execute the following while in this directory.

```
$> rustup install 1.21.0
$> rustup set override 1.21.0
```

In order to run a solution compile it with the rustc compiler and then run the resulting binary directly.This is also done to imitate the codeforces environment with a single file and no crates.

```
$> rustc <problem>.rs && ./<problem>
```

