import { BisPrime } from "@tests/js/valid/simple_programs/is_prime.mjs"
import { expect, it } from "bun:test"
import { tag } from "../helpers"

it("isPrime(0)", () => {
    expect(BisPrime(0)).toEqual(tag("false"))
})

it("isPrime(1)", () => {
    expect(BisPrime(1)).toEqual(tag("false"))
})

it("isPrime(2)", () => {
    expect(BisPrime(2)).toEqual(tag("true"))
})

it("isPrime(3)", () => {
    expect(BisPrime(3)).toEqual(tag("true"))
})

it("isPrime(4)", () => {
    expect(BisPrime(4)).toEqual(tag("false"))
})

it("isPrime(5)", () => {
    expect(BisPrime(5)).toEqual(tag("true"))
})

it("isPrime(6)", () => {
    expect(BisPrime(6)).toEqual(tag("false"))
})

it("isPrime(7)", () => {
    expect(BisPrime(7)).toEqual(tag("true"))
})

it("isPrime(8)", () => {
    expect(BisPrime(8)).toEqual(tag("false"))
})

it("isPrime(9)", () => {
    expect(BisPrime(9)).toEqual(tag("false"))
})

it("isPrime(10)", () => {
    expect(BisPrime(10)).toEqual(tag("false"))
})

it("isPrime(11)", () => {
    expect(BisPrime(11)).toEqual(tag("true"))
})

it("isPrime(12)", () => {
    expect(BisPrime(12)).toEqual(tag("false"))
})
