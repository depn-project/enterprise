import { i18n } from "@/shared/lib/i18n";
import { LanguageLocale } from "./model";

export const changeLocale = (locale: LanguageLocale): void => {
  i18n.global.locale.value = locale;
};
