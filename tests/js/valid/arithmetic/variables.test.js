import { BsevenPlusThree, BsevenMinusThree, BsevenTimesThree, BsevenDividedThree, BdifferenceOfSevenTimesThreeAndSevenPlusThree } from "@tests/js/valid/arithmetic/variables.mjs"
import { expect, it } from "bun:test"

it("two variables can be added", () => {
    expect(BsevenPlusThree.valueOf()).toBe(10)
})

it("two variables can be subtracted", () => {
    expect(BsevenMinusThree.valueOf()).toBe(4)
})

it("two variables can be multiplied", () => {
    expect(BsevenTimesThree.valueOf()).toBe(21)
})

it("two variables can be divided", () => { 
    expect(BsevenDividedThree.valueOf()).toBe(2)
})

it("variables assigned from the results of arithmetic operations can be used in other arithmetic operations", () => {
    expect(BdifferenceOfSevenTimesThreeAndSevenPlusThree.valueOf()).toBe(11)
})
