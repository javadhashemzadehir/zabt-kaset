import { getRequestConfig } from "next-intl/server";

export const locales = ["en", "fa"] as const;
export type AppLocale = (typeof locales)[number];
export const defaultLocale: AppLocale = "en";

export default getRequestConfig(async () => {
  const locale = defaultLocale;
  return {
    locale,
    messages: (await import(`../locales/${locale}/common.json`)).default,
  };
});
