import { z } from "zod";

export const mediaUrlSchema = z
  .string()
  .trim()
  .url()
  .refine((value) => value.startsWith("http://") || value.startsWith("https://"), {
    message: "Only http(s) URLs are allowed",
  });

export function parseUrlList(raw: string): string[] {
  return raw
    .split(/[\n\r,]+/)
    .map((part) => part.trim())
    .filter(Boolean);
}

export function validateUrlList(raw: string): { urls: string[]; errors: string[] } {
  const parts = parseUrlList(raw);
  const urls: string[] = [];
  const errors: string[] = [];
  for (const part of parts) {
    const result = mediaUrlSchema.safeParse(part);
    if (result.success) {
      urls.push(result.data);
    } else {
      errors.push(part);
    }
  }
  return { urls, errors };
}
