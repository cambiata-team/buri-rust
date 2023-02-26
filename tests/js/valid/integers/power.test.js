import {
    BfivePowerEight,
    BonePowerTwenty,
    BtwentyPowerOne,
    BtwentyPowerZero,
    BtwoPowerThree,
    BzeroPowerTwenty,
    BzeroPowerZero,
} from "@tests/js/valid/integers/power.mjs"
import { expect, it } from "bun:test"

it("5 ** 8 == 390,625", () => {
    expect(BfivePowerEight.valueOf()).toBe(390_625)
})

it("2 ** 3 == 8", () => {
    expect(BtwoPowerThree.valueOf()).toBe(8)
})

it("20 ** 0 == 1", () => {
    expect(BtwentyPowerZero.valueOf()).toBe(1)
})

it("20 ** 1 == 20", () => {
    expect(BtwentyPowerOne.valueOf()).toBe(20)
})

it("0 ** 20 == 0", () => {
    expect(BzeroPowerTwenty.valueOf()).toBe(0)
})

it("0 ** 0 == 1", () => {
    expect(BzeroPowerZero.valueOf()).toBe(1)
})

it("1 ** 20 == 1", () => {
    expect(BonePowerTwenty.valueOf()).toBe(1)
})
