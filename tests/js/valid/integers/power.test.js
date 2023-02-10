import {
    fivePowerEight,
    onePowerTwenty,
    twentyPowerOne,
    twentyPowerZero,
    twoPowerThree,
    zeroPowerTwenty,
    zeroPowerZero,
} from "@tests/js/valid/integers/power.mjs"
import { expect, it } from "bun:test"

it("5 ** 8 == 390,625", () => {
    expect(fivePowerEight.valueOf()).toBe(390_625)
})

it("2 ** 3 == 8", () => {
    expect(twoPowerThree.valueOf()).toBe(8)
})

it("20 ** 0 == 1", () => {
    expect(twentyPowerZero.valueOf()).toBe(1)
})

it("20 ** 1 == 20", () => {
    expect(twentyPowerOne.valueOf()).toBe(20)
})

it("0 ** 20 == 0", () => {
    expect(zeroPowerTwenty.valueOf()).toBe(0)
})

it("0 ** 0 == 1", () => {
    expect(zeroPowerZero.valueOf()).toBe(1)
})

it("1 ** 20 == 1", () => {
    expect(onePowerTwenty.valueOf()).toBe(1)
})
