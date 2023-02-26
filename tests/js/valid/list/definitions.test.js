import {
    BlistOfNumbers,
    BlistOfStrings,
    BexpressionsInList,
    BnestedLists,
    BrecordsInList,
    BmultilineList,
} from "@tests/js/valid/list/definitions.mjs";

import { expect, it } from "bun:test";

it("a list can contain numbers", () => {
    expect(BlistOfNumbers).toEqual([0, 1, 2, 3]);
});

it("a list can contain strings", () => {
    expect(BlistOfStrings).toEqual(["a", "b", "c"]);
});

it("a list can be nested", () => {
    expect(BnestedLists).toEqual([
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
    ]);
});

it("a list can contain expressions", () => {
    expect(BexpressionsInList.length).toEqual(2);
    expect(BexpressionsInList[0].valueOf()).toEqual(1 + 1);
    expect(BexpressionsInList[1].valueOf()).toEqual(3 ** 6 + 2);
});

it("a list can contain records", () => {
    expect(BrecordsInList).toEqual([
        { id: 12345, job: "janitor" },
        { id: 54321, job: "accountant" },
    ]);
});

it("a list can be multiple lines long", () => {
    expect(BmultilineList).toEqual([1, 2, 3, 4]);
});
