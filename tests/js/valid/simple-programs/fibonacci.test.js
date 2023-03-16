import { Bfib } from "@tests/js/valid/simple-programs/fibonacci.mjs"
import { expect, it } from "bun:test"

it("fib(0)", () => {
    expect(Bfib(0)).toBe(1)
})

it("fib(1)", () => {
    expect(Bfib(1)).toBe(1)
})

it("fib(2)", () => {
    expect(Bfib(2)).toBe(2)
})

it("fib(3)", () => {
    expect(Bfib(3)).toBe(3)
})

it("fib(4)", () => {
    expect(Bfib(4)).toBe(5)
})

it("fib(5)", () => {
    expect(Bfib(5)).toBe(8)
})
