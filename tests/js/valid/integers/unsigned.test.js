import { BfiveMinusTwo, Bone, BonePlusTwo, BsevenModThree, BsixDividedByTwo, Btwo, BtwoTimesThree, BtwoToThePowerOfThree } from "@tests/js/valid/integers/unsigned.mjs"

import { expect, it } from "bun:test"

it("a literal with the value of 1 should be equal to 1", () => {
    expect(Bone.valueOf()).toBe(1)
})

it("a literal with the value of 2 should be equal to 2", () => {
    expect(Btwo.valueOf()).toBe(2)
})

it("literal one plus literal two should equal three", () => {
    expect(BonePlusTwo.valueOf()).toBe(3)
})

it("literal five minus literal two should equal three", () => {
    expect(BfiveMinusTwo.valueOf()).toBe(3)
})

it("literal two times literal three should equal six", () => {
    expect(BtwoTimesThree.valueOf()).toBe(6)
})

it("literal six divided by literal two should equal three", () => {
    expect(BsixDividedByTwo.valueOf()).toBe(3)
})

it("literal seven mod literal three should equal one", () => {
    expect(BsevenModThree.valueOf()).toBe(1)
})

it("literal two to the power of literal three should equal eight", () => {
    expect(BtwoToThePowerOfThree.valueOf()).toBe(8)
})