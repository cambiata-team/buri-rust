import {
    Badd,
    BgetAge,
    BgetName,
    BifTrueFiveElseThree,
    BmultilineAdd,
    Bperson,
} from "@tests/js/valid/functions/definitions.mjs"
import { describe, expect, it } from "bun:test"
import { tag } from "../helpers"

describe("can write a function that adds two numbers", () => {
    it("2 + 3 = 5", () => {
        expect(Badd(2, 3).valueOf()).toBe(5)
    })
    it("132 + 243 = 375", () => {
        expect(Badd(132, 243).valueOf()).toBe(375)
    })
})

describe("can write a function defined on multiple lines that adds two numbers", () => {
    it("2 + 3 = 5", () => {
        expect(BmultilineAdd(2, 3).valueOf()).toBe(5)
    })
    it("132 + 243 = 375", () => {
        expect(BmultilineAdd(132, 243).valueOf()).toBe(375)
    })
})

describe("can write functions with an if statement", () => {
    it("true returns 5", () => {
        expect(BifTrueFiveElseThree(tag("true"))).toBe(5)
    })
    it("false returns 3", () => {
        expect(BifTrueFiveElseThree(tag("false"))).toBe(3)
    })
})

describe("can write functions that get record fields", () => {
    it("can get the name", () => {
        expect(BgetName(Bperson)).toBe("John")
    })
    it("can get the age", () => {
        expect(BgetAge(Bperson)).toBe(30)
    })
})
