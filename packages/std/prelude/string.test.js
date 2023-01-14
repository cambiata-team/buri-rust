import { expect, it } from "bun:test"
import "./index.js"

it("cloning a string should keep the same value", () => {
    const str = "hello"
    expect(str.$clone().valueOf()).toBe("hello")
})

it("strings should be able to be concatenated", () => {
    const str = "hello"
    expect(str.concat(" world").valueOf()).toBe("hello world")
})

it("strings should retain their original properties after cloning", () => {
    const str = "hello"
    expect(str.$clone().length).toBe(5)
})

it("trait methods added to strings should be available after cloning", () => {
    class MyString extends String {
        greet() {
            return "hello"
        }
    }
    const str = new MyString("hello")
    expect(str.$clone().greet()).toBe("hello")
})
