# JS tests

This folder contains the end-to-end tests for the Buri JS output. This doc outlines how to use and run these tests.

## Running the tests

From the repo root, run the following commands:

```bash
cargo run rust/e2e # build the JS outputs
bun wiptest # test the JS outputs
```

If both commands succeed, the tests passed!

## Adding a new test

This directory contains two types of tests. First, in the `/tests/js/valid/` directory are all tests for valid Buri code. We expect Buri code in here to both compile and function correctly. Second, in the `/tests/js/invalid/` directory are all tests for invalid Buri code. We expect all Buri code in here to fail to compile.

### Adding a new valid test

Use the following steps to add a new valid test:

1. Add a new Buri file to the `/tests/js/valid/` directory (subdirectories are allowed)
2. Write the Buri code you want to test inside this file
3. In the same location of Buri file, add a new file with the extension `.test.js`. By convention, the name of this file should match the name of the Buri file. So if you added a file called `foo.buri`, you should add a file called `foo.test.js`.
4. In this JS file, import anything you exported from the Buri file (except for types). The identifiers are the same. However, ensure you update the file path to start with `@tests/js/valid/` instead of `tests/js/valid/` and change the file extension to `.mjs` instead of `.buri`.
5. In this JS file, write any assertions you want to test using Bun's testing framework. While the testing framework isn't well documented, reference [Jest](https://jestjs.io/docs/en/using-matchers) since it's very similar to Bun's testing framework.

Once you've finished writing your test, run the tests as described above. If the test passes, you're done!

#### Debugging a valid test

If a test fails, here's a few steps you should use to debug it:

1. If the Buri file fails to build, ensure it's written correctly. Unfortunately we don't have great error messages yet, so you may need to debug it manually. It's also possible there's a bug in the compiler, so feel free to open an issue if you think that's the case.
2. If the Bun test fails, check Bun's terminal output. Perhaps it'll give you a hint as to what went wrong (e.g., function returned `25` instead of `42`).
3. If the Bun test fails, you can also check the JS output. All JS files are located in `.buri/dist/` and have the same relative folder structure as the Buri files in the test folder.

### Adding a new invalid test

To add a new invalid test, just add a new file to the `/tests/js/invalid/` directory with the `.buri` extension. The file should contain the invalid Buri code, and can go in subdirectories of `/tests/js/invalid/`. The testing framework will automatically pick up the new file and ensure it fails to compile.
