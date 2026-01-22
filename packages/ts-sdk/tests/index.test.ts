import { fn } from "@epimnesis/core";
import { expect, test } from "vitest";

test("fn", () => {
  expect(fn()).toBe("Hello, tsdown!");
});
