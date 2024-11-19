// place files you want to import through the `$lib` alias in this folder.

import { isHasError } from "./api";
import type { HasError } from "./types";

export function ifErrorToNull<T>(input: T | HasError): T | null {
    if (isHasError(input)) {
        return null
    } else {
        return input
    }
}