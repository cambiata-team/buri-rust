import {
    BnegativeOne,
    BnegativeTwo,
    BonePlusNegativeOne,
    BoneTimesNegative27,
} from "@tests/js/valid/integers/signed.mjs"
import { expect, it } from "bun:test"

it("unary operator followed by an integer 1 is negative 1", () => {
    expect(BnegativeOne.valueOf()).toBe(-1)
})

it("unary operator followed by an integer 2 is negative 2", () => {
    expect(BnegativeTwo.valueOf()).toBe(-2)
})

it("unary operators can be used with addition", () => {
    expect(BonePlusNegativeOne.valueOf()).toBe(0)
})

it("unary operators can be used with multiplication", () => {
    expect(BoneTimesNegative27.valueOf()).toBe(-27)
})
