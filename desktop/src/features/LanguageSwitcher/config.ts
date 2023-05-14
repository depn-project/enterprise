import { LanguageLocale } from "./model";

export const DEFAULT_LOCALE: LanguageLocale = "be";
export const LANGUAGE_SWITCHER_CONFIG: Array<{
  value: LanguageLocale;
  label: string;
}> = [
  { value: "ru", label: "Русский" },
  { value: "be", label: "Беларуская" },
  { value: "en", label: "English" },
];
