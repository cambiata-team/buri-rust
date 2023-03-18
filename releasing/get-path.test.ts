import { describe, expect, it } from "bun:test"
import { getPathBuilder } from "./get-path"

describe("path output depends on argv", () => {
    it("if argv.length == 0, getPath just returns the original path", () => {
        expect(getPathBuilder([])("foo")).toEqual("foo")
    })

    it("if argv.length == 1, getPath just returns the original path", () => {
        expect(getPathBuilder(["foo"])("bar")).toEqual("bar")
    })

    it("if argv.length == 2, getPath just returns the original path", () => {
        expect(getPathBuilder(["foo", "bar"])("baz")).toEqual("baz")
    })

    it("if argv.length == 3, getPath returns the path joined with the third argument", () => {
        expect(getPathBuilder(["foo", "bar", "baz"])("qux")).toEqual("baz/qux")
    })

    it("if argv.length > 4, getPath returns the path joined with the third argument", () => {
        expect(getPathBuilder(["foo", "bar", "baz", "qux"])("quux")).toEqual(
            "baz/quux"
        )
    })
})

describe("path edge cases", () => {
    it("path with leading slash", () => {
        expect(getPathBuilder(["foo", "bar", "baz"])("/qux")).toEqual("baz/qux")
    })
})
