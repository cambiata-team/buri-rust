import {
    listOfNumbers,
    listOfStrings,
    expressionsInList,
    nestedLists,
    recordsInList,
    multilineList,
} from "@tests/js/valid/list/definitions.mjs";

import { expect, it } from "bun:test";

it("a list can contain numbers", () => {
    expect(listOfNumbers).toEqual([0, 1, 2, 3]);
});

it("a list can contain strings", () => {
    expect(listOfStrings).toEqual(["a", "b", "c"]);
});

it("a list can be nested", () => {
    expect(nestedLists).toEqual([
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
    ]);
});

it("a list can contain expressions", () => {
    expect(expressionsInList.length).toEqual(2);
    expect(expressionsInList[0].valueOf()).toEqual(1 + 1);
    expect(expressionsInList[1].valueOf()).toEqual(3 ** 6 + 2);
});

it("a list can contain records", () => {
    expect(recordsInList).toEqual([
        { id: 12345, job: "janitor" },
        { id: 54321, job: "accountant" },
    ]);
});

it("a list can be multiple lines long", () => {
    expect(multilineList).toEqual([1, 2, 3, 4]);
});
