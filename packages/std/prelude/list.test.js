import { describe, expect, it } from "bun:test"
import "./index.js"

it("cloning an list should keep the same value", () => {
    const list = [1, 2, 3]
    expect(list.$clone()).toEqual(list)
})

it("cloning an list should return a new list", () => {
    const list = [1, 2, 3]
    expect(list.$clone()).not.toBe(list)
})

it("trait methods added to lists should be available after cloning", () => {
    class MyList extends Array {
        greet() {
            return "hello"
        }
    }
    const list = new MyList(1, 2, 3)
    expect(list.$clone().greet()).toBe("hello")
})

describe("equality with tags", () => {
    it("the names should be equal", () => {
        expect(["foo"].equals(["foo"])).toBe(true)
    })

    it("the names should not be equal", () => {
        expect(["foo"].equals(["bar"])).toBe(false)
    })
})

describe("inequality with tags", () => {
    it("the names should not be equal", () => {
        expect(["foo"].notEquals(["foo"])).toBe(false)
    })

    it("the names should be equal", () => {
        expect(["foo"].notEquals(["bar"])).toBe(true)
    })
})
