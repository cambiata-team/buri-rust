import { fiveMinusTwo, one, onePlusTwo, sevenModThree, sixDividedByTwo, two, twoTimesThree, twoToThePowerOfThree } from "@tests/js/valid/integers/unsigned.mjs"

import { expect, it } from "bun:test"

it("a literal with the value of 1 should be equal to 1", () => {
    expect(one.valueOf()).toBe(1)
})

it("a literal with the value of 2 should be equal to 2", () => {
    expect(two.valueOf()).toBe(2)
})

it("literal one plus literal two should equal three", () => {
    expect(onePlusTwo.valueOf()).toBe(3)
})

it("literal five minus literal two should equal three", () => {
    expect(fiveMinusTwo.valueOf()).toBe(3)
})

it("literal two times literal three should equal six", () => {
    expect(twoTimesThree.valueOf()).toBe(6)
})

it("literal six divided by literal two should equal three", () => {
    expect(sixDividedByTwo.valueOf()).toBe(3)
})

it("literal seven mod literal three should equal one", () => {
    expect(sevenModThree.valueOf()).toBe(1)
})

it("literal two to the power of literal three should equal eight", () => {
    expect(twoToThePowerOfThree.valueOf()).toBe(8)
})