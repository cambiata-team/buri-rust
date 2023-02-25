import {
    falseAndFalse,
    falseAndTrue,
    falseOrFalse,
    falseOrTrue,
    negativeOneEqualsNegativeOne,
    negativeOneEqualsNegativeTwo,
    negativeOneEqualsOne,
    negativeOneEqualsTwo,
    negativeOneGreaterThanEqualsNegativeOne,
    negativeOneGreaterThanEqualsNegativeTwo,
    negativeOneGreaterThanEqualsOne,
    negativeOneGreaterThanEqualsTwo,
    negativeOneGreaterThanNegativeOne,
    negativeOneGreaterThanNegativeTwo,
    negativeOneGreaterThanOne,
    negativeOneGreaterThanTwo,
    negativeOneLessThanEqualsNegativeOne,
    negativeOneLessThanEqualsNegativeTwo,
    negativeOneLessThanEqualsOne,
    negativeOneLessThanEqualsTwo,
    negativeOneLessThanNegativeOne,
    negativeOneLessThanNegativeTwo,
    negativeOneLessThanOne,
    negativeOneLessThanTwo,
    negativeOneNotEqualsNegativeOne,
    negativeOneNotEqualsNegativeTwo,
    negativeOneNotEqualsOne,
    negativeOneNotEqualsTwo,
    notFalse,
    notTrue,
    oneEqualsOne,
    oneEqualsTwo,
    oneGreaterThanEqualsOne,
    oneGreaterThanEqualsTwo,
    oneGreaterThanOne,
    oneGreaterThanTwo,
    oneLessThanEqualsOne,
    oneLessThanEqualsTwo,
    oneLessThanOne,
    oneLessThanTwo,
    oneNotEqualsOne,
    oneNotEqualsTwo,
    trueAndFalse,
    trueAndTrue,
    trueOrFalse,
    trueOrTrue,
    twoGreaterThanEqualsOne,
    twoGreaterThanOne,
    twoLessThanEqualsOne,
    twoLessThanOne,
} from "@tests/js/valid/logical-operators/definitions.mjs"
import { describe, expect, it } from "bun:test"
import { tag } from "../helpers"

describe("and", () => {
    it("#true and #true -- #true", () => {
        expect(trueAndTrue).toEqual(tag("true"))
    })

    it("#true and #false -- #false", () => {
        expect(trueAndFalse).toEqual(tag("false"))
    })

    it("#false and #true -- #false", () => {
        expect(falseAndTrue).toEqual(tag("false"))
    })

    it("#false and #false -- #false", () => {
        expect(falseAndFalse).toEqual(tag("false"))
    })
})

describe("or", () => {
    it("#true or #true -- #true", () => {
        expect(trueOrTrue).toEqual(tag("true"))
    })

    it("#true or #false -- #true", () => {
        expect(trueOrFalse).toEqual(tag("true"))
    })

    it("#false or #true -- #true", () => {
        expect(falseOrTrue).toEqual(tag("true"))
    })

    it("#false or #false -- #false", () => {
        expect(falseOrFalse).toEqual(tag("false"))
    })
})

describe("not", () => {
    it("not #true -- #false", () => {
        expect(notTrue).toEqual(tag("false"))
    })

    it("not #false -- #true", () => {
        expect(notFalse).toEqual(tag("true"))
    })
})

describe("==", () => {
    it("1 == 1 -- #true", () => {
        expect(oneEqualsOne).toEqual(tag("true"))
    })

    it("1 == 2 -- #false", () => {
        expect(oneEqualsTwo).toEqual(tag("false"))
    })

    it("-1 == -1 -- #true", () => {
        expect(negativeOneEqualsNegativeOne).toEqual(tag("true"))
    })

    it("-1 == 1 -- #false", () => {
        expect(negativeOneEqualsOne).toEqual(tag("false"))
    })

    it("-1 == -2 -- #false", () => {
        expect(negativeOneEqualsNegativeTwo).toEqual(tag("false"))
    })

    it("-1 == 2 -- #false", () => {
        expect(negativeOneEqualsTwo).toEqual(tag("false"))
    })
})

