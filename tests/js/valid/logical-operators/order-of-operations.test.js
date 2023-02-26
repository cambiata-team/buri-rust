import {
    BfalseAndFalseOrFalse,
    BfalseAndFalseOrTrue,
    BfalseAndNotFalse,
    BfalseAndNotTrue,
    BfalseAndTrueOrFalse,
    BfalseAndTrueOrTrue,
    BfalseOrFalseAndFalse,
    BfalseOrFalseAndTrue,
    BfalseOrNotFalse,
    BfalseOrNotTrue,
    BfalseOrTrueAndFalse,
    BfalseOrTrueAndTrue,
    BnotFalseAndFalse,
    BnotFalseAndTrue,
    BnotFalseOrFalse,
    BnotFalseOrTrue,
    BnotTrueAndFalse,
    BnotTrueAndTrue,
    BnotTrueOrFalse,
    BnotTrueOrTrue,
    BtrueAndFalseOrFalse,
    BtrueAndFalseOrTrue,
    BtrueAndNotFalse,
    BtrueAndNotTrue,
    BtrueAndTrueOrFalse,
    BtrueAndTrueOrTrue,
    BtrueOrFalseAndFalse,
    BtrueOrFalseAndTrue,
    BtrueOrNotFalse,
    BtrueOrNotTrue,
    BtrueOrTrueAndFalse,
    BtrueOrTrueAndTrue,
} from "@tests/js/valid/logical-operators/order-of-operations.mjs"
import { describe, expect, it } from "bun:test"
import { tag } from "../helpers"

describe("and or", () => {
    it("#true and #true or #true", () => {
        expect(BtrueAndTrueOrTrue).toEqual(tag("true"))
    })

    it("#true and #true or #false", () => {
        expect(BtrueAndTrueOrFalse).toEqual(tag("true"))
    })

    it("#true and #false or #true", () => {
        expect(BtrueAndFalseOrTrue).toEqual(tag("true"))
    })

    it("#false and #true or #true", () => {
        expect(BfalseAndTrueOrTrue).toEqual(tag("true"))
    })

    it("#false and #false or #true", () => {
        expect(BfalseAndFalseOrTrue).toEqual(tag("true"))
    })

    it("#true and #false or #false", () => {
        expect(BtrueAndFalseOrFalse).toEqual(tag("false"))
    })

    it("#false and #true or #false", () => {
        expect(BfalseAndTrueOrFalse).toEqual(tag("false"))
    })

    it("#false and #false or #false", () => {
        expect(BfalseAndFalseOrFalse).toEqual(tag("false"))
    })
})

describe("or and", () => {
    it("#true or #true and #true", () => {
        expect(BtrueOrTrueAndTrue).toEqual(tag("true"))
    })

    it("#true or #true and #false", () => {
        expect(BtrueOrTrueAndFalse).toEqual(tag("true"))
    })

    it("#true or #false and #true", () => {
        expect(BtrueOrFalseAndTrue).toEqual(tag("true"))
    })

    it("#false or #true and #true", () => {
        expect(BfalseOrTrueAndTrue).toEqual(tag("true"))
    })

    it("#false or #false and #true", () => {
        expect(BfalseOrFalseAndTrue).toEqual(tag("false"))
    })

    it("#true or #false and #false", () => {
        expect(BtrueOrFalseAndFalse).toEqual(tag("true"))
    })

    it("#false or #true and #false", () => {
        expect(BfalseOrTrueAndFalse).toEqual(tag("false"))
    })

    it("#false or #false and #false", () => {
        expect(BfalseOrFalseAndFalse).toEqual(tag("false"))
    })
})

describe("not and", () => {
    it("not #true and #true", () => {
        expect(BnotTrueAndTrue).toEqual(tag("false"))
    })

    it("not #false and #true", () => {
        expect(BnotFalseAndTrue).toEqual(tag("true"))
    })

    it("not #true and #false", () => {
        expect(BnotTrueAndFalse).toEqual(tag("false"))
    })

    it("not #false and #false", () => {
        expect(BnotFalseAndFalse).toEqual(tag("false"))
    })
})

describe("not or", () => {
    it("not #true or #true", () => {
        expect(BnotTrueOrTrue).toEqual(tag("true"))
    })

    it("not #false or #true", () => {
        expect(BnotFalseOrTrue).toEqual(tag("true"))
    })

    it("not #true or #false", () => {
        expect(BnotTrueOrFalse).toEqual(tag("false"))
    })

    it("not #false or #false", () => {
        expect(BnotFalseOrFalse).toEqual(tag("true"))
    })
})

describe("and not", () => {
    it("#true and not #true", () => {
        expect(BtrueAndNotTrue).toEqual(tag("false"))
    })

    it("#false and not #true", () => {
        expect(BfalseAndNotTrue).toEqual(tag("false"))
    })

    it("#true and not #false", () => {
        expect(BtrueAndNotFalse).toEqual(tag("true"))
    })

    it("#false and not #false", () => {
        expect(BfalseAndNotFalse).toEqual(tag("false"))
    })
})

describe("or not", () => {
    it("#true or not #true", () => {
        expect(BtrueOrNotTrue).toEqual(tag("true"))
    })

    it("#false or not #true", () => {
        expect(BfalseOrNotTrue).toEqual(tag("false"))
    })

    it("#true or not #false", () => {
        expect(BtrueOrNotFalse).toEqual(tag("true"))
    })

    it("#false or not #false", () => {
        expect(BfalseOrNotFalse).toEqual(tag("true"))
    })
})
