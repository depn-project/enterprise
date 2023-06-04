export const locationI18n = (country: string, city: string, _: any) => {
  return `${_(`location.${country.toLowerCase()}.name`)}, ${_(
    `location.${country.toLowerCase()}.city.${city.toLowerCase()}`
  )}`;
};