describe("!=", () => {
    it("1 != 1 -- #false", () => {
        expect(oneNotEqualsOne).toEqual(tag("false"))
    })

    it("1 != 2 -- #true", () => {
        expect(oneNotEqualsTwo).toEqual(tag("true"))
    })

    it("-1 != -1 -- #false", () => {
        expect(negativeOneNotEqualsNegativeOne).toEqual(tag("false"))
    })

    it("-1 != 1 -- #true", () => {
        expect(negativeOneNotEqualsOne).toEqual(tag("true"))
    })

    it("-1 != -2 -- #true", () => {
        expect(negativeOneNotEqualsNegativeTwo).toEqual(tag("true"))
    })

    it("-1 != 2 -- #true", () => {
        expect(negativeOneNotEqualsTwo).toEqual(tag("true"))
    })
})

describe("<", () => {
    it("1 < 1 -- #false", () => {
        expect(oneLessThanOne).toEqual(tag("false"))
    })

    it("1 < 2 -- #true", () => {
        expect(oneLessThanTwo).toEqual(tag("true"))
    })

    it("2 < 1 -- #false", () => {
        expect(twoLessThanOne).toEqual(tag("false"))
    })

    it("-1 < -1 -- #false", () => {
        expect(negativeOneLessThanNegativeOne).toEqual(tag("false"))
    })

    it("-1 < 1 -- #true", () => {
        expect(negativeOneLessThanOne).toEqual(tag("true"))
    })

    it("-1 < -2 -- #false", () => {
        expect(negativeOneLessThanNegativeTwo).toEqual(tag("false"))
    })

    it("-1 < 2 -- #true", () => {
        expect(negativeOneLessThanTwo).toEqual(tag("true"))
    })
})

describe("<=", () => {
    it("1 <= 1 -- #true", () => {
        expect(oneLessThanEqualsOne).toEqual(tag("true"))
    })

    it("1 <= 2 -- #true", () => {
        expect(oneLessThanEqualsTwo).toEqual(tag("true"))
    })

    it("2 <= 1 -- #false", () => {
        expect(twoLessThanEqualsOne).toEqual(tag("false"))
    })

    it("-1 <= -1 -- #true", () => {
        expect(negativeOneLessThanEqualsNegativeOne).toEqual(tag("true"))
    })

    it("-1 <= 1 -- #true", () => {
        expect(negativeOneLessThanEqualsOne).toEqual(tag("true"))
    })

    it("-1 <= -2 -- #false", () => {
        expect(negativeOneLessThanEqualsNegativeTwo).toEqual(tag("false"))
    })

    it("-1 <= 2 -- #true", () => {
        expect(negativeOneLessThanEqualsTwo).toEqual(tag("true"))
    })
})

describe(">", () => {
    it("1 > 1 -- #false", () => {
        expect(oneGreaterThanOne).toEqual(tag("false"))
    })

    it("1 > 2 -- #false", () => {
        expect(oneGreaterThanTwo).toEqual(tag("false"))
    })

    it("2 > 1 -- #true", () => {
        expect(twoGreaterThanOne).toEqual(tag("true"))
    })

    it("-1 > -1 -- #false", () => {
        expect(negativeOneGreaterThanNegativeOne).toEqual(tag("false"))
    })

    it("-1 > 1 -- #false", () => {
        expect(negativeOneGreaterThanOne).toEqual(tag("false"))
    })

    it("-1 > -2 -- #true", () => {
        expect(negativeOneGreaterThanNegativeTwo).toEqual(tag("true"))
    })

    it("-1 > 2 -- #false", () => {
        expect(negativeOneGreaterThanTwo).toEqual(tag("false"))
    })
})

describe(">=", () => {
    it("1 >= 1 -- #true", () => {
        expect(oneGreaterThanEqualsOne).toEqual(tag("true"))
    })

    it("1 >= 2 -- #false", () => {
        expect(oneGreaterThanEqualsTwo).toEqual(tag("false"))
    })

    it("2 >= 1 -- #true", () => {
        expect(twoGreaterThanEqualsOne).toEqual(tag("true"))
    })

    it("-1 >= -1 -- #true", () => {
        expect(negativeOneGreaterThanEqualsNegativeOne).toEqual(tag("true"))
    })

    it("-1 >= 1 -- #false", () => {
        expect(negativeOneGreaterThanEqualsOne).toEqual(tag("false"))
    })

    it("-1 >= -2 -- #true", () => {
        expect(negativeOneGreaterThanEqualsNegativeTwo).toEqual(tag("true"))
    })

    it("-1 >= 2 -- #false", () => {
        expect(negativeOneGreaterThanEqualsTwo).toEqual(tag("false"))
    })
})
