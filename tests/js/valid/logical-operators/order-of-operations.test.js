import {
    falseAndFalseOrFalse,
    falseAndFalseOrTrue,
    falseAndNotFalse,
    falseAndNotTrue,
    falseAndTrueOrFalse,
    falseAndTrueOrTrue,
    falseOrFalseAndFalse,
    falseOrFalseAndTrue,
    falseOrNotFalse,
    falseOrNotTrue,
    falseOrTrueAndFalse,
    falseOrTrueAndTrue,
    notFalseAndFalse,
    notFalseAndTrue,
    notFalseOrFalse,
    notFalseOrTrue,
    notTrueAndFalse,
    notTrueAndTrue,
    notTrueOrFalse,
    notTrueOrTrue,
    trueAndFalseOrFalse,
    trueAndFalseOrTrue,
    trueAndNotFalse,
    trueAndNotTrue,
    trueAndTrueOrFalse,
    trueAndTrueOrTrue,
    trueOrFalseAndFalse,
    trueOrFalseAndTrue,
    trueOrNotFalse,
    trueOrNotTrue,
    trueOrTrueAndFalse,
    trueOrTrueAndTrue,
} from "@tests/js/valid/logical-operators/order-of-operations.mjs"
import { describe, expect, it } from "bun:test"
import { tag } from "../helpers"

describe("and or", () => {
    it("#true and #true or #true", () => {
        expect(trueAndTrueOrTrue).toEqual(tag("true"))
    })

    it("#true and #true or #false", () => {
        expect(trueAndTrueOrFalse).toEqual(tag("true"))
    })

    it("#true and #false or #true", () => {
        expect(trueAndFalseOrTrue).toEqual(tag("true"))
    })

    it("#false and #true or #true", () => {
        expect(falseAndTrueOrTrue).toEqual(tag("true"))
    })

    it("#false and #false or #true", () => {
        expect(falseAndFalseOrTrue).toEqual(tag("true"))
    })

    it("#true and #false or #false", () => {
        expect(trueAndFalseOrFalse).toEqual(tag("false"))
    })

    it("#false and #true or #false", () => {
        expect(falseAndTrueOrFalse).toEqual(tag("false"))
    })

    it("#false and #false or #false", () => {
        expect(falseAndFalseOrFalse).toEqual(tag("false"))
    })
})

describe("or and", () => {
    it("#true or #true and #true", () => {
        expect(trueOrTrueAndTrue).toEqual(tag("true"))
    })

    it("#true or #true and #false", () => {
        expect(trueOrTrueAndFalse).toEqual(tag("true"))
    })

    it("#true or #false and #true", () => {
        expect(trueOrFalseAndTrue).toEqual(tag("true"))
    })

    it("#false or #true and #true", () => {
        expect(falseOrTrueAndTrue).toEqual(tag("true"))
    })

    it("#false or #false and #true", () => {
        expect(falseOrFalseAndTrue).toEqual(tag("false"))
    })

    it("#true or #false and #false", () => {
        expect(trueOrFalseAndFalse).toEqual(tag("true"))
    })

    it("#false or #true and #false", () => {
        expect(falseOrTrueAndFalse).toEqual(tag("false"))
    })

    it("#false or #false and #false", () => {
        expect(falseOrFalseAndFalse).toEqual(tag("false"))
    })
})

describe("not and", () => {
    it("not #true and #true", () => {
        expect(notTrueAndTrue).toEqual(tag("false"))
    })

    it("not #false and #true", () => {
        expect(notFalseAndTrue).toEqual(tag("true"))
    })

    it("not #true and #false", () => {
        expect(notTrueAndFalse).toEqual(tag("false"))
    })

    it("not #false and #false", () => {
        expect(notFalseAndFalse).toEqual(tag("false"))
    })
})

describe("not or", () => {
    it("not #true or #true", () => {
        expect(notTrueOrTrue).toEqual(tag("true"))
    })

    it("not #false or #true", () => {
        expect(notFalseOrTrue).toEqual(tag("true"))
    })

    it("not #true or #false", () => {
        expect(notTrueOrFalse).toEqual(tag("false"))
    })

    it("not #false or #false", () => {
        expect(notFalseOrFalse).toEqual(tag("true"))
    })
})

describe("and not", () => {
    it("#true and not #true", () => {
        expect(trueAndNotTrue).toEqual(tag("false"))
    })

    it("#false and not #true", () => {
        expect(falseAndNotTrue).toEqual(tag("false"))
    })

    it("#true and not #false", () => {
        expect(trueAndNotFalse).toEqual(tag("true"))
    })

    it("#false and not #false", () => {
        expect(falseAndNotFalse).toEqual(tag("false"))
    })
})

describe("or not", () => {
    it("#true or not #true", () => {
        expect(trueOrNotTrue).toEqual(tag("true"))
    })

    it("#false or not #true", () => {
        expect(falseOrNotTrue).toEqual(tag("false"))
    })

    it("#true or not #false", () => {
        expect(trueOrNotFalse).toEqual(tag("true"))
    })

    it("#false or not #false", () => {
        expect(falseOrNotFalse).toEqual(tag("true"))
    })
})
