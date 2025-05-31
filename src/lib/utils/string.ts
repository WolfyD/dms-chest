// This file will contain all the UI utilities for the app

/**Converts a string to a lowercase string with spaces replaced with hyphens
 * @param string - The string to convert
 * @returns The converted string
 */
export function get_id_from_string(string: string) {
    return string.toLowerCase().replace(/ /g, '-');
}
