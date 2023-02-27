import {
    BnegativeOneEqualsNegativeOne,
    BnegativeOneEqualsNegativeTwo,
    BnegativeOneEqualsOne,
    BnegativeOneEqualsTwo,
    BnegativeOneGreaterThanEqualsNegativeOne,
    BnegativeOneGreaterThanEqualsNegativeTwo,
    BnegativeOneGreaterThanEqualsOne,
    BnegativeOneGreaterThanEqualsTwo,
    BnegativeOneGreaterThanNegativeOne,
    BnegativeOneGreaterThanNegativeTwo,
    BnegativeOneGreaterThanOne,
    BnegativeOneGreaterThanTwo,
    BnegativeOneLessThanEqualsNegativeOne,
    BnegativeOneLessThanEqualsNegativeTwo,
    BnegativeOneLessThanEqualsOne,
    BnegativeOneLessThanEqualsTwo,
    BnegativeOneLessThanNegativeOne,
    BnegativeOneLessThanNegativeTwo,
    BnegativeOneLessThanOne,
    BnegativeOneLessThanTwo,
    BnegativeOneNotEqualsNegativeOne,
    BnegativeOneNotEqualsNegativeTwo,
    BnegativeOneNotEqualsOne,
    BnegativeOneNotEqualsTwo,
    BoneEqualsOne,
    BoneEqualsTwo,
    BoneGreaterThanEqualsOne,
    BoneGreaterThanEqualsTwo,
    BoneGreaterThanOne,
    BoneGreaterThanTwo,
    BoneLessThanEqualsOne,
    BoneLessThanEqualsTwo,
    BoneLessThanOne,
    BoneLessThanTwo,
    BoneNotEqualsOne,
    BoneNotEqualsTwo,
    BtwoGreaterThanEqualsOne,
    BtwoGreaterThanOne,
    BtwoLessThanEqualsOne,
    BtwoLessThanOne,
} from "@tests/js/valid/comparison-operators/function-return.mjs"
import { describe, expect, it } from "bun:test"
import { tag } from "../helpers"

describe("==", () => {
    it("1 == 1 -- #true", () => {
        expect(BoneEqualsOne).toEqual(tag("true"))
    })

    it("1 == 2 -- #false", () => {
        expect(BoneEqualsTwo).toEqual(tag("false"))
    })

    it("-1 == -1 -- #true", () => {
        expect(BnegativeOneEqualsNegativeOne).toEqual(tag("true"))
    })

    it("-1 == 1 -- #false", () => {
        expect(BnegativeOneEqualsOne).toEqual(tag("false"))
    })

    it("-1 == -2 -- #false", () => {
        expect(BnegativeOneEqualsNegativeTwo).toEqual(tag("false"))
    })

    it("-1 == 2 -- #false", () => {
        expect(BnegativeOneEqualsTwo).toEqual(tag("false"))
    })
})

describe("!=", () => {
    it("1 != 1 -- #false", () => {
        expect(BoneNotEqualsOne).toEqual(tag("false"))
    })

    it("1 != 2 -- #true", () => {
        expect(BoneNotEqualsTwo).toEqual(tag("true"))
    })

    it("-1 != -1 -- #false", () => {
        expect(BnegativeOneNotEqualsNegativeOne).toEqual(tag("false"))
    })

    it("-1 != 1 -- #true", () => {
        expect(BnegativeOneNotEqualsOne).toEqual(tag("true"))
    })

    it("-1 != -2 -- #true", () => {
        expect(BnegativeOneNotEqualsNegativeTwo).toEqual(tag("true"))
    })

    it("-1 != 2 -- #true", () => {
        expect(BnegativeOneNotEqualsTwo).toEqual(tag("true"))
    })
})

describe("<", () => {
    it("1 < 1 -- #false", () => {
        expect(BoneLessThanOne).toEqual(tag("false"))
    })

    it("1 < 2 -- #true", () => {
        expect(BoneLessThanTwo).toEqual(tag("true"))
    })

    it("2 < 1 -- #false", () => {
        expect(BtwoLessThanOne).toEqual(tag("false"))
    })

    it("-1 < -1 -- #false", () => {
        expect(BnegativeOneLessThanNegativeOne).toEqual(tag("false"))
    })

    it("-1 < 1 -- #true", () => {
        expect(BnegativeOneLessThanOne).toEqual(tag("true"))
    })

    it("-1 < -2 -- #false", () => {
        expect(BnegativeOneLessThanNegativeTwo).toEqual(tag("false"))
    })

    it("-1 < 2 -- #true", () => {
        expect(BnegativeOneLessThanTwo).toEqual(tag("true"))
    })
})

describe("<=", () => {
    it("1 <= 1 -- #true", () => {
        expect(BoneLessThanEqualsOne).toEqual(tag("true"))
    })

    it("1 <= 2 -- #true", () => {
        expect(BoneLessThanEqualsTwo).toEqual(tag("true"))
    })

    it("2 <= 1 -- #false", () => {
        expect(BtwoLessThanEqualsOne).toEqual(tag("false"))
    })

    it("-1 <= -1 -- #true", () => {
        expect(BnegativeOneLessThanEqualsNegativeOne).toEqual(tag("true"))
    })

    it("-1 <= 1 -- #true", () => {
        expect(BnegativeOneLessThanEqualsOne).toEqual(tag("true"))
    })

    it("-1 <= -2 -- #false", () => {
        expect(BnegativeOneLessThanEqualsNegativeTwo).toEqual(tag("false"))
    })

    it("-1 <= 2 -- #true", () => {
        expect(BnegativeOneLessThanEqualsTwo).toEqual(tag("true"))
    })
})

describe(">", () => {
    it("1 > 1 -- #false", () => {
        expect(BoneGreaterThanOne).toEqual(tag("false"))
    })

    it("1 > 2 -- #false", () => {
        expect(BoneGreaterThanTwo).toEqual(tag("false"))
    })

    it("2 > 1 -- #true", () => {
        expect(BtwoGreaterThanOne).toEqual(tag("true"))
    })

    it("-1 > -1 -- #false", () => {
        expect(BnegativeOneGreaterThanNegativeOne).toEqual(tag("false"))
    })

    it("-1 > 1 -- #false", () => {
        expect(BnegativeOneGreaterThanOne).toEqual(tag("false"))
    })

    it("-1 > -2 -- #true", () => {
        expect(BnegativeOneGreaterThanNegativeTwo).toEqual(tag("true"))
    })

    it("-1 > 2 -- #false", () => {
        expect(BnegativeOneGreaterThanTwo).toEqual(tag("false"))
    })
})

describe(">=", () => {
    it("1 >= 1 -- #true", () => {
        expect(BoneGreaterThanEqualsOne).toEqual(tag("true"))
    })

    it("1 >= 2 -- #false", () => {
        expect(BoneGreaterThanEqualsTwo).toEqual(tag("false"))
    })

    it("2 >= 1 -- #true", () => {
        expect(BtwoGreaterThanEqualsOne).toEqual(tag("true"))
    })

    it("-1 >= -1 -- #true", () => {
        expect(BnegativeOneGreaterThanEqualsNegativeOne).toEqual(tag("true"))
    })

    it("-1 >= 1 -- #false", () => {
        expect(BnegativeOneGreaterThanEqualsOne).toEqual(tag("false"))
    })

    it("-1 >= -2 -- #true", () => {
        expect(BnegativeOneGreaterThanEqualsNegativeTwo).toEqual(tag("true"))
    })

    it("-1 >= 2 -- #false", () => {
        expect(BnegativeOneGreaterThanEqualsTwo).toEqual(tag("false"))
    })
})
