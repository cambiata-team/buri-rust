import {
    BzeroEqualsOne,
    BoneEqualsZero,
    BzeroNotEqualsOne,
    BoneNotEqualsZero,
    BzeroGreaterThanOne,
    BoneGreaterThanOne,
    BtwoGreaterThanOne,
    BoneLessThanZero,
    BoneLessThanOne,
    BoneLessThanTwo,
    BzeroGreaterThanOrEqualToOne,
    BoneGreaterThanOrEqualToOne,
    BtwoGreaterThanOrEqualToOne,
    BoneLessThanOrEqualToZero,
    BoneLessThanOrEqualToOne,
    BoneLessThanOrEqualToTwo,
    BfourGreaterThanOrEqualToFive,
    BfiveGreaterThanOrEqualToFive,
    BsixGreaterThanOrEqualToFive,
    BfiveNotGreaterThanFour,
    BfiveNotGreaterThanFive,
    BfiveNotGreaterThanSix,
    BfourLessThanOrEqualToFive,
    BfiveLessThanOrEqualToFive,
    BsixLessThanOrEqualToFive,
    BfiveNotLessThanFour,
    BfiveNotLessThanFive,
    BfiveNotLessThanSix,
} from "@tests/js/valid/comparison-operators/commutativity.mjs"
import { describe, expect, it } from "bun:test"

describe("==", () => {
    it("0 == 1 <--> 1 == 0", () => {
        expect(BzeroEqualsOne).toEqual(BoneEqualsZero)
    })
})

describe("!=", () => {
    it("0 != 1 <--> 1 != 0", () => {
        expect(BzeroNotEqualsOne).toEqual(BoneNotEqualsZero)
    })
})

describe("> and <", () => {
    it("0 > 1 <--> 1 < 0", () => {
        expect(BzeroGreaterThanOne).toEqual(BoneLessThanZero)
    })
    it("1 > 1 <--> 1 < 1", () => {
        expect(BoneGreaterThanOne).toEqual(BoneLessThanOne)
    })
    it("2 > 1 <--> 1 < 2", () => {
        expect(BtwoGreaterThanOne).toEqual(BoneLessThanTwo)
    })
})

describe(">= and <=", () => {
    it("0 >= 1 <--> 1 <= 0", () => {
        expect(BzeroGreaterThanOrEqualToOne).toEqual(BoneLessThanOrEqualToZero)
    })
    it("1 >= 1 <--> 1 <= 1", () => {
        expect(BoneGreaterThanOrEqualToOne).toEqual(BoneLessThanOrEqualToOne)
    })
    it("2 >= 1 <--> 1 <= 2", () => {
        expect(BtwoGreaterThanOrEqualToOne).toEqual(BoneLessThanOrEqualToTwo)
    })
})

describe(">= and >", () => {
    it("4 >= 5 <--> not (5 > 4)", () => {
        expect(BfourGreaterThanOrEqualToFive).toEqual(BfiveNotGreaterThanFour)
    })
    it("5 >= 5 <--> not (5 > 5)", () => {
        expect(BfiveGreaterThanOrEqualToFive).toEqual(BfiveNotGreaterThanFive)
    })
    it("6 >= 5 <--> not (5 > 6)", () => {
        expect(BsixGreaterThanOrEqualToFive).toEqual(BfiveNotGreaterThanSix)
    })
})

describe("<= and <", () => {
    it("4 <= 5 <--> not (5 < 4)", () => {
        expect(BfourLessThanOrEqualToFive).toEqual(BfiveNotLessThanFour)
    })
    it("5 <= 5 <--> not (5 < 5)", () => {
        expect(BfiveLessThanOrEqualToFive).toEqual(BfiveNotLessThanFive)
    })
    it("6 <= 5 <--> not (5 < 6)", () => {
        expect(BsixLessThanOrEqualToFive).toEqual(BfiveNotLessThanSix)
    })
})
