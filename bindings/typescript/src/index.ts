import { Ajv2020, type ErrorObject, type ValidateFunction } from "ajv/dist/2020.js";
import { schema } from "./schema.js";
import type { BetulaTypes } from "./types.js";

export type * from "./types.js";

export type Kind = keyof BetulaTypes;

export class BetulaValidationError extends Error {
  constructor(
    readonly kind: Kind,
    readonly errors: ErrorObject[],
  ) {
    super(`invalid ${kind}: ${ajv.errorsText(errors)}`);
    this.name = "BetulaValidationError";
  }
}

const ajv = new Ajv2020({ strict: true, allowUnionTypes: true, allErrors: true });
ajv.addSchema(schema as object);
const validators = new Map<Kind, ValidateFunction>();

function validatorFor(kind: Kind): ValidateFunction {
  let validate = validators.get(kind);
  if (!validate) {
    if (!Object.hasOwn(schema.$defs, kind)) throw new Error(`unknown betula kind: ${String(kind)}`);
    validate = ajv.compile({ $ref: `${schema.$id}#/$defs/${kind}` });
    validators.set(kind, validate);
  }
  return validate;
}

/** Type guard: true if `data` conforms to the `kind` schema. */
export function is<K extends Kind>(kind: K, data: unknown): data is BetulaTypes[K] {
  return validatorFor(kind)(data);
}

/** Validate already-decoded JSON, throwing `BetulaValidationError` if it doesn't conform. */
export function parse<K extends Kind>(kind: K, data: unknown): BetulaTypes[K] {
  const validate = validatorFor(kind);
  if (!validate(data)) throw new BetulaValidationError(kind, validate.errors ?? []);
  return data as BetulaTypes[K];
}

/** Parse JSON text, throwing `BetulaValidationError` if it doesn't conform. */
export function parseJson<K extends Kind>(kind: K, text: string): BetulaTypes[K] {
  return parse(kind, JSON.parse(text));
}
