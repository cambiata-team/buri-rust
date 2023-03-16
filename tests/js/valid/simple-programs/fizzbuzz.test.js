import { Bfizzbuzz } from "@tests/js/valid/simple-programs/fizzbuzz.mjs"
import { expect, it } from "bun:test"

it("fizzbuzz(0)", () => {
    expect(Bfizzbuzz(0)).toBe("fizzbuzz")
})

it("fizzbuzz(1)", () => {
    expect(Bfizzbuzz(1)).toBe("")
})

it("fizzbuzz(2)", () => {
    expect(Bfizzbuzz(2)).toBe("")
})

it("fizzbuzz(3)", () => {
    expect(Bfizzbuzz(3)).toBe("fizz")
})

it("fizzbuzz(4)", () => {
    expect(Bfizzbuzz(4)).toBe("")
})

it("fizzbuzz(5)", () => {
    expect(Bfizzbuzz(5)).toBe("buzz")
})

it("fizzbuzz(6)", () => {
    expect(Bfizzbuzz(6)).toBe("fizz")
})

it("fizzbuzz(7)", () => {
    expect(Bfizzbuzz(7)).toBe("")
})

it("fizzbuzz(8)", () => {
    expect(Bfizzbuzz(8)).toBe("")
})

it("fizzbuzz(9)", () => {
    expect(Bfizzbuzz(9)).toBe("fizz")
})

it("fizzbuzz(10)", () => {
    expect(Bfizzbuzz(10)).toBe("buzz")
})

it("fizzbuzz(11)", () => {
    expect(Bfizzbuzz(11)).toBe("")
})

it("fizzbuzz(12)", () => {
    expect(Bfizzbuzz(12)).toBe("fizz")
})

it("fizzbuzz(13)", () => {
    expect(Bfizzbuzz(13)).toBe("")
})

it("fizzbuzz(14)", () => {
    expect(Bfizzbuzz(14)).toBe("")
})

it("fizzbuzz(15)", () => {
    expect(Bfizzbuzz(15)).toBe("fizzbuzz")
})
