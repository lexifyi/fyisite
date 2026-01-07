import eslint from "@eslint/js";
import { defineConfig, globalIgnores } from "eslint/config";
import tseslint from "typescript-eslint";
import { importX } from "eslint-plugin-import-x";

export default defineConfig([
  globalIgnores(["dist/", "eslint.config.mjs"]),
  eslint.configs.recommended,
  importX.flatConfigs.recommended,
  importX.flatConfigs.typescript,
  tseslint.configs.strictTypeChecked,
  tseslint.configs.stylisticTypeChecked,
  {
    languageOptions: {
      parserOptions: {
        projectService: { allowDefaultProject: ["*.js", "*.mjs"] },
      },
    },
    rules: {
      eqeqeq: ["error", "smart"],
      "no-duplicate-imports": ["warn", { allowSeparateTypeImports: true }],
      "no-empty": "warn",
      "no-template-curly-in-string": "warn",
      "no-unused-private-class-members": "warn",
      "no-useless-assignment": "warn",
      "import-x/consistent-type-specifier-style": ["error", "prefer-top-level"],
      "@typescript-eslint/explicit-module-boundary-types": "warn",
      "@typescript-eslint/no-empty-function": [
        "warn",
        { allow: ["private-constructors", "protected-constructors"] },
      ],
      "@typescript-eslint/no-extraneous-class": "warn",
      "@typescript-eslint/no-invalid-void-type": "off",
      "@typescript-eslint/no-non-null-assertion": "off",
      "@typescript-eslint/no-unnecessary-condition": [
        "warn",
        { allowConstantLoopConditions: "only-allowed-literals" },
      ],
      "@typescript-eslint/no-unused-vars": [
        "warn",
        {
          args: "all",
          argsIgnorePattern: "^_",
          caughtErrors: "all",
          caughtErrorsIgnorePattern: "^_",
          destructuredArrayIgnorePattern: "^_",
          varsIgnorePattern: "^_",
          ignoreRestSiblings: true,
        },
      ],
      "@typescript-eslint/prefer-promise-reject-errors": "off",
      "@typescript-eslint/prefer-readonly": "warn",
      "@typescript-eslint/require-await": "warn",
      "@typescript-eslint/restrict-template-expressions": [
        "error",
        { allowNumber: true },
      ],
      "@typescript-eslint/strict-boolean-expressions": [
        "error",
        { allowNumber: false },
      ],
    },
  },
]);
