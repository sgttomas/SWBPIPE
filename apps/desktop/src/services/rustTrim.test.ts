import { expect, it } from "vitest";
import { trimLikeRust } from "./rustTrim";

it("trims Unicode White_Space at both ends, as Rust's str::trim does, and keeps U+FEFF", () => {
  expect(trimLikeRust(" \t\n invented.library \r\n")).toBe("invented.library");
  expect(trimLikeRust("  invented　")).toBe("invented");
  expect(trimLikeRust("﻿invented.library")).toBe("﻿invented.library");
  expect("﻿invented.library".trim()).toBe("invented.library"); // the divergence this helper exists for
  expect(trimLikeRust("in ner")).toBe("in ner");
});
