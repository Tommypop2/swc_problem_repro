# SWC Problem Repro

- All tests pass with `cargo test`.
- Run `wasm-pack build --target web`, the output javascript in `./pkg/swc_problem_repro.js` contains `import * as __wbg_star0 from 'env';`, which shouldn't occur and indicates a problem with compatibility.
- When the lines for transpiling typescript in the transform are removed, correct javascript is generated.
