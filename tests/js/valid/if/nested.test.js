import {
    BnestedIfTrueTrue,
    BnestedIfTrueFalse,
    BnestedIfFalseTrue,
    BnestedIfFalseFalse,
    BcolorRedString,
    BcolorGreenString,
    BcolorBlueString,
    BcolorYellowString,
} from "@tests/js/valid/if/nested.mjs"
import { describe, expect, it } from "bun:test"
import { tag } from "../helpers"

describe("basic nested if statements", () => {

    it("BnestedIfTrueTrue === 1" , () => {
        expect(BnestedIfTrueTrue).toBe(1)
    })

    it("BnestedIfTrueFalse === 2" , () => {
        expect(BnestedIfTrueFalse).toBe(2)
    })

    it("BnestedIfFalseTrue === 3" , () => {
        expect(BnestedIfFalseTrue).toBe(3)
    })

    it("BnestedIfFalseFalse === 4" , () => {
        expect(BnestedIfFalseFalse).toBe(4)
    })

})

describe("nested if statements with function calls", () => {

    it("BcolorRedString === \"red\"" , () => {
        expect(BcolorRedString).toBe("red")
    })

    it("BcolorGreenString === \"green\"" , () => {
        expect(BcolorGreenString).toBe("green")
    })

    it("BcolorBlueString === \"blue\"" , () => {
        expect(BcolorBlueString).toBe("blue")
    })

    it("BcolorYellowString === \"unknown\"" , () => {
        expect(BcolorYellowString).toBe("unknown")
    })

})
