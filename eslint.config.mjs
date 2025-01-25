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
      "no-unused-vars": ["off"],
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
  },
];
