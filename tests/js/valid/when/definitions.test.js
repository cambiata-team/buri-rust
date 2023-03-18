import {
    BcolorToString,
    BgetRedComponent,
} from "@tests/js/valid/when/definitions.mjs"
import { tag } from "../helpers"

describe("colorToString", () => {
    it('colorToString(#red) == "red"', () => {
        expect(BcolorToString(tag("red"))).toEqual("red")
    })

    it('colorToString(#green) == "green"', () => {
        expect(BcolorToString(tag("green"))).toEqual("green")
    })

    it('colorToString(#blue) == "blue"', () => {
        expect(BcolorToString(tag("blue"))).toEqual("blue")
    })
})

describe("getRedComponent", () => {
    it("getRedComponent(#rgb(255, 0, 0)) == 255", () => {
        expect(BgetRedComponent(tag("rgb", 255, 0, 0))).toEqual(255)
    })

    it("getRedComponent(#rgb(0, 255, 0)) == 0", () => {
        expect(BgetRedComponent(tag("rgb", 0, 255, 0))).toEqual(0)
    })
})
