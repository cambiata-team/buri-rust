import { BhelloString, BhelloWorldString, BemptyString, BnewlineString, BpiString } from "@tests/js/valid/strings/string.mjs"

import { expect, it } from "bun:test"

it("empty string literal should have the value of an empty string", () => {
    expect(BemptyString.valueOf()).toBe("")
})

it("hello string literal should have the value of hello", () => {
    expect(BhelloString.valueOf()).toBe("hello")
})

it("newline string literal should have the value of newline", () => {
    expect(BnewlineString.valueOf()).toBe("\n")
})

it("lowercase pi string literal should have the value of lowercase letter pi", () => {
    expect(BpiString.valueOf()).toBe("π")
})

it("hello concatenated with world should have the value of hello world", () => {
    expect(BhelloWorldString.valueOf()).toBe("HelloWorld")
})
