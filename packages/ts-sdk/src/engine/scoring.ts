/**
 * Scoring algorithms - ported from Rust engine
 */

import type { Candidate, ProceduralPayload } from "./types";

const HALF_LIFE_MS = 24 * 60 * 60 * 1000;

export function cosineSimilarity(
  a: Float32Array | number[],
  b: Float32Array | number[],
): number {
  if (a.length === 0 || b.length === 0 || a.length !== b.length) {
    return 0.0;
  }

  let dot = 0.0;
  let normA = 0.0;
  let normB = 0.0;

  for (let i = 0; i < a.length; i++) {
    dot += a[i] * b[i];
    normA += a[i] * a[i];
    normB += b[i] * b[i];
  }

  if (normA === 0.0 || normB === 0.0) {
    return 0.0;
  }

  return dot / (Math.sqrt(normA) * Math.sqrt(normB));
}

export function computeRecency(eventTime: number, now: number): number {
  if (eventTime >= now) {
    return 1.0;
  }

  const ageMs = now - eventTime;
  return Math.pow(0.5, ageMs / HALF_LIFE_MS);
}

export function matchesTrigger(procMem: ProceduralPayload): string | undefined {
  return procMem.triggers[0];
}

export function getPayload<K extends Candidate["kind"]>(
  candidate: Candidate,
  kind: K,
): Extract<Candidate, { kind: K }> | undefined {
  if (candidate.kind === kind) {
    return candidate as Extract<Candidate, { kind: K }>;
  }
  return undefined;
}
