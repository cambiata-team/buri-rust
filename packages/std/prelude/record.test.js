import { expect, it } from "bun:test"
import "./index.js"

it("cloning a record should keep the same value", () => {
    const record = { a: 1, b: 2 }
    expect(record.$clone()).toEqual(record)
})

it("cloning a record should return a new record", () => {
    const record = { a: 1, b: 2 }
    expect(record.$clone()).not.toBe(record)
})

it("trait methods added to records should be available after cloning", () => {
    class MyRecord extends Object {
        greet() {
            return "hello"
        }
    }
    const record = new MyRecord({ a: 1, b: 2 })
    expect(record.$clone().greet()).toBe("hello")
})

it("can set keys by their name", () => {
    const record = { a: 1, b: 2 }
    const newRecord = record.$set("a", 3)
    expect(newRecord.a).toBe(3)
})

it("setting a key produces a new object", () => {
    const record = { a: 1, b: 2 }
    const newRecord = record.$set("a", 3)
    expect(newRecord).not.toBe(record)
})
