import { helloString, helloWorldString, emptyString, newlineString, piString } from "@tests/js/valid/strings/string.mjs"

import { expect, it } from "bun:test"

it("empty string literal should have the value of an empty string", () => {
    expect(emptyString.valueOf()).toBe("")
})

it("hello string literal should have the value of hello", () => {
    expect(helloString.valueOf()).toBe("hello")
})

it("newline string literal should have the value of newline", () => {
    expect(newlineString.valueOf()).toBe("\n")
})

it("lowercase pi string literal should have the value of lowercase letter pi", () => {
    expect(piString.valueOf()).toBe("π")
})

it("hello concatenated with world should have the value of hello world", () => {
    expect(helloWorldString.valueOf()).toBe("HelloWorld")
})
