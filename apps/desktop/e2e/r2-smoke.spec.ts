import { expect, test } from "@playwright/test";
import { inflateSync } from "node:zlib";

test("R2 desktop preview smoke covers solve, results, report, and viewport overlay", async ({ page }) => {
  await page.goto("/");

  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await expect(page.getByRole("heading", { name: "OpenPipeStress Technical Preview" })).toBeVisible();
  await expect(page.getByTestId("viewport-deformation-status")).toContainText("not started; result rows=0");
  await expect(page.getByTestId("local-project-status")).toContainText("network=false");
  await expect(page.getByTestId("local-project-status")).toContainText("telemetry=false");

  const canvas = page.locator(".viewport-canvas canvas");
  await expect(canvas).toBeVisible();
  const before = await canvas.screenshot();
  await page.waitForTimeout(700);
  const after = await canvas.screenshot();
  expect(pngStats(before).uniqueColors).toBeGreaterThan(100);
  expect(diffPngPixels(before, after)).toBeGreaterThan(100);

  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=647");
  await expect(page.getByTestId("viewport-deformation-status")).toContainText("available; nodes=5; max=33.211157 mm");
  await expect(page.getByTestId("viewport-deformation-boundary")).toContainText(
    "scale=normalized_display_offset_not_physical_length"
  );

  const solvedCanvas = await canvas.screenshot();
  expect(pngStats(solvedCanvas).uniqueColors).toBeGreaterThan(100);

  await expect(page.getByTestId("results-panel")).toBeVisible();
  await expect(page.getByTestId("result-filter-summary")).toContainText("647 of 647 results match filter");
  await page.getByTestId("result-filter-input").fill("pipe-P-120");
  await expect(page.getByTestId("result-filter-summary")).toContainText("167 of 647 results match filter");
  await expect(page.getByTestId("result-page-summary")).toContainText(
    "Showing 1 to 50 of 167 matching results; page 1 of 4"
  );
  await page.getByTestId("result-row-result:force:pipe-P-120:axial").click();
  await expect(page.getByTestId("result-detail-panel")).toContainText("pipe:P-120");
  await expect(page.getByTestId("result-detail-panel")).toContainText("recovered_from_local_element_stiffness");

  const report = page.getByLabel("Report packet");
  await expect(report).toContainText("run:preview-linear-static-001");
  const reportHref = await report.getByTestId("report-export-link").getAttribute("href");
  expect(reportHref).toBeTruthy();
  const reportPacket = JSON.parse(decodeURIComponent(reportHref!.split(",", 2)[1]));
  expect(reportPacket.document_kind).toBe("openpipestress.technical_preview.report_packet_export");
  expect(reportPacket.run_audit.analysis_run_ref.ref).toBe("run:preview-linear-static-001");
  expect(reportPacket.private_payload_included).toBe(false);
  expect(reportPacket.protected_content_included).toBe(false);
  expect(reportPacket.release_or_professional_claim).toBe(false);
});

type PngImage = {
  bytesPerPixel: number;
  data: Uint8Array;
  height: number;
  width: number;
};

function pngStats(buffer: Buffer): { height: number; opaquePixels: number; uniqueColors: number; width: number } {
  const image = decodePng(buffer);
  const colors = new Set<string>();
  let opaquePixels = 0;
  for (let offset = 0; offset < image.data.length; offset += image.bytesPerPixel) {
    const alpha = image.bytesPerPixel === 4 ? image.data[offset + 3] : 255;
    if (alpha > 0) opaquePixels += 1;
    if (colors.size < 2_000) {
      colors.add(Array.from(image.data.slice(offset, offset + image.bytesPerPixel)).join(","));
    }
  }
  return { height: image.height, opaquePixels, uniqueColors: colors.size, width: image.width };
}

function diffPngPixels(leftBuffer: Buffer, rightBuffer: Buffer): number {
  const left = decodePng(leftBuffer);
  const right = decodePng(rightBuffer);
  const compared = Math.min(left.data.length, right.data.length);
  const bytesPerPixel = Math.min(left.bytesPerPixel, right.bytesPerPixel);
  let changed = 0;
  for (let offset = 0; offset + bytesPerPixel <= compared; offset += bytesPerPixel) {
    const channelDelta =
      Math.abs(left.data[offset] - right.data[offset]) +
      Math.abs(left.data[offset + 1] - right.data[offset + 1]) +
      Math.abs(left.data[offset + 2] - right.data[offset + 2]);
    if (channelDelta > 6) changed += 1;
  }
  return changed;
}

function decodePng(buffer: Buffer): PngImage {
  const signature = buffer.subarray(0, 8).toString("hex");
  if (signature !== "89504e470d0a1a0a") throw new Error("not a PNG");

  let offset = 8;
  let width = 0;
  let height = 0;
  let colorType = -1;
  const idatChunks: Buffer[] = [];

  while (offset < buffer.length) {
    const length = buffer.readUInt32BE(offset);
    const type = buffer.subarray(offset + 4, offset + 8).toString("ascii");
    const data = buffer.subarray(offset + 8, offset + 8 + length);
    if (type === "IHDR") {
      width = data.readUInt32BE(0);
      height = data.readUInt32BE(4);
      const bitDepth = data.readUInt8(8);
      colorType = data.readUInt8(9);
      if (bitDepth !== 8 || ![2, 6].includes(colorType)) {
        throw new Error(`unsupported PNG format: bitDepth=${bitDepth}, colorType=${colorType}`);
      }
    } else if (type === "IDAT") {
      idatChunks.push(Buffer.from(data));
    } else if (type === "IEND") {
      break;
    }
    offset += length + 12;
  }

  const bytesPerPixel = colorType === 6 ? 4 : 3;
  const stride = width * bytesPerPixel;
  const inflated = inflateSync(Buffer.concat(idatChunks));
  const data = new Uint8Array(height * stride);
  let sourceOffset = 0;

  for (let row = 0; row < height; row += 1) {
    const filter = inflated[sourceOffset];
    sourceOffset += 1;
    const raw = inflated.subarray(sourceOffset, sourceOffset + stride);
    sourceOffset += stride;
    const rowOffset = row * stride;
    const previousRowOffset = rowOffset - stride;
    for (let index = 0; index < stride; index += 1) {
      const left = index >= bytesPerPixel ? data[rowOffset + index - bytesPerPixel] : 0;
      const up = row > 0 ? data[previousRowOffset + index] : 0;
      const upLeft = row > 0 && index >= bytesPerPixel ? data[previousRowOffset + index - bytesPerPixel] : 0;
      data[rowOffset + index] = (raw[index] + filterValue(filter, left, up, upLeft)) & 0xff;
    }
  }

  return { bytesPerPixel, data, height, width };
}

function filterValue(filter: number, left: number, up: number, upLeft: number): number {
  if (filter === 0) return 0;
  if (filter === 1) return left;
  if (filter === 2) return up;
  if (filter === 3) return Math.floor((left + up) / 2);
  if (filter === 4) return paeth(left, up, upLeft);
  throw new Error(`unsupported PNG filter: ${filter}`);
}

function paeth(left: number, up: number, upLeft: number): number {
  const estimate = left + up - upLeft;
  const leftDistance = Math.abs(estimate - left);
  const upDistance = Math.abs(estimate - up);
  const upLeftDistance = Math.abs(estimate - upLeft);
  if (leftDistance <= upDistance && leftDistance <= upLeftDistance) return left;
  if (upDistance <= upLeftDistance) return up;
  return upLeft;
}
