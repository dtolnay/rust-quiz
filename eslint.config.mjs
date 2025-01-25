import globals from "globals";
import pluginJs from "@eslint/js";

/** @type {import('eslint').Linter.Config[]} */
export default [
  {
    files: ["**/*.js"],
    languageOptions: {
      sourceType: "script",
    },
  },
  pluginJs.configs.recommended,
  {
    files: ["docs/questions.js"],
    rules: {
      "no-unused-vars": ["error", { varsIgnorePattern: "^questions$" }],
    },
  },
  {
    files: ["docs/quiz.js"],
    languageOptions: {
      globals: {
        hljs: "readonly",
        questions: "readonly",
        ...globals.browser,
      },
    },
    rules: {
      "no-empty": ["error", { allowEmptyCatch: true }],
      "no-unused-vars": [
        "error",
        { caughtErrors: "none", varsIgnorePattern: "^reset$" },
      ],
    },
  },
];
