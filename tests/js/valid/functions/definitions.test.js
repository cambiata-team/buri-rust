import {
    add,
    getAge,
    getName,
    ifTrueFiveElseThree,
    multilineAdd,
    person,
} from "@tests/js/valid/functions/definitions.mjs"
import { describe, expect, it } from "bun:test"
import { tag } from "../helpers"

describe("can write a function that adds two numbers", () => {
    it("2 + 3 = 5", () => {
        expect(add(2, 3).valueOf()).toBe(5)
    })
    it("132 + 243 = 375", () => {
        expect(add(132, 243).valueOf()).toBe(375)
    })
})

describe("can write a function defined on multiple lines that adds two numbers", () => {
    it("2 + 3 = 5", () => {
        expect(multilineAdd(2, 3).valueOf()).toBe(5)
    })
    it("132 + 243 = 375", () => {
        expect(multilineAdd(132, 243).valueOf()).toBe(375)
    })
})

describe("can write functions with an if statement", () => {
    it("true returns 5", () => {
        expect(ifTrueFiveElseThree(tag("true"))).toBe(5)
    })
    it("false returns 3", () => {
        expect(ifTrueFiveElseThree(tag("false"))).toBe(3)
    })
})

describe("can write functions that get record fields", () => {
    it("can get the name", () => {
        expect(getName(person)).toBe("John")
    })
    it("can get the age", () => {
        expect(getAge(person)).toBe(30)
    })
})
