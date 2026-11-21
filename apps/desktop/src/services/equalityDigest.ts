/** An in-page equality witness: SHA-256 of the UTF-8 form of a text that is fed to it in
 * pieces, so a text is compared with an earlier one without either being held whole.
 *
 * It detects that a value in this page changed since it was received (previewService's
 * received and registered results, the reader's received text). It is never published,
 * never written to a record, never compared with a hash made outside this page (native,
 * wasm or a saved one), and it is not a fallback for `hashService.ts`: the desktop's
 * canonical hashes are the wasm engine's (H1 / F-5a). A digest equals another only when
 * the two texts fed to them are the same code units, whatever pieces they were fed in.
 *
 * Every code unit is encoded: a surrogate pair as its code point (also when the pair is
 * split between pieces), a lone surrogate as its three-byte form, so different texts
 * never feed the same bytes. */

const K = new Int32Array([
  0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
  0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
  0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
  0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
  0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
  0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
  0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
  0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
]);

/** One digest: `put` the text's pieces in order, then `hex` once. */
export class EqualityDigest {
  private readonly state = new Int32Array([0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19]);
  private readonly block = new Uint8Array(64);
  private readonly words = new Int32Array(64);
  private filled = 0;
  /** Whole 64-byte blocks compressed so far. */
  private blocks = 0;
  /** A high surrogate that ended the last piece, waiting for its pair (0: none). */
  private high = 0;
  private done = false;
  /** Code units fed (the length of the whole text). */
  chars = 0;

  put(text: string): void {
    if (this.done) throw new Error("EqualityDigest: put after hex");
    const length = text.length;
    this.chars += length;
    const block = this.block;
    let filled = this.filled;
    let index = 0;
    if (this.high !== 0 && length > 0) {
      const high = this.high;
      this.high = 0;
      const next = text.charCodeAt(0);
      if (next >= 0xdc00 && next <= 0xdfff) { this.filled = filled; this.point(0x10000 + ((high - 0xd800) << 10) + (next - 0xdc00)); filled = this.filled; index = 1; }
      else { this.filled = filled; this.point(high); filled = this.filled; }
    }
    while (index < length) {
      const unit = text.charCodeAt(index);
      if (unit < 0x80) {
        block[filled++] = unit;
        if (filled === 64) { this.compress(); filled = 0; }
        index += 1;
        continue;
      }
      this.filled = filled;
      if (unit >= 0xd800 && unit <= 0xdbff) {
        if (index + 1 === length) { this.high = unit; index += 1; filled = this.filled; continue; }
        const next = text.charCodeAt(index + 1);
        if (next >= 0xdc00 && next <= 0xdfff) { this.point(0x10000 + ((unit - 0xd800) << 10) + (next - 0xdc00)); index += 2; }
        else { this.point(unit); index += 1; }
      } else {
        this.point(unit);
        index += 1;
      }
      filled = this.filled;
    }
    this.filled = filled;
  }

  /** The lowercase hex SHA-256 of everything fed. The digest takes no more pieces. */
  hex(): string {
    if (this.done) throw new Error("EqualityDigest: hex twice");
    if (this.high !== 0) { const high = this.high; this.high = 0; this.point(high); }
    this.done = true;
    const bytes = this.blocks * 64 + this.filled;
    const block = this.block;
    block[this.filled++] = 0x80;
    if (this.filled > 56) { block.fill(0, this.filled); this.compress(); this.filled = 0; }
    block.fill(0, this.filled, 56);
    // The length in bits, big-endian, in the last eight bytes.
    const high = Math.floor(bytes / 0x20000000), low = (bytes * 8) >>> 0;
    block[56] = high >>> 24; block[57] = high >>> 16; block[58] = high >>> 8; block[59] = high;
    block[60] = low >>> 24; block[61] = low >>> 16; block[62] = low >>> 8; block[63] = low;
    this.compress();
    let out = "";
    for (const word of this.state) out += (word >>> 0).toString(16).padStart(8, "0");
    return out;
  }

  /** One code point (or lone surrogate) as UTF-8. */
  private point(code: number): void {
    if (code < 0x800) { this.byte(0xc0 | (code >> 6)); this.byte(0x80 | (code & 63)); }
    else if (code < 0x10000) { this.byte(0xe0 | (code >> 12)); this.byte(0x80 | ((code >> 6) & 63)); this.byte(0x80 | (code & 63)); }
    else { this.byte(0xf0 | (code >> 18)); this.byte(0x80 | ((code >> 12) & 63)); this.byte(0x80 | ((code >> 6) & 63)); this.byte(0x80 | (code & 63)); }
  }

  private byte(value: number): void {
    this.block[this.filled++] = value;
    if (this.filled === 64) { this.compress(); this.filled = 0; }
  }

  private compress(): void {
    const w = this.words, b = this.block, h = this.state;
    for (let i = 0; i < 16; i += 1) w[i] = (b[4 * i] << 24) | (b[4 * i + 1] << 16) | (b[4 * i + 2] << 8) | b[4 * i + 3];
    for (let i = 16; i < 64; i += 1) {
      const x = w[i - 15], y = w[i - 2];
      const s0 = ((x >>> 7) | (x << 25)) ^ ((x >>> 18) | (x << 14)) ^ (x >>> 3);
      const s1 = ((y >>> 17) | (y << 15)) ^ ((y >>> 19) | (y << 13)) ^ (y >>> 10);
      w[i] = (w[i - 16] + s0 + w[i - 7] + s1) | 0;
    }
    let a = h[0], bb = h[1], c = h[2], d = h[3], e = h[4], f = h[5], g = h[6], hh = h[7];
    for (let i = 0; i < 64; i += 1) {
      const S1 = ((e >>> 6) | (e << 26)) ^ ((e >>> 11) | (e << 21)) ^ ((e >>> 25) | (e << 7));
      const t1 = (hh + S1 + ((e & f) ^ (~e & g)) + K[i] + w[i]) | 0;
      const S0 = ((a >>> 2) | (a << 30)) ^ ((a >>> 13) | (a << 19)) ^ ((a >>> 22) | (a << 10));
      const t2 = (S0 + ((a & bb) ^ (a & c) ^ (bb & c))) | 0;
      hh = g; g = f; f = e; e = (d + t1) | 0; d = c; c = bb; bb = a; a = (t1 + t2) | 0;
    }
    h[0] = (h[0] + a) | 0; h[1] = (h[1] + bb) | 0; h[2] = (h[2] + c) | 0; h[3] = (h[3] + d) | 0;
    h[4] = (h[4] + e) | 0; h[5] = (h[5] + f) | 0; h[6] = (h[6] + g) | 0; h[7] = (h[7] + hh) | 0;
    this.blocks += 1;
  }
}

/** The equality digest of one whole text. */
export function equalityDigestOf(text: string): string {
  const digest = new EqualityDigest();
  digest.put(text);
  return digest.hex();
}
