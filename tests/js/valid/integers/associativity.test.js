import {
    eightDivideLParenThreeDivideTwoRParen,
    eightDivideThreeDivideTwo,
    eightModFiveModTwo,
    eightModLParenFiveModTwoRParen,
    lParenEightDivideThreeRParenDivideTwo,
    lParenEightModFiveRParenModTwo,
    lParenOnePlusTwoRParenPlusThree,
    lParenOneTimesTwoRParenTimesThree,
    lParenTenMinusFiveRParenMinusTwo,
    lParenTwoPowerThreeRParenPowerTwo,
    onePlusLParenTwoPlusThreeRParen,
    onePlusTwoPlusThree,
    oneTimesLParenTwoTimesThreeRParen,
    oneTimesTwoTimesThree,
    tenMinusFiveMinusTwo,
    tenMinusLParenFiveMinusTwoRParen,
    twoPowerLParenThreePowerTwoRParen,
    twoPowerThreePowerTwo,
} from "@tests/js/valid/integers/associativity.mjs"
import { describe, expect, it } from "bun:test"

describe("associative operators", () => {
    describe("addition", () => {
        it("1 + (2 + 3) == (1 + 2) + 3", () => {
            expect(onePlusLParenTwoPlusThreeRParen.valueOf()).toBe(
                lParenOnePlusTwoRParenPlusThree.valueOf()
            )
        })

        it("addition is left associative by default: 1 + 2 + 3 == (1 + 2) + 3", () => {
            expect(onePlusTwoPlusThree.valueOf()).toBe(
                lParenOnePlusTwoRParenPlusThree.valueOf()
            )
        })
    })

    describe("multiplication", () => {
        it("1 * (2 * 3) == (1 * 2) * 3", () => {
            expect(oneTimesLParenTwoTimesThreeRParen.valueOf()).toBe(
                lParenOneTimesTwoRParenTimesThree.valueOf()
            )
        })

        it("multiplication is left associative by default: 1 * 2 * 3 == (1 * 2) * 3", () => {
            expect(oneTimesTwoTimesThree.valueOf()).toBe(
                lParenOneTimesTwoRParenTimesThree.valueOf()
            )
        })
    })
})

describe("non-associative operators", () => {
    describe("subtraction", () => {
        it("10 - (5 - 2) != (10 - 5) - 2", () => {
            expect(tenMinusLParenFiveMinusTwoRParen.valueOf()).not.toBe(
                lParenTenMinusFiveRParenMinusTwo.valueOf()
            )
        })

        it("subtraction is left associative by default: 10 - 5 - 2 == (10 - 5) - 2", () => {
            expect(tenMinusFiveMinusTwo.valueOf()).toBe(
                lParenTenMinusFiveRParenMinusTwo.valueOf()
            )
        })
    })

    describe("division", () => {
        it("8 / (3 / 2) != (8 / 3) / 2", () => {
            expect(eightDivideLParenThreeDivideTwoRParen.valueOf()).not.toBe(
                lParenEightDivideThreeRParenDivideTwo.valueOf()
            )
        })

        it("division is left associative by default: 8 / 3 / 2 == (8 / 3) / 2", () => {
            expect(eightDivideThreeDivideTwo.valueOf()).toBe(
                lParenEightDivideThreeRParenDivideTwo.valueOf()
            )
        })
    })

    describe("modulo", () => {
        it("8 % (5 % 2) != (8 % 5) % 2", () => {
            expect(eightModLParenFiveModTwoRParen.valueOf()).not.toBe(
                lParenEightModFiveRParenModTwo.valueOf()
            )
        })

        it("modulo is left associative by default: 8 % 5 % 2 == (8 % 5) % 2", () => {
            expect(eightModFiveModTwo.valueOf()).toBe(
                lParenEightModFiveRParenModTwo.valueOf()
            )
        })
    })

    describe("power", () => {
        it("2 ** (3 ** 2) != (2 ** 3) ** 2", () => {
            expect(twoPowerLParenThreePowerTwoRParen.valueOf()).not.toBe(
                lParenTwoPowerThreeRParenPowerTwo.valueOf()
            )
        })

        it("power is right associative by default: 2 ** 3 ** 2 == 2 ** (3 ** 2)", () => {
            expect(twoPowerThreePowerTwo.valueOf()).toBe(
                twoPowerLParenThreePowerTwoRParen.valueOf()
            )
        })
    })
})
