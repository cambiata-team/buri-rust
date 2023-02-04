import {
    eightyFiveModEightMinusTwo,
    eightyFiveModEightPlusTwo,
    eightyFiveModLParenEightMinusTwoRParen,
    eightyFiveModLParenEightPlusTwoRParen,
    lParenEightyFiveModEightRParenMinusTwo,
    lParenEightyFiveModEightRParenPlusTwo,
    lParenTwelveDividedThreeRParenMinusOne,
    lParenTwelveDividedThreeRParenPlusOne,
    lParenTwoPowerThreeRParenMinusOne,
    lParenTwoPowerThreeRParenPlusOne,
    lParenTwoTimesThreeRParenMinusOne,
    lParenTwoTimesThreeRParenPlusOne,
    oneMinusTwelveDividedThree,
    oneMinusTwoPowerThree,
    oneMinusTwoTimesThree,
    onePlusLParenTwoTimesThreeRParenPowerLParenSevenMinusFiveRParen,
    onePlusTwelveDividedThree,
    onePlusTwoPowerThree,
    onePlusTwoTimesThree,
    onePlusTwoTimesThreePowerSevenMinusFive,
    twelveDividedLParenThreeMinusOneRParen,
    twelveDividedLParenThreePlusOneRParen,
    twelveDividedThreeMinusOne,
    twelveDividedThreePlusOne,
    twoMinusEightyFiveModEight,
    twoPlusEightyFiveModEight,
    twoPowerLParenThreeMinusOneRParen,
    twoPowerLParenThreePlusOneRParen,
    twoPowerThreeMinusOne,
    twoPowerThreePlusOne,
    twoTimesLParenThreeMinusOneRParen,
    twoTimesLParenThreePlusOneRParen,
    twoTimesThreeMinusOne,
    twoTimesThreePlusOne,
} from "@tests/js/valid/integers/order-of-operations.mjs"
import { describe, expect, it } from "bun:test"

describe("raw expressions", () => {
    it("2 * 3 + 1", () => {
        expect(twoTimesThreePlusOne.valueOf()).toBe(7)
    })

    it("1 + 2 * 3", () => {
        expect(onePlusTwoTimesThree.valueOf()).toBe(7)
    })

    it("12 / 3 + 1", () => {
        expect(twelveDividedThreePlusOne.valueOf()).toBe(5)
    })

    it("1 + 12 / 3", () => {
        expect(onePlusTwelveDividedThree.valueOf()).toBe(5)
    })

    it("85 % 8 + 2", () => {
        expect(eightyFiveModEightPlusTwo.valueOf()).toBe(7)
    })

    it("2 + 85 % 8", () => {
        expect(twoPlusEightyFiveModEight.valueOf()).toBe(7)
    })

    it("2 ** 3 + 1", () => {
        expect(twoPowerThreePlusOne.valueOf()).toBe(9)
    })

    it("1 + 2 ** 3", () => {
        expect(onePlusTwoPowerThree.valueOf()).toBe(9)
    })

    it("2 * 3 - 1", () => {
        expect(twoTimesThreeMinusOne.valueOf()).toBe(5)
    })

    it("1 - 2 * 3", () => {
        expect(oneMinusTwoTimesThree.valueOf()).toBe(-5)
    })

    it("12 / 3 - 1", () => {
        expect(twelveDividedThreeMinusOne.valueOf()).toBe(3)
    })

    it("1 - 12 / 3", () => {
        expect(oneMinusTwelveDividedThree.valueOf()).toBe(-3)
    })

    it("85 % 8 - 2", () => {
        expect(eightyFiveModEightMinusTwo.valueOf()).toBe(3)
    })

    it("2 - 85 % 8", () => {
        expect(twoMinusEightyFiveModEight.valueOf()).toBe(-3)
    })

    it("2 ** 3 - 1", () => {
        expect(twoPowerThreeMinusOne.valueOf()).toBe(7)
    })

    it("1 - 2 ** 3", () => {
        expect(oneMinusTwoPowerThree.valueOf()).toBe(-7)
    })
})

describe("with parenthesis", () => {
    it("(2 * 3) + 1", () => {
        expect(lParenTwoTimesThreeRParenPlusOne.valueOf()).toBe(7)
    })

    it("2 * (3 + 1)", () => {
        expect(twoTimesLParenThreePlusOneRParen.valueOf()).toBe(8)
    })

    it("(12 / 3) + 1", () => {
        expect(lParenTwelveDividedThreeRParenPlusOne.valueOf()).toBe(5)
    })

    it("12 / (3 + 1)", () => {
        expect(twelveDividedLParenThreePlusOneRParen.valueOf()).toBe(3)
    })

    it("(85 % 8) + 2", () => {
        expect(lParenEightyFiveModEightRParenPlusTwo.valueOf()).toBe(7)
    })

    it("85 % (8 + 2)", () => {
        expect(eightyFiveModLParenEightPlusTwoRParen.valueOf()).toBe(5)
    })

    it("(2 ** 3) + 1", () => {
        expect(lParenTwoPowerThreeRParenPlusOne.valueOf()).toBe(9)
    })

    it("2 ** (3 + 1)", () => {
        expect(twoPowerLParenThreePlusOneRParen.valueOf()).toBe(16)
    })

    it("(2 * 3) - 1", () => {
        expect(lParenTwoTimesThreeRParenMinusOne.valueOf()).toBe(5)
    })

    it("2 * (3 - 1)", () => {
        expect(twoTimesLParenThreeMinusOneRParen.valueOf()).toBe(4)
    })

    it("(12 / 3) - 1", () => {
        expect(lParenTwelveDividedThreeRParenMinusOne.valueOf()).toBe(3)
    })

    it("12 / (3 - 1)", () => {
        expect(twelveDividedLParenThreeMinusOneRParen.valueOf()).toBe(6)
    })

    it("(85 % 8) - 2", () => {
        expect(lParenEightyFiveModEightRParenMinusTwo.valueOf()).toBe(3)
    })

    it("85 % (8 - 2)", () => {
        expect(eightyFiveModLParenEightMinusTwoRParen.valueOf()).toBe(1)
    })

    it("(2 ** 3) - 1", () => {
        expect(lParenTwoPowerThreeRParenMinusOne.valueOf()).toBe(7)
    })

    it("2 ** (3 - 1)", () => {
        expect(twoPowerLParenThreeMinusOneRParen.valueOf()).toBe(4)
    })
})

describe("complex expressions", () => {
    it("1 + 2 * 3 ** 7 - 5", () => {
        expect(onePlusTwoTimesThreePowerSevenMinusFive.valueOf()).toBe(4370)
    })

    it("1 + (2 * 3) ** (7 - 5)", () => {
        expect(
            onePlusLParenTwoTimesThreeRParenPowerLParenSevenMinusFiveRParen.valueOf()
        ).toBe(37)
    })
})
