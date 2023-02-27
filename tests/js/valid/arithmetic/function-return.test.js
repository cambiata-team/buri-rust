import { BsevenPlusThree, BsevenMinusThree, BsevenTimesThree, BsevenDividedThree, BproductOfOnePlusTwoAndThreePlusFour } from "@tests/js/valid/arithmetic/function-return.mjs"
import { expect, it } from "bun:test"

it("the results of two function calls can be added", () => {
    expect(BsevenPlusThree.valueOf()).toBe(10)
})

it("the results of two function calls can be subtracted", () => {
    expect(BsevenMinusThree.valueOf()).toBe(4)
})

it("the results of two function calls can be multiplied", () => {
    expect(BsevenTimesThree.valueOf()).toBe(21)
})

it("the results of two function calls can be divided", () => { 
    expect(BsevenDividedThree.valueOf()).toBe(2)
})

it("arithmetic of function return values treats function calls like parenthesis around the function definitions", () => {
    expect(BproductOfOnePlusTwoAndThreePlusFour.valueOf()).toBe(21)
})
