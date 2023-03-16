import { BhelloWorld } from "@tests/js/valid/simple-programs/hello-world.mjs"
import { expect, it } from "bun:test"

it("hello world", () => {
    expect(BhelloWorld()).toEqual("Hello World!")
})
