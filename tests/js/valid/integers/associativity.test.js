import {
    BeightDivideLParenThreeDivideTwoRParen,
    BeightDivideThreeDivideTwo,
    BeightModFiveModTwo,
    BeightModLParenFiveModTwoRParen,
    BlParenEightDivideThreeRParenDivideTwo,
    BlParenEightModFiveRParenModTwo,
    BlParenOnePlusTwoRParenPlusThree,
    BlParenOneTimesTwoRParenTimesThree,
    BlParenTenMinusFiveRParenMinusTwo,
    BlParenTwoPowerThreeRParenPowerTwo,
    BonePlusLParenTwoPlusThreeRParen,
    BonePlusTwoPlusThree,
    BoneTimesLParenTwoTimesThreeRParen,
    BoneTimesTwoTimesThree,
    BtenMinusFiveMinusTwo,
    BtenMinusLParenFiveMinusTwoRParen,
    BtwoPowerLParenThreePowerTwoRParen,
    BtwoPowerThreePowerTwo,
} from "@tests/js/valid/integers/associativity.mjs"
import { describe, expect, it } from "bun:test"

describe("associative operators", () => {
    describe("addition", () => {
        it("1 + (2 + 3) == (1 + 2) + 3", () => {
            expect(BonePlusLParenTwoPlusThreeRParen.valueOf()).toBe(
                BlParenOnePlusTwoRParenPlusThree.valueOf()
            )
        })

        it("addition is left associative by default: 1 + 2 + 3 == (1 + 2) + 3", () => {
            expect(BonePlusTwoPlusThree.valueOf()).toBe(
                BlParenOnePlusTwoRParenPlusThree.valueOf()
            )
        })
    })

    describe("multiplication", () => {
        it("1 * (2 * 3) == (1 * 2) * 3", () => {
            expect(BoneTimesLParenTwoTimesThreeRParen.valueOf()).toBe(
                BlParenOneTimesTwoRParenTimesThree.valueOf()
            )
        })

        it("multiplication is left associative by default: 1 * 2 * 3 == (1 * 2) * 3", () => {
            expect(BoneTimesTwoTimesThree.valueOf()).toBe(
                BlParenOneTimesTwoRParenTimesThree.valueOf()
            )
        })
    })
})

describe("non-associative operators", () => {
    describe("subtraction", () => {
        it("10 - (5 - 2) != (10 - 5) - 2", () => {
            expect(BtenMinusLParenFiveMinusTwoRParen.valueOf()).not.toBe(
                BlParenTenMinusFiveRParenMinusTwo.valueOf()
            )
        })

        it("subtraction is left associative by default: 10 - 5 - 2 == (10 - 5) - 2", () => {
            expect(BtenMinusFiveMinusTwo.valueOf()).toBe(
                BlParenTenMinusFiveRParenMinusTwo.valueOf()
            )
        })
    })

    describe("division", () => {
        it("8 / (3 / 2) != (8 / 3) / 2", () => {
            expect(BeightDivideLParenThreeDivideTwoRParen.valueOf()).not.toBe(
                BlParenEightDivideThreeRParenDivideTwo.valueOf()
            )
        })

        it("division is left associative by default: 8 / 3 / 2 == (8 / 3) / 2", () => {
            expect(BeightDivideThreeDivideTwo.valueOf()).toBe(
                BlParenEightDivideThreeRParenDivideTwo.valueOf()
            )
        })
    })

    describe("modulo", () => {
        it("8 % (5 % 2) != (8 % 5) % 2", () => {
            expect(BeightModLParenFiveModTwoRParen.valueOf()).not.toBe(
                BlParenEightModFiveRParenModTwo.valueOf()
            )
        })

        it("modulo is left associative by default: 8 % 5 % 2 == (8 % 5) % 2", () => {
            expect(BeightModFiveModTwo.valueOf()).toBe(
                BlParenEightModFiveRParenModTwo.valueOf()
            )
        })
    })

    describe("power", () => {
        it("2 ** (3 ** 2) != (2 ** 3) ** 2", () => {
            expect(BtwoPowerLParenThreePowerTwoRParen.valueOf()).not.toBe(
                BlParenTwoPowerThreeRParenPowerTwo.valueOf()
            )
        })

        it("power is right associative by default: 2 ** 3 ** 2 == 2 ** (3 ** 2)", () => {
            expect(BtwoPowerThreePowerTwo.valueOf()).toBe(
                BtwoPowerLParenThreePowerTwoRParen.valueOf()
            )
        })
    })
})
