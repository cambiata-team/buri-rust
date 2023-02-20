import {
    five,
    multilineFive,
    multilineThree,
    three,
} from "@tests/js/valid/if/basic.mjs"
import { expect, it } from "bun:test"

it("should evaluate the true branch", () => {
    expect(five).toBe(5)
})

it("should evaluate the false branch", () => {
    expect(three).toBe(3)
})

it("should evaluate the true branch of a multiline if", () => {
    expect(multilineFive).toBe(5)
})

it("should evaluate the false branch of a multiline if", () => {
    expect(multilineThree).toBe(3)
})
