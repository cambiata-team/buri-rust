import { BisPalindromeSequence } from "@tests/js/valid/simple-programs/is-palindrome-sequence.mjs"
import { expect, it } from "bun:test"
import { tag } from "../helpers"

it("isPalindromeSequence([])", () => {
    expect(BisPalindromeSequence([])).toEqual(tag("true"))
})

it("isPalindromeSequence([1])", () => {
    expect(BisPalindromeSequence([1])).toEqual(tag("true"))
})

it("isPalindromeSequence([1, 1])", () => {
    expect(BisPalindromeSequence([1, 1])).toEqual(tag("true"))
})

it("isPalindromeSequence([1, 2])", () => {
    expect(BisPalindromeSequence([1, 2])).toEqual(tag("false"))
})

it("isPalindromeSequence([2, 1])", () => {
    expect(BisPalindromeSequence([2, 1])).toEqual(tag("false"))
})

it("isPalindromeSequence([1, 1, 1])", () => {
    expect(BisPalindromeSequence([1, 1, 1])).toEqual(tag("true"))
})

it("isPalindromeSequence([1, 1, 2])", () => {
    expect(BisPalindromeSequence([1, 1, 2])).toEqual(tag("false"))
})

it("isPalindromeSequence([1, 2, 1])", () => {
    expect(BisPalindromeSequence([1, 2, 1])).toEqual(tag("true"))
})

it("isPalindromeSequence([2, 1, 1])", () => {
    expect(BisPalindromeSequence([2, 1, 1])).toEqual(tag("false"))
})

it("isPalindromeSequence([1, 2, 1, 2])", () => {
    expect(BisPalindromeSequence([1, 2, 1, 2])).toEqual(tag("false"))
})

it("isPalindromeSequence([2, 1, 2, 1])", () => {
    expect(BisPalindromeSequence([2, 1, 2, 1])).toEqual(tag("false"))
})

it("isPalindromeSequence([1, 2, 2, 1])", () => {
    expect(BisPalindromeSequence([1, 2, 2, 1])).toEqual(tag("true"))
})

it("isPalindromeSequence([2, 1, 1, 2])", () => {
    expect(BisPalindromeSequence([2, 1, 1, 2])).toEqual(tag("true"))
})

it("isPalindromeSequence([1, 2, 3, 4, 5, 4, 3, 2, 1])", () => {
    expect(BisPalindromeSequence([1, 2, 3, 4, 5, 4, 3, 2, 1])).toEqual(tag("true"))
})

it("isPalindromeSequence([1, 2, 3, 4, 5, 4, 4, 2, 1])", () => {
    expect(BisPalindromeSequence([1, 2, 3, 4, 5, 4, 4, 2, 1])).toEqual(tag("false"))
})
