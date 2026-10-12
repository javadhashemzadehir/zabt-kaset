import { describe, expect, it } from "vitest";
import { parseUrlList, validateUrlList } from "./url";

describe("url helpers", () => {
  it("splits mixed separators", () => {
    expect(
      parseUrlList("https://a.test/x\nhttps://b.test/y, https://c.test/z"),
    ).toHaveLength(3);
  });

  it("rejects non-http schemes", () => {
    const result = validateUrlList("javascript:alert(1)\nhttps://youtube.com/watch?v=1");
    expect(result.urls).toEqual(["https://youtube.com/watch?v=1"]);
    expect(result.errors).toHaveLength(1);
  });
});
