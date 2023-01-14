import { expect, it } from "bun:test"
import "./index.js"

it("cloning a number should keep the same value", () => {
    const num = 42
    expect(num.$clone().valueOf()).toBe(num)
})

it("numbers should be able to be added", () => {
    const num = 42
    expect(num.add(1).valueOf()).toBe(43)
})

it("numbers should be able to be subtracted", () => {
    const num = 42
    expect(num.subtract(1).valueOf()).toBe(41)
})

it("numbers should be able to be multiplied", () => {
    const num = 42
    expect(num.multiply(2).valueOf()).toBe(84)
    expect(num.multiply(3).valueOf()).toBe(126)
})

it("numbers should be able to be divided", () => {
    const num = 42
    expect(num.divide(2).valueOf()).toBe(21)
    expect(num.divide(3).valueOf()).toBe(14)
})

it("numbers should be able to be moduloed", () => {
    const num = 42
    expect(num.modulo(2).valueOf()).toBe(0)
    expect(num.modulo(3).valueOf()).toBe(0)
    expect(num.modulo(4).valueOf()).toBe(2)
})

it("numbers should be able to be powered", () => {
    const num = 42
    expect(num.power(2).valueOf()).toBe(1764)
    expect(num.power(3).valueOf()).toBe(74088)
})

it("numbers should be able to be compared for equality", () => {
    const num = 42
    expect(num.equals(42)).toBe(true)
    expect(num.equals(43)).toBe(false)
})

it("numbers should be able to be compared for inequality", () => {
    const num = 42
    expect(num.notEquals(42)).toBe(false)
    expect(num.notEquals(43)).toBe(true)
})

it("numbers should be able to be compared for less than", () => {
    const num = 42
    expect(num.lessThan(42)).toBe(false)
    expect(num.lessThan(43)).toBe(true)
})

it("numbers should be able to be compared for less than or equal to", () => {
    const num = 42
    expect(num.lessThanOrEquals(42)).toBe(true)
    expect(num.lessThanOrEquals(43)).toBe(true)
})

it("numbers should be able to be compared for greater than", () => {
    const num = 42
    expect(num.greaterThan(42)).toBe(false)
    expect(num.greaterThan(41)).toBe(true)
})

it("numbers should be able to be compared for greater than or equal to", () => {
    const num = 42
    expect(num.greaterThanOrEquals(42)).toBe(true)
    expect(num.greaterThanOrEquals(41)).toBe(true)
})

it("numbers should retain their original properties after cloning", () => {
    const num = 42
    expect(num.$clone().toFixed(2)).toBe("42.00")
})

it("trait methods added to an integer still have it's methods after cloning", () => {
    class MyInt extends Number {
        greet() {
            return "hello"
        }
    }
    const int = new MyInt(42)
    expect(int.$clone().greet()).toBe("hello")
})
