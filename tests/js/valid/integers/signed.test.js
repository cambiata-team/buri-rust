import {
    negativeOne,
    negativeTwo,
    onePlusNegativeOne,
    oneTimesNegative27,
} from "@tests/js/valid/integers/signed.mjs"
import { expect, it } from "bun:test"

it("unary operator followed by an integer 1 is negative 1", () => {
    expect(negativeOne.valueOf()).toBe(-1)
})

it("unary operator followed by an integer 2 is negative 2", () => {
    expect(negativeTwo.valueOf()).toBe(-2)
})

it("unary operators can be used with addition", () => {
    expect(onePlusNegativeOne.valueOf()).toBe(0)
})

it("unary operators can be used with multiplication", () => {
    expect(oneTimesNegative27.valueOf()).toBe(-27)
})
